use std::{error::Error, io, num::ParseIntError, panic};

use reliability::{
    batches::{apply_reservations, BatchError, MAX_BYTES},
    boundaries::{publish_score, WorkerPanicked},
    quantities::{load_quantity, LoadQuantityError},
    row_errors::parse_row,
    stock::{InsufficientStock, Stock},
};

#[test]
fn all_initial_capacities_and_requests_follow_the_reservation_contract() {
    for total in 0..=u8::MAX {
        for request in 0..=u8::MAX {
            let mut stock = Stock::new(total);
            if request <= total {
                assert_eq!(stock.reserve(request), Ok(()));
                assert_eq!(stock.available(), total - request);
                assert_eq!(stock.held(), request);
            } else {
                assert_eq!(
                    stock.reserve(request),
                    Err(InsufficientStock {
                        requested: request,
                        available: total,
                    })
                );
                assert_eq!(stock, Stock::new(total));
            }
        }
    }
}

#[test]
fn later_rejection_preserves_prior_successes() {
    let mut stock = Stock::new(7);
    assert_eq!(stock.reserve(4), Ok(()));
    let snapshot = stock;
    assert_eq!(
        stock.reserve(5),
        Err(InsufficientStock {
            requested: 5,
            available: 3
        })
    );
    assert_eq!(stock, snapshot);
    assert_eq!(stock.reserve(3), Ok(()));
    assert_eq!((stock.available(), stock.held()), (0, 7));
    assert_eq!(stock.reserve(0), Ok(()));
}

struct BrokenReader;

impl io::Read for BrokenReader {
    fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
        Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "sample failure",
        ))
    }
}

#[test]
fn the_loader_converts_each_error_at_the_corresponding_boundary() {
    assert_eq!(load_quantity(&b" 65535\r\n"[..]).unwrap(), u16::MAX);
    assert!(matches!(
        load_quantity(&b"65536"[..]),
        Err(LoadQuantityError::Parse(_))
    ));
    assert!(
        matches!(load_quantity(BrokenReader), Err(LoadQuantityError::Read(error))
        if error.kind() == io::ErrorKind::PermissionDenied)
    );
    assert!(
        matches!(load_quantity(&[0xff][..]), Err(LoadQuantityError::Read(error))
        if error.kind() == io::ErrorKind::InvalidData)
    );
}

#[test]
fn contextual_errors_keep_the_cause_and_omit_input_from_display() {
    let error = parse_row(3, "private field").unwrap_err();
    assert_eq!(error.line(), 3);
    assert_eq!(error.to_string(), "invalid quantity on line 3");
    let cause = error
        .source()
        .unwrap()
        .downcast_ref::<ParseIntError>()
        .unwrap();
    assert_eq!(cause.kind(), "x".parse::<u8>().unwrap_err().kind());
    assert!(cause.source().is_none());
}

#[cfg(panic = "unwind")]
#[test]
fn the_publication_boundary_drops_worker_locals_and_preserves_previous_score() {
    use std::sync::atomic::{AtomicBool, Ordering};

    struct Guard<'a>(&'a AtomicBool);

    impl Drop for Guard<'_> {
        fn drop(&mut self) {
            self.0.store(true, Ordering::Relaxed);
        }
    }

    let dropped = AtomicBool::new(false);
    let mut score = 11;
    assert_eq!(
        publish_score(&mut score, || {
            let _guard = Guard(&dropped);
            panic::panic_any("controlled demonstration");
        }),
        Err(WorkerPanicked)
    );
    assert!(dropped.load(Ordering::Relaxed));
    assert_eq!(score, 11);
    assert_eq!(publish_score(&mut score, || 20), Ok(()));
    assert_eq!(score, 20);
}

#[cfg(panic = "unwind")]
#[test]
fn an_unrecognized_payload_reaches_the_outer_unwind_boundary() {
    let outer = panic::catch_unwind(|| {
        let mut score = 11;
        let _result = publish_score(&mut score, || panic::panic_any(17_u8));
    });
    assert_eq!(*outer.unwrap_err().downcast::<u8>().unwrap(), 17);
}

#[test]
fn batches_publish_only_after_every_row_succeeds() {
    let mut stock = Stock::new(7);
    assert!(apply_reservations(&mut stock, "2\r\n4\r\n").is_ok());
    assert_eq!((stock.available(), stock.held()), (1, 6));
    let snapshot = stock;
    assert!(matches!(
        apply_reservations(&mut stock, "1\n1"),
        Err(BatchError::Stock {
            line: 2,
            reason: InsufficientStock {
                available: 0,
                requested: 1
            }
        })
    ));
    assert_eq!(stock, snapshot);
    assert!(matches!(apply_reservations(&mut stock, "1\n\nx"),
        Err(BatchError::Quantity(error)) if error.line() == 2));
    assert_eq!(stock, snapshot);
    assert!(apply_reservations(&mut stock, "").is_ok());
    assert_eq!(stock, snapshot);
}

#[test]
fn batch_limits_and_error_precedence_are_deterministic() {
    let mut stock = Stock::new(4);
    let boundary = "0".repeat(MAX_BYTES);
    assert!(apply_reservations(&mut stock, &boundary).is_ok());
    assert!(matches!(
        apply_reservations(&mut stock, &format!("x{boundary}")),
        Err(BatchError::TooManyBytes)
    ));
    assert!(matches!(
        apply_reservations(&mut stock, &"é".repeat(MAX_BYTES / 2 + 1)),
        Err(BatchError::TooManyBytes)
    ));
    assert!(matches!(
        apply_reservations(&mut stock, "1\n1\n1\n1\nx"),
        Err(BatchError::TooManyLines)
    ));
    assert!(matches!(
        apply_reservations(&mut stock, "1\n9\n0\n0\n0"),
        Err(BatchError::Stock { line: 2, .. })
    ));
    assert_eq!(stock, Stock::new(4));
}
