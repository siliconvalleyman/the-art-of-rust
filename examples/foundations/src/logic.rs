//! Boolean gate equations and simultaneous state updates from Chapter 3.

pub fn alarm(sensor_a: bool, sensor_b: bool, maintenance: bool) -> bool {
    (sensor_a || sensor_b) && !maintenance
}

fn nand(a: bool, b: bool) -> bool {
    !(a && b)
}

pub fn nand_selector(old: bool, new: bool, enabled: bool) -> bool {
    let not_enabled = nand(enabled, enabled);
    nand(nand(old, not_enabled), nand(new, enabled))
}

// ANCHOR: full_adder
/// Return the sum bit and the outgoing carry.
pub fn full_adder(a: bool, b: bool, carry_in: bool) -> (bool, bool) {
    let propagate = a ^ b;
    let generate = a & b;
    (propagate ^ carry_in, generate | (propagate & carry_in))
}
// ANCHOR_END: full_adder

pub fn swap_registers((a, b): (bool, bool), enabled: bool) -> (bool, bool) {
    if enabled {
        (b, a)
    } else {
        (a, b)
    }
}

pub fn zero_detect([a, b, c, d, e, f, g, h]: [bool; 8]) -> bool {
    let low = (a | b) | (c | d);
    let high = (e | f) | (g | h);
    !(low | high)
}
