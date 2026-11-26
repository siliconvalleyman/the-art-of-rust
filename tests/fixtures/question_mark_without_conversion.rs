fn first() -> Result<u8, u16> {
    Err(1)
}

fn second() -> Result<u8, String> {
    Ok(first()?)
}

fn main() {
    let _ = second();
}
