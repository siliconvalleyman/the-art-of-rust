/// Return the first and last items without requiring any operation on T.
pub fn ends<T>(values: &[T]) -> Option<(&T, &T)> {
    Some((values.first()?, values.last()?))
}

#[derive(Debug, PartialEq, Eq)]
pub struct Pair<A, B> {
    left: A,
    right: B,
}

impl<A, B> Pair<A, B> {
    pub fn new(left: A, right: B) -> Self {
        Self { left, right }
    }

    pub fn reversed(self) -> Pair<B, A> {
        Pair::new(self.right, self.left)
    }

    pub fn into_parts(self) -> (A, B) {
        (self.left, self.right)
    }
}

/// Report the count of initialized positions in a fixed-size array.
pub fn array_count<T, const N: usize>(_: &[T; N]) -> usize {
    N
}
