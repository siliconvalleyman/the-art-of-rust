#[path = "../../examples/data-modeling/src/dispatch.rs"]
mod dispatch;

fn main() {
    let draft = dispatch::Dispatch::new(17);
    let tracking = std::num::NonZeroU64::new(99).unwrap();
    let _ = draft.record_shipment(tracking);
}
