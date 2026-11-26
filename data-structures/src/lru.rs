use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
pub enum Insertion<K, V> {
    Stored,
    Updated(V),
    Evicted(K, V),
    Rejected(K, V),
}

pub struct SmallLru<K, V> {
    capacity: usize,
    entries: VecDeque<(K, V)>,
}

impl<K: Eq, V> SmallLru<K, V> {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            entries: VecDeque::new(),
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        let index = self.entries.iter().position(|(stored, _)| stored == key)?;
        let pair = self.entries.remove(index)?;
        self.entries.push_back(pair);
        self.entries.back().map(|(_, value)| value)
    }

    pub fn insert(&mut self, key: K, value: V) -> Insertion<K, V> {
        if self.capacity == 0 {
            return Insertion::Rejected(key, value);
        }
        let index = self.entries.iter().position(|(stored, _)| stored == &key);
        if let Some((_, prior)) = index.and_then(|index| self.entries.remove(index)) {
            self.entries.push_back((key, value));
            return Insertion::Updated(prior);
        }
        let evicted = if self.entries.len() == self.capacity {
            self.entries.pop_front()
        } else {
            None
        };
        self.entries.push_back((key, value));
        match evicted {
            Some((key, value)) => Insertion::Evicted(key, value),
            None => Insertion::Stored,
        }
    }
}
