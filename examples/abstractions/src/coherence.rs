use std::fmt::{self, Display, Formatter};

/// A local trait can extend a type from another crate.
pub trait HasLabel {
    fn has_label(&self) -> bool;
}

impl HasLabel for str {
    fn has_label(&self) -> bool {
        !self.trim().is_empty()
    }
}

/// A local wrapper can define its own formatting behavior.
pub struct DispatchId(pub u32);

impl Display for DispatchId {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "dispatch-{}", self.0)
    }
}
