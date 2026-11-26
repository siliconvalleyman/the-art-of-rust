//! The byte machine from Chapter 1.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Instruction {
    Read,
    Load(usize),
    Store(usize),
    Add(usize),
    JumpIfZero(usize),
    Write,
    Halt,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Stop {
    Halted,
    OutOfBudget,
    MissingInput,
    InvalidAddress(usize),
    MissingInstruction(usize),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Run {
    pub output: Vec<u8>,
    pub attempted_steps: usize,
    pub consumed_bytes: usize,
    pub stop: Stop,
}

#[derive(Default)]
struct Machine {
    accumulator: u8,
    memory: [u8; 4],
    pc: usize,
    consumed_bytes: usize,
    output: Vec<u8>,
}

impl Machine {
    fn cell(&self, address: usize) -> Result<u8, Stop> {
        self.memory
            .get(address)
            .copied()
            .ok_or(Stop::InvalidAddress(address))
    }

    fn step(&mut self, program: &[Instruction], input: &[u8]) -> Result<(), Stop> {
        let instruction = program
            .get(self.pc)
            .ok_or(Stop::MissingInstruction(self.pc))?;
        match *instruction {
            Instruction::Read => {
                self.accumulator = *input.get(self.consumed_bytes).ok_or(Stop::MissingInput)?;
                self.consumed_bytes += 1;
            }
            Instruction::Load(address) => self.accumulator = self.cell(address)?,
            Instruction::Store(address) => {
                let cell = self
                    .memory
                    .get_mut(address)
                    .ok_or(Stop::InvalidAddress(address))?;
                *cell = self.accumulator;
            }
            Instruction::Add(address) => {
                self.accumulator = self.accumulator.wrapping_add(self.cell(address)?);
            }
            Instruction::JumpIfZero(target) if self.accumulator == 0 => {
                self.pc = target;
                return Ok(());
            }
            Instruction::JumpIfZero(_) => {}
            Instruction::Write => self.output.push(self.accumulator),
            Instruction::Halt => return Err(Stop::Halted),
        }
        self.pc += 1;
        Ok(())
    }

    fn finish(self, attempted_steps: usize, stop: Stop) -> Run {
        Run {
            output: self.output,
            attempted_steps,
            consumed_bytes: self.consumed_bytes,
            stop,
        }
    }
}

/// Execute from the chapter's zero-initialized state with a step limit.
///
/// Each attempted fetch counts toward the limit. Previously written bytes
/// survive a stopped execution.
///
/// ```
/// use foundations::machine::{run, Instruction::*, Stop};
///
/// let result = run(&[Read, Store(0), Read, Add(0), Write, Halt], &[23, 18], 6);
/// assert_eq!(result.output, [41]);
/// assert_eq!(result.stop, Stop::Halted);
/// ```
pub fn run(program: &[Instruction], input: &[u8], budget: usize) -> Run {
    let mut machine = Machine::default();
    for attempt in 1..=budget {
        if let Err(stop) = machine.step(program, input) {
            return machine.finish(attempt, stop);
        }
    }
    machine.finish(budget, Stop::OutOfBudget)
}
