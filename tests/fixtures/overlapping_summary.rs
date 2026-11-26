use std::fmt::Display;

trait Summary {
    fn summary(&self) -> String;
}

impl<T: Display> Summary for T {
    fn summary(&self) -> String {
        self.to_string()
    }
}

impl Summary for String {
    fn summary(&self) -> String {
        format!("text: {self}")
    }
}

fn main() {}
