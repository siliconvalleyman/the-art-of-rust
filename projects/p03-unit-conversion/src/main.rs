use std::{
    env,
    io::{self, Write},
    process::ExitCode,
};

use art_convert::{ConversionError, Length, Unit};

fn diagnostic(code: u8, message: impl std::fmt::Display) -> ExitCode {
    let _ = writeln!(io::stderr().lock(), "{message}");
    ExitCode::from(code)
}

fn main() -> ExitCode {
    let mut args = env::args_os().skip(1);
    let (Some(quantity), Some(from), Some(to), places, None) = (
        args.next(),
        args.next(),
        args.next(),
        args.next(),
        args.next(),
    ) else {
        return diagnostic(2, "usage: art-convert <quantity> <from> <to> [places]");
    };

    let conversion = (|| {
        let from = Unit::parse(&from)?;
        let to = Unit::parse(&to)?;
        let length = Length::parse(&quantity, from)?;
        let places = places
            .map(|value| {
                value
                    .to_str()
                    .and_then(|text| text.parse().ok())
                    .ok_or(ConversionError::Precision)
            })
            .transpose()?;
        length.render(to, places)
    })();

    match conversion {
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
        Err(error) => diagnostic(2, error),
    }
}
