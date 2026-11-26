fn store<'a>(slot: &mut &'a str, value: &'a str) {
    *slot = value;
}

fn main() {
    let mut slot: &'static str = "initial";
    let local = String::from("temporary");
    store(&mut slot, local.as_str());
    println!("{slot}");
}
