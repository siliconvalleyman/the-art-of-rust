struct Grams(u32);
struct Milligrams(u32);

fn label_count(weight: Grams) -> u32 {
    weight.0
}

fn main() {
    let sample = Milligrams(4_000);
    assert_eq!(label_count(sample), 4);
}
