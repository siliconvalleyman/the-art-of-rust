mod permits {
    use std::cell::Cell;

    pub struct Pool {
        free: Cell<usize>,
    }

    pub struct Lease<'a> {
        pool: &'a Pool,
    }

    impl Pool {
        pub fn new(capacity: usize) -> Self {
            Self {
                free: Cell::new(capacity),
            }
        }

        pub fn available(&self) -> usize {
            self.free.get()
        }

        pub fn acquire(&self) -> Option<Lease<'_>> {
            let remaining = self.available().checked_sub(1)?;
            self.free.set(remaining);
            Some(Lease { pool: self })
        }
    }

    impl Drop for Lease<'_> {
        fn drop(&mut self) {
            self.pool.free.set(self.pool.available() + 1);
        }
    }
}
