fn main() {
    let label = String::from("parcel");
    let measure = move || label.len();
    println!("{}", measure());
    println!("{label}");
}
