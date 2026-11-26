use std::{
    env,
    io::{self, Write},
    process::ExitCode,
    time::{SystemTime, UNIX_EPOCH},
};

use art_guess::{play, Outcome};

fn diagnostic(code: u8, message: &str) -> ExitCode {
    let _ = writeln!(io::stderr().lock(), "{message}");
    ExitCode::from(code)
}

fn time_seed() -> u64 {
    let elapsed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|error| error.duration());
    elapsed.as_secs() ^ u64::from(elapsed.subsec_nanos())
}

fn main() -> ExitCode {
    let mut args = env::args_os().skip(1);
    let seed = match (args.next(), args.next(), args.next()) {
        (None, None, None) => time_seed(),
        (Some(flag), Some(value), None) if flag == "--seed" => {
            let Some(seed) = value.to_str().and_then(|value| value.parse().ok()) else {
                return diagnostic(2, "expected an unsigned 64-bit decimal seed");
            };
            seed
        }
        _ => return diagnostic(2, "usage: art-guess [--seed <decimal>]"),
    };

    match play(seed, io::stdin().lock(), io::stdout().lock()) {
        Ok(Outcome::Won) => ExitCode::SUCCESS,
        Ok(Outcome::Exhausted) => ExitCode::from(1),
        Ok(Outcome::EndOfInput) => diagnostic(2, "input ended before a guess"),
        Ok(Outcome::LineTooLong) => diagnostic(2, "guess exceeds the 32-byte line limit"),
        Err(_) => diagnostic(1, "could not read or write the game"),
    }
}
