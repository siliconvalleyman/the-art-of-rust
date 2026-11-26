use std::panic::{catch_unwind, UnwindSafe};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WorkerPanicked;

pub fn publish_score(
    published: &mut u16,
    compute: impl FnOnce() -> u16 + UnwindSafe,
) -> Result<(), WorkerPanicked> {
    let score = match catch_unwind(compute) {
        Ok(score) => score,
        Err(payload) => match payload.downcast::<&'static str>() {
            Ok(_) => return Err(WorkerPanicked),
            Err(payload) => std::panic::resume_unwind(payload),
        },
    };
    *published = score;
    Ok(())
}
