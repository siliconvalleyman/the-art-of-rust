use foundations::{
    architecture, logic,
    machine::{self, Instruction::*, Stop},
    operating_systems::{self, Access, InterpretError, NestingError, Page, TranslationError},
    representation::{self, ByteOrder},
};

#[test]
fn machine_copies_state_and_counts_consumed_input() {
    let reverse = [Read, Store(0), Read, Write, Load(0), Write, Halt];
    let result = machine::run(&reverse, &[6, 9, 42], 7);
    assert_eq!(result.output, [9, 6]);
    assert_eq!(result.consumed_bytes, 2);
    assert_eq!(result.attempted_steps, 7);
    assert_eq!(result.stop, Stop::Halted);

    let missing = machine::run(&reverse, &[6], 7);
    assert!(missing.output.is_empty());
    assert_eq!(missing.stop, Stop::MissingInput);
}

#[test]
fn machine_bounds_work_and_preserves_output_before_failure() {
    assert_eq!(machine::run(&[Halt], &[], 0).stop, Stop::OutOfBudget);
    assert_eq!(machine::run(&[Halt], &[], 1).stop, Stop::Halted);
    let looping = machine::run(&[JumpIfZero(0)], &[], 8);
    assert_eq!(looping.stop, Stop::OutOfBudget);
    assert_eq!(looping.attempted_steps, 8);

    let partial = machine::run(&[Read, Write, Read, Halt], &[31], 4);
    assert_eq!(partial.output, [31]);
    assert_eq!(partial.stop, Stop::MissingInput);
    assert_eq!(partial.attempted_steps, 3);

    assert_eq!(
        machine::run(&[Store(4)], &[], 1).stop,
        Stop::InvalidAddress(4)
    );
    assert_eq!(
        machine::run(&[JumpIfZero(usize::MAX)], &[], 2).stop,
        Stop::MissingInstruction(usize::MAX)
    );
}

#[test]
fn machine_checks_both_branch_paths_and_wraps() {
    let program = [
        Read,
        JumpIfZero(8),
        Read,
        Store(0),
        Add(0),
        Write,
        Load(1),
        JumpIfZero(10),
        Read,
        Write,
        Halt,
    ];
    assert_eq!(machine::run(&program, &[0, 128], 11).output, [128]);
    assert_eq!(machine::run(&program, &[1, 128], 11).output, [0]);
    let missing = machine::run(&program, &[0], 11);
    assert!(missing.output.is_empty());
    assert_eq!(missing.stop, Stop::MissingInput);
}

#[test]
fn representation_checks_all_unsigned_additions() {
    for a in 0..=u8::MAX {
        for b in 0..=u8::MAX {
            let (sum, carry) = representation::add_byte(a, b);
            let expected = u16::from(a) + u16::from(b);
            assert_eq!(u16::from(sum) + 256 * u16::from(carry), expected);
            assert_eq!(
                representation::checked_byte_sum(a, b),
                expected.try_into().ok()
            );
        }
    }
}

#[test]
fn representation_decodes_known_formats_and_rejects_wrong_lengths() {
    assert_eq!(representation::byte_bits(37), [0, 0, 1, 0, 0, 1, 0, 1]);
    assert_eq!(representation::byte_bits(0), [0; 8]);
    assert_eq!(representation::byte_bits(255), [1; 8]);
    assert_eq!(representation::parse_hex_byte(b"aF"), Some(175));
    assert_eq!(representation::parse_hex_byte(b"00"), Some(0));
    assert_eq!(representation::parse_hex_byte(b"FF"), Some(255));
    for invalid in [b"".as_slice(), b"0x", b"0 ", b"0F ", "Ａ0".as_bytes()] {
        assert_eq!(representation::parse_hex_byte(invalid), None);
    }
    assert_eq!(
        representation::read_u16(&[0x12, 0x34], ByteOrder::Big),
        Some(0x1234)
    );
    assert_eq!(
        representation::read_u16(&[0x12, 0x34], ByteOrder::Little),
        Some(0x3412)
    );
    assert_eq!(representation::read_u16(&[0x12], ByteOrder::Big), None);
}

