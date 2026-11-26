use std::{
    env,
    io::{self, Write},
    process::ExitCode,
};

use art_calc::{calculate, CalcError};

const USAGE: &str = "usage: art-calc <left> <operator> <right>";

fn diagnostic(code: u8, message: impl std::fmt::Display) -> ExitCode {
    let _ = writeln!(io::stderr().lock(), "{message}");
    ExitCode::from(code)
}

fn main() -> ExitCode {
    let mut args = env::args_os().skip(1);
    let (Some(left), Some(operator), Some(right), None) =
        (args.next(), args.next(), args.next(), args.next())
    else {
        return diagnostic(2, USAGE);
    };

    match calculate(&left, &operator, &right) {
        Ok(value) => {
            let mut stdout = io::stdout().lock();
            if writeln!(stdout, "{value}")
                .and_then(|()| stdout.flush())
                .is_err()
            {
                return diagnostic(1, "could not write result");
            }
            ExitCode::SUCCESS
        }
        Err(error @ (CalcError::Operand | CalcError::Operator)) => diagnostic(2, error),
        Err(error) => diagnostic(1, error),
    }
}
