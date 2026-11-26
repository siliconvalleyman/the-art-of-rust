use std::mem::{align_of, size_of};

#[repr(C)]
struct Record {
    tag: u8,
    count: u32,
    flags: u16,
}

fn main() {
    let record = Record {
        tag: 1,
        count: 42,
        flags: 3,
    };
    let base = std::ptr::addr_of!(record) as usize;
    let tag = std::ptr::addr_of!(record.tag) as usize - base;
    let count = std::ptr::addr_of!(record.count) as usize - base;
    let flags = std::ptr::addr_of!(record.flags) as usize - base;

    println!(
        "size={}, alignment={}, tag={tag}, count={count}, flags={flags}",
        size_of::<Record>(),
        align_of::<Record>()
    );
}
