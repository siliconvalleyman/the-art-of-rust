use std::{
    cell::Cell,
    collections::{LinkedList, VecDeque},
    rc::Rc,
};

use data_structures::linked::FrontList;

#[test]
fn front_changes_agree_with_a_vector_stack() {
    for mut history in 0..3_u32.pow(7) {
        let mut list = FrontList::new();
        let mut expected = Vec::new();
        for _ in 0..7 {
            match history % 3 {
                2 => assert_eq!(list.pop_front(), expected.pop()),
                value => {
                    list.push_front(value);
                    expected.push(value);
                }
            }
            assert!(list.iter().eq(expected.iter().rev()));
            history /= 3;
        }
    }
}

#[test]
fn normal_drop_drains_the_remaining_chain() {
    struct Probe(Rc<Cell<usize>>);

    impl Drop for Probe {
        fn drop(&mut self) {
            self.0.set(self.0.get() + 1);
        }
    }

    const COUNT: usize = 25_000;
    let drops = Rc::new(Cell::new(0));
    let mut list = FrontList::new();
    for _ in 0..COUNT {
        list.push_front(Probe(Rc::clone(&drops)));
    }
    let removed = list.pop_front().unwrap();
    drop(list);
    assert_eq!(drops.get(), COUNT - 1);
    drop(removed);
    assert_eq!(drops.get(), COUNT);
}

#[test]
fn standard_double_ended_containers_preserve_order() {
    let mut left = LinkedList::from([1, 2]);
    let mut right = LinkedList::from([3, 4]);
    left.append(&mut right);
    assert!(right.is_empty());
    assert_eq!(left.pop_back(), Some(4));
    assert!(left.into_iter().eq([1, 2, 3]));

    let mut ring = VecDeque::from([1, 2, 3]);
    assert_eq!(ring.pop_front(), Some(1));
    ring.push_back(4);
    let (front, back) = ring.as_slices();
    assert!(front.iter().chain(back).copied().eq([2, 3, 4]));
}