#[test]
fn gate_models_match_all_input_assignments() {
    for a in [false, true] {
        for b in [false, true] {
            for control in [false, true] {
                let (sum, carry) = logic::full_adder(a, b, control);
                assert_eq!(
                    u8::from(sum) + 2 * u8::from(carry),
                    u8::from(a) + u8::from(b) + u8::from(control)
                );
                assert_eq!(logic::alarm(a, b, control), !control && (a || b));
                assert_eq!(
                    logic::nand_selector(a, b, control),
                    if control { b } else { a }
                );
                assert_eq!(
                    logic::swap_registers((a, b), control),
                    if control { (b, a) } else { (a, b) }
                );
            }
        }
    }
    for value in 0..=u8::MAX {
        let bits = representation::byte_bits(value).map(|bit| bit != 0);
        assert_eq!(logic::zero_detect(bits), value == 0);
    }
}

#[test]
fn architecture_models_preserve_fields_and_cache_state() {
    let word = architecture::encode_add(7, 5, 6).unwrap();
    assert_eq!(word, 0x0062_83b3);
    assert_eq!(word.to_le_bytes(), [0xb3, 0x83, 0x62, 0]);
    assert_eq!(architecture::encode_add(3, 1, 2), Some(0x0020_81b3));
    assert_eq!(architecture::encode_add(32, 0, 0), None);

    assert_eq!(architecture::hits([]).count(), 0);
    assert!(architecture::hits([0, 3, 4, 7, 8, 0]).eq([false, true, false, true, false, false]));
    assert!(architecture::hits([0, 8, 0, 8, 0, 8]).eq([false; 6]));
    assert!(architecture::hits([0, 4, 0, 4, 0, 4]).eq([false, false, true, true, true, true]));
    for length in 0..=32 {
        let observed = architecture::hits(0..length).filter(|&hit| !hit).count() as u64;
        assert_eq!(architecture::sequential_misses(length), observed);
    }
    assert_eq!(architecture::sequential_misses(u64::MAX), 1 << 62);
}

#[test]
fn translation_uses_the_supplied_map_and_rejects_forbidden_access() {
    let mut a = [None; 16];
    let mut b = [None; 16];
    a[2] = Some(Page {
        frame: 9,
        writable: true,
    });
    b[2] = Some(Page {
        frame: 12,
        writable: false,
    });

    assert_eq!(
        operating_systems::translate(&a, 0x23, Access::Read),
        Ok(0x93)
    );
    assert_eq!(
        operating_systems::translate(&a, 0x23, Access::Write),
        Ok(0x93)
    );
    assert_eq!(
        operating_systems::translate(&b, 0x23, Access::Read),
        Ok(0xc3)
    );
    assert_eq!(
        operating_systems::translate(&b, 0x23, Access::Write),
        Err(TranslationError::Permission)
    );
    assert_eq!(
        operating_systems::translate(&a, 0x30, Access::Read),
        Err(TranslationError::Unmapped)
    );
}

#[test]
fn recognizers_report_boundary_cases() {
    assert!(operating_systems::even_parity(&[]));
    assert!(!operating_systems::even_parity(&[true]));
    assert!(operating_systems::even_parity(&[false, false]));
    assert!(!operating_systems::even_parity(&[true, false, true, true]));

    assert_eq!(operating_systems::bounded_parentheses(b"", 0), Ok(()));
    assert_eq!(operating_systems::bounded_parentheses(b"()()", 1), Ok(()));
    assert_eq!(
        operating_systems::bounded_parentheses(b"(()", 1),
        Err(NestingError::DepthLimit(1))
    );
    assert_eq!(
        operating_systems::bounded_parentheses(b")(", 1),
        Err(NestingError::UnmatchedClosing(0))
    );
    assert_eq!(
        operating_systems::bounded_parentheses(b"(", 1),
        Err(NestingError::Unfinished(1))
    );
    assert_eq!(
        operating_systems::bounded_parentheses(b"(?)", 1),
        Err(NestingError::InvalidSymbol(1))
    );
}

#[test]
fn interpreter_halts_before_examining_trailing_input() {
    assert_eq!(operating_systems::interpret_toggle(&[1, 0, 1, 2, 9]), Ok(0));
    assert_eq!(operating_systems::interpret_toggle(&[1, 2]), Ok(1));
    assert_eq!(
        operating_systems::interpret_toggle(&[]),
        Err(InterpretError::MissingHalt)
    );
    assert_eq!(
        operating_systems::interpret_toggle(&[0, 7, 2]),
        Err(InterpretError::InvalidOpcode { index: 1, byte: 7 })
    );
}
