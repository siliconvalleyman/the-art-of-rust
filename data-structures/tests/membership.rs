use std::{
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
};

use data_structures::{
    lru::{Insertion, SmallLru},
    membership::Bloom64,
};

#[derive(Debug, PartialEq, Eq)]
struct CollidingKey(u8);

impl Hash for CollidingKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u8(0);
    }
}

#[test]
fn collisions_do_not_merge_unequal_keys() {
    let mut names = HashMap::new();
    assert_eq!(names.insert(CollidingKey(1), "first"), None);
    assert_eq!(names.insert(CollidingKey(2), "second"), None);
    assert_eq!(names.insert(CollidingKey(1), "new"), Some("first"));
    assert_eq!(names.get(&CollidingKey(2)), Some(&"second"));
    assert_eq!(names.remove(&CollidingKey(1)), Some("new"));
    assert_eq!(names.len(), 1);

    let mut seen = HashSet::new();
    assert!(seen.insert(CollidingKey(1)));
    assert!(seen.insert(CollidingKey(2)));
    assert!(!seen.insert(CollidingKey(1)));
    assert_eq!(seen.len(), 2);
}

#[test]
fn bloom_queries_match_the_fixed_occupancy_count() {
    let mut filter = Bloom64::default();
    assert!(!filter.might_contain([2, 2]));
    filter.insert([1, 2]);
    filter.insert([3, 4]);
    assert!(filter.might_contain([1, 2]));
    assert!(filter.might_contain([3, 4]));
    assert!(filter.might_contain([1, 4]));
    assert!(!filter.might_contain([1, 5]));
    let mut passing = 0;
    for first in 0..64 {
        for second in 0..64 {
            passing += usize::from(filter.might_contain([first, second]));
        }
    }
    assert_eq!(passing, 16);
}

#[test]
fn lru_hits_and_replacements_change_recency() {
    let mut cache = SmallLru::new(2);
    assert_eq!(cache.insert("A", 1), Insertion::Stored);
    assert_eq!(cache.insert("B", 2), Insertion::Stored);
    assert_eq!(cache.get(&"A"), Some(&1));
    assert_eq!(cache.get(&"Z"), None);
    assert_eq!(cache.insert("C", 3), Insertion::Evicted("B", 2));
    assert_eq!(cache.insert("A", 4), Insertion::Updated(1));
    assert_eq!(cache.insert("D", 5), Insertion::Evicted("C", 3));
    assert_eq!(cache.get(&"A"), Some(&4));
    assert_eq!(cache.get(&"B"), None);
}

#[test]
fn zero_capacity_rejects_and_returns_the_pair() {
    let mut cache = SmallLru::new(0);
    assert_eq!(
        cache.insert(1, String::from("owned")),
        Insertion::Rejected(1, String::from("owned"))
    );
    assert_eq!(cache.get(&1), None);
}
