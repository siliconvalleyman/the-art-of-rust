# 020. Address spaces, pages, and resource limits

A file viewer requests a range that starts two bytes
before a page boundary. A single logical request can
touch two pages with different permissions or backing
storage. The implementation must reason about which
pages the whole range covers and how much work or
resident memory a large request can require.

**Prerequisites.** [Operating systems and translation](../part-01/005-operating-systems.md)
and [hierarchy, allocation, and locality](019-allocation-and-locality.md).
After this chapter, you should be able to derive the
pages covered by a checked range, trace page permissions
and residency, and design a resource limit that names
the quantity it bounds.

## A mapping gives the page and offset meaning

A virtual address identifies a location within one
address space. With page size `P`, its page number
is the quotient on division by `P`, and its offset
is the remainder. When `P` is a power of two, the
low bits specify the offset and the remaining bits
specify the page number. A mapping gives the page's
permitted operations and supplies the physical frame
when the page is resident and the operation is permitted.

The illustrative pages below have 16 bytes. In
practice, page sizes and permission details depend
on the architecture and operating system. Some
platforms allow several page sizes in one address
space. Use the documented model when interpreting
mappings and fault reports.

| Virtual page | Frame when resident | Read | Write |
| ---: | ---: | --- | --- |
| 0 | 10 | allowed | allowed |
| 1 | 3 | allowed | denied |
| 2 | absent until resolved | allowed | denied |

*Figure 1. Adjacent virtual pages can map to different
physical frames, or one can require fault handling.*

For virtual address `0x0e`, page 0 has offset 14.
It translates here to `10 × 16 + 14 = 174`, or
`0xae`. Virtual address `0x10` starts page 1
and translates to `3 × 16 = 48`. A read can succeed
on both pages, although the resulting physical positions
are not adjacent. A write can succeed on page 0 and
be denied on page 1. This table is an explanatory
model and does not imply that a real language
operation spanning pages completes partly on failure.
That behavior depends on the operation and platform.

## Determine the covered pages without overflow

Given nonnegative start `s`, length `n`, and a valid
page size `P`, find the first and last page containing
at least one byte in the requested range. For empty input
there is no page to return. Otherwise the final included
address is `s + n − 1`. Computing `s + (n−1)` with a
checked addition allows a valid range ending at the last
representable address, even when its exclusive end is not
representable.

For `n > 0`, the page number is a nondecreasing function
of the byte address. The first and last pages therefore are
`s div P` and `(s+n−1) div P`. Every page between
them is covered: aside from the first page, each page's
starting address lies between the beginning and end.
This proves that a contiguous page range suffices. No
individual byte scan is necessary.

```rust,edition2021
#[derive(Debug, PartialEq, Eq)]
enum RangeError {
    InvalidPageSize,
    AddressOverflow,
}

fn covered_pages(
    start: u64,
    length: u64,
    page_size: u64,
) -> Result<Option<(u64, u64)>, RangeError> {
    if !page_size.is_power_of_two() {
        return Err(RangeError::InvalidPageSize);
    }
    if length == 0 {
        return Ok(None);
    }
    let end = start
        .checked_add(length - 1)
        .ok_or(RangeError::AddressOverflow)?;
    Ok(Some((start / page_size, end / page_size)))
}

assert_eq!(covered_pages(14, 4, 16), Ok(Some((0, 1))));
assert_eq!(covered_pages(16, 0, 16), Ok(None));
assert_eq!(covered_pages(u64::MAX, 1, 1), Ok(Some((u64::MAX, u64::MAX))));
assert_eq!(covered_pages(u64::MAX, 2, 16), Err(RangeError::AddressOverflow));
assert_eq!(covered_pages(0, 8, 0), Err(RangeError::InvalidPageSize));
```

The error distinguishes invalid configuration from a
range that cannot be represented in the input address
type. An empty valid request produces absence. The
`?` returns early when the checked addition reports
an error. The arithmetic performs a fixed number of
word operations. A caller enumerating every page to
check permission still incurs work proportional to the
number of pages checked. It may also need to synchronize
against concurrent mapping changes. A span alone grants
no access to those pages.

## Distinguish permission, residency, and availability

An address in a valid logical range can still refer to
a denied mapping. A mapping can allow a read and be
temporarily nonresident. A **page fault** can bring a
valid page into memory or signal an operation that the
kernel cannot authorize or complete. When it can resolve
the fault, the instruction can resume under the platform's
rules. An unavailable mapping and a resolved fault have
different effects even if both initially interrupt a
machine operation.

Mapping a file can allow the system to read pages from
the file as the process touches them. The size of the
mapped range need not equal the number of bytes
currently resident in physical memory. A **working set**
in a workload model is a collection of pages actively
used during a chosen recent window. Some operating
systems also use the term for the resident pages accounted
to a process. State the sense being measured.
[Microsoft documentation on virtual address space](https://learn.microsoft.com/en-us/windows/win32/memory/virtual-address-space).

Imagine a scan cycling through pages 0, 1, and 2.
With two resident frames available and an initially empty
least-recently-used model, the first three requests fault.
Afterwards, each requested page differs from the two most
recently demanded pages, which are the resident ones.
The cycle continues to fault. With three frames, the
same trace faults three times and then reuses the
resident pages. The large difference follows from
residency and access order. An actual OS can group
I/O, use different replacement information, or count
faults that never wait for storage.

## Apply limits before the expensive operation

A service that handles ranges from a request should
check the length, arithmetic, and upper bound before
starting work. A byte limit bounds transferred bytes,
while a limit on pages or I/O operations bounds a
different resource. A tiny range crossing a page boundary
can require two page accesses. A large valid length can
still consume excessive memory when several requests are
active. Apply a limit that expresses the workload and
reserve shared capacity together with accepting work when
concurrent requests must honor one total.

Rust collections can request capacity ahead of time.
`Vec::try_reserve` provides a fallible interface for
capacity overflow or reported allocation failure.
On success the vector can hold its current length plus
the requested additional elements. That result does
not serve as a measurement that all
pages are resident or as a promise that external resources
will remain available. Check which failure conditions the
toolchain and platform expose before relying on a recovery
path. Use bounded chunks when a contract permits processing
data incrementally.
[Rust standard library documentation for Vec](https://doc.rust-lang.org/1.75.0/std/vec/struct.Vec.html#method.try_reserve).

## Practice and assessment

{{#include ../../exercises/C020.md}}

## Summary

Compute the last included address with checked arithmetic.
Page numbers describe positions within mappings, permissions
constrain access, and residency affects service cost. Name
the resource a limit protects and check the limit before
committing the work.

[Hierarchy, allocation, fragmentation, and locality](019-allocation-and-locality.md) · [Solutions](../memory-solutions.md#chapter-020) · [Ownership, moves, Copy, and Clone](../part-05/021-ownership-and-moves.md)
