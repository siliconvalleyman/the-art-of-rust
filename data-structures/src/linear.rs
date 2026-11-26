#[derive(Debug)]
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct Queue<T, const N: usize> {
    slots: [Option<T>; N],
    first: usize,
    len: usize,
}

impl<T, const N: usize> Queue<T, N> {
    pub fn new() -> Self {
        Self {
            slots: std::array::from_fn(|_| None),
            first: 0,
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn push(&mut self, item: T) -> Result<(), T> {
        if self.len == N {
            return Err(item);
        }
        let suffix = N - self.first;
        let end = if self.len < suffix {
            self.first + self.len
        } else {
            self.len - suffix
        };
        debug_assert!(self.slots[end].is_none());
        self.slots[end] = Some(item);
        self.len += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let item = self.slots[self.first].take();
        debug_assert!(item.is_some());
        self.first += 1;
        if self.first == N {
            self.first = 0;
        }
        self.len -= 1;
        item
    }
}

impl<T, const N: usize> Default for Queue<T, N> {
    fn default() -> Self {
        Self::new()
    }
}
