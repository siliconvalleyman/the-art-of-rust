#[path = "../src/dispatch.rs"]
pub mod dispatch;

#[no_mangle]
pub fn triple_static(value: u32) -> u32 {
    dispatch::via_generic(&dispatch::Scale(3), value)
}

#[no_mangle]
pub fn adjust_dynamic(adjustment: &dyn dispatch::Adjust, value: u32) -> u32 {
    dispatch::via_object(adjustment, value)
}
