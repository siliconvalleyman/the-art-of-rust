trait Duplicate {
    fn duplicate(&self) -> Self;
}

fn accepts_object(_: &dyn Duplicate) {}

fn main() {}
