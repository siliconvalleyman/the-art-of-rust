use std::{cell::Cell, collections::VecDeque, rc::Rc};

use data_structures::linear::{Queue, Stack};

#[test]
fn stack_peeks_and_pops_in_reverse_order() {
    let mut stack = Stack::new();
    assert_eq!(stack.pop(), None);
    stack.push(String::from("first"));
    stack.push(String::from("second"));
    assert_eq!(stack.peek().map(String::as_str), Some("second"));
    assert_eq!(stack.pop().as_deref(), Some("second"));
    assert_eq!(stack.pop().as_deref(), Some("first"));
    assert!(stack.peek().is_none());
}

fn compare_all_short_histories<const N: usize>() {
    for mut encoded in 0..3_u32.pow(7) {
        let mut actual = Queue::<_, N>::new();
        let mut expected = VecDeque::new();
        for _ in 0..7 {
            match encoded % 3 {
                2 => assert_eq!(actual.pop(), expected.pop_front()),
                value => {
                    if expected.len() == N {
                        assert_eq!(actual.push(value), Err(value));
                    } else {
                        expected.push_back(value);
                        assert_eq!(actual.push(value), Ok(()));
                    }
                }
            }
            encoded /= 3;
            assert_eq!(actual.len(), expected.len());
            assert_eq!(actual.is_empty(), expected.is_empty());
        }
        while let Some(value) = expected.pop_front() {
            assert_eq!(actual.pop(), Some(value));
        }
        assert_eq!(actual.pop(), None);
    }
}

#[test]
fn bounded_queues_match_a_reference_across_short_histories() {
    compare_all_short_histories::<0>();
    compare_all_short_histories::<1>();
    compare_all_short_histories::<2>();
    compare_all_short_histories::<3>();
}

#[test]
fn full_queue_returns_ownership_and_drops_only_occupied_slots() {
    struct Probe(Rc<Cell<u8>>);

    impl Drop for Probe {
        fn drop(&mut self) {
            self.0.set(self.0.get() + 1);
        }
    }

    let drops = Rc::new(Cell::new(0));
    let mut queue = Queue::<_, 2>::new();
    assert!(queue.push(Probe(Rc::clone(&drops))).is_ok());
    assert!(queue.push(Probe(Rc::clone(&drops))).is_ok());
    let Err(rejected) = queue.push(Probe(Rc::clone(&drops))) else {
        panic!("the third item should have been returned");
    };
    let removed = queue.pop().unwrap();
    assert_eq!(drops.get(), 0);
    drop(queue);
    assert_eq!(drops.get(), 1);
    drop(removed);
    drop(rejected);
    assert_eq!(drops.get(), 3);
}
