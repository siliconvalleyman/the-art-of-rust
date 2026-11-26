use std::marker::PhantomData;
use std::num::NonZeroUsize;

/// Marker types distinguish units without storing a per-value unit code.
#[repr(transparent)]
pub struct Quantity<Unit> {
    amount: u32,
    unit: PhantomData<Unit>,
}

impl<Unit> Quantity<Unit> {
    pub fn new(amount: u32) -> Self {
        Self {
            amount,
            unit: PhantomData,
        }
    }

    pub fn amount(&self) -> u32 {
        self.amount
    }
}

/// A callback must accept a borrow chosen anew at each call.
pub fn inspect_two<F>(inspect: F) -> [usize; 2]
where
    F: for<'a> Fn(&'a str) -> usize,
{
    let first = String::from("zone");
    let first_result = inspect(&first);
    let second_result = {
        let second = String::from("aisle");
        inspect(&second)
    };
    [first_result, second_result]
}

pub trait Lending {
    type Item<'a>
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>>;
}

/// Visit every overlapping window of a strictly positive width.
pub struct ByteWindows<'data> {
    bytes: &'data mut [u8],
    width: NonZeroUsize,
    start: usize,
}

impl<'data> ByteWindows<'data> {
    pub fn new(bytes: &'data mut [u8], width: NonZeroUsize) -> Self {
        Self {
            bytes,
            width,
            start: 0,
        }
    }
}

impl Lending for ByteWindows<'_> {
    type Item<'a> = &'a mut [u8] where Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        let end = self.start.checked_add(self.width.get())?;
        let window = self.bytes.get_mut(self.start..end)?;
        self.start += 1;
        Some(window)
    }
}
