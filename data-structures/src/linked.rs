struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

pub struct FrontList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> FrontList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push_front(&mut self, value: T) {
        let next = self.head.take();
        self.head = Some(Box::new(Node { value, next }));
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let mut node = self.head.take()?;
        self.head = node.next.take();
        Some(node.value)
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        std::iter::successors(self.head.as_deref(), |node| node.next.as_deref())
            .map(|node| &node.value)
    }
}

impl<T> Default for FrontList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for FrontList<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}
