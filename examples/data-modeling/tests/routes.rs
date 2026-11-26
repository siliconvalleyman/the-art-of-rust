use data_modeling::route::{Desk, Route};

#[test]
fn routes_dispatch_every_zone_and_locker() {
    assert_eq!(Route::Collect.desk(), Desk::Counter);
    for code in 0..=u8::MAX {
        assert_eq!(Route::Locker(code).desk(), Desk::Locker(code));
        assert_eq!(
            Route::Deliver {
                zone: code,
                fragile: true
            }
            .desk(),
            Desk::Padded(code)
        );
        assert_eq!(
            Route::Deliver {
                zone: code,
                fragile: false
            }
            .desk(),
            Desk::Standard(code)
        );
    }
}
