use std::{
    io::{self, Write},
    process::{Command, Output, Stdio},
};

use art_guess::{play, secret_for_seed, Outcome, MAX_LINE_BYTES, MAX_ROUNDS};

#[test]
fn replayed_seeds_have_stable_numbers_in_range() {
    assert_eq!(secret_for_seed(1), 62);
    assert_eq!(secret_for_seed(2), 23);
    assert_eq!(secret_for_seed(17), 94);
    assert_eq!(secret_for_seed(0), secret_for_seed(1));
    assert!((0..10_000)
        .map(secret_for_seed)
        .all(|number| (1..=100).contains(&number)));
}

#[test]
fn each_consumed_line_uses_one_round_until_the_win() {
    let input = &b"1\n100\n\xff\r\n62\nnever read\n"[..];
    let mut output = Vec::new();
    assert_eq!(play(1, input, &mut output).unwrap(), Outcome::Won);
    assert_eq!(
        String::from_utf8(output).unwrap(),
        concat!(
            "Round 1/7. Guess 1-100:\nToo low.\n",
            "Round 2/7. Guess 1-100:\nToo high.\n",
            "Round 3/7. Guess 1-100:\nEnter a whole number from 1 to 100.\n",
            "Round 4/7. Guess 1-100:\nCorrect.\n",
        )
    );
}

#[test]
fn encoded_byte_boundary_and_eof_have_distinct_outcomes() {
    let exact = format!("{}62\n", " ".repeat(MAX_LINE_BYTES - 3));
    assert_eq!(play(1, exact.as_bytes(), io::sink()).unwrap(), Outcome::Won);
    let excess = format!(" {exact}");
    let mut input = excess.as_bytes();
    assert_eq!(
        play(1, &mut input, io::sink()).unwrap(),
        Outcome::LineTooLong
    );
    assert!(input.is_empty());
    let mut input = &b""[..];
    assert_eq!(
        play(1, &mut input, io::sink()).unwrap(),
        Outcome::EndOfInput
    );
    assert_eq!(play(1, &b"62"[..], io::sink()).unwrap(), Outcome::Won);
}

#[test]
fn input_after_the_last_round_is_left_unconsumed() {
    let turns = format!("{}62\n", "bad\n".repeat(usize::from(MAX_ROUNDS)));
    let mut input = turns.as_bytes();
    let mut output = Vec::new();
    assert_eq!(
        play(1, &mut input, &mut output).unwrap(),
        Outcome::Exhausted
    );
    assert_eq!(input, b"62\n");
    assert!(output.ends_with(b"No rounds left. The number was 62.\n"));
}

fn run(args: &[&str], input: &[u8]) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_art-guess"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("launch the guessing game");
    child.stdin.take().unwrap().write_all(input).unwrap();
    child.wait_with_output().unwrap()
}

#[test]
fn the_cli_exposes_replay_and_reports_distinct_failures() {
    let win = run(&["--seed", "1"], b"62\r\n");
    assert_eq!(win.status.code(), Some(0));
    assert!(win.stdout.ends_with(b"Correct.\n"));
    assert!(win.stderr.is_empty());

    let short = run(&["--seed", "1"], b"");
    assert_eq!(short.status.code(), Some(2));
    assert_eq!(short.stderr, b"input ended before a guess\n");

    let malformed = run(&["--seed", "-1"], b"");
    assert_eq!(malformed.status.code(), Some(2));
    assert!(malformed.stdout.is_empty());
    assert_eq!(
        malformed.stderr,
        b"expected an unsigned 64-bit decimal seed\n"
    );
}
