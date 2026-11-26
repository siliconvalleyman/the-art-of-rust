use functional::callbacks::{make_once, sample_two, visit_all};

#[test]
fn callbacks_can_share_mutate_and_transfer_captured_values() {
    let lookup = [10, 20];
    assert_eq!(sample_two(|index| lookup[usize::from(index)]), [10, 20]);
    assert_eq!(lookup, [10, 20]);
    let mut total = 0_u32;
    visit_all(&[], |value| total += u32::from(value));
    assert_eq!(total, 0);
    visit_all(&[3, 5], |value| total += u32::from(value));
    assert_eq!(total, 8);
    let text = String::from("take ownership");
    assert_eq!(make_once(move || text), "take ownership");
}
