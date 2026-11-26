enum Status {
    Waiting,
    Ready,
    Cancelled,
}

fn label(status: Status) -> &'static str {
    match status {
        Status::Waiting => "waiting",
        Status::Ready => "ready",
    }
}

fn main() {
    assert_eq!(label(Status::Cancelled), "cancelled");
}
