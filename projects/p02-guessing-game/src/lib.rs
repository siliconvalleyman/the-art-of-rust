use std::io::{self, BufRead, Read, Write};

pub const MAX_ROUNDS: u8 = 7;
pub const MAX_LINE_BYTES: usize = 32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Won,
    Exhausted,
    EndOfInput,
    LineTooLong,
}

pub fn secret_for_seed(seed: u64) -> u8 {
    let mut state = if seed == 0 { 1 } else { seed };
    state ^= state << 13;
    state ^= state >> 7;
    state ^= state << 17;
    (state % 100 + 1) as u8
}

fn parse_guess(line: &[u8]) -> Option<u8> {
    let guess = std::str::from_utf8(line).ok()?.trim().parse::<u8>().ok()?;
    (1..=100).contains(&guess).then_some(guess)
}

pub fn play(seed: u64, mut input: impl BufRead, mut output: impl Write) -> io::Result<Outcome> {
    use std::cmp::Ordering;

    let secret = secret_for_seed(seed);
    let mut line = Vec::with_capacity(MAX_LINE_BYTES + 1);
    for round in 1..=MAX_ROUNDS {
        writeln!(output, "Round {round}/{MAX_ROUNDS}. Guess 1-100:")?;
        output.flush()?;

        line.clear();
        (&mut input)
            .take((MAX_LINE_BYTES + 1) as u64)
            .read_until(b'\n', &mut line)?;
        if line.is_empty() {
            return Ok(Outcome::EndOfInput);
        }
        if line.len() > MAX_LINE_BYTES {
            return Ok(Outcome::LineTooLong);
        }
        let Some(guess) = parse_guess(&line) else {
            writeln!(output, "Enter a whole number from 1 to 100.")?;
            continue;
        };
        match guess.cmp(&secret) {
            Ordering::Less => writeln!(output, "Too low.")?,
            Ordering::Greater => writeln!(output, "Too high.")?,
            Ordering::Equal => {
                writeln!(output, "Correct.")?;
                output.flush()?;
                return Ok(Outcome::Won);
            }
        }
    }
    writeln!(output, "No rounds left. The number was {secret}.")?;
    output.flush()?;
    Ok(Outcome::Exhausted)
}
