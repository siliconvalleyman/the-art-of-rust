/// Price a fixed number of items at 125 cents each.
pub fn base_price(items: u16) -> u32 {
    u32::from(items) * 125
}

/// Subtract a discount rounded down to whole cents.
pub fn apply_discount(cents: u32) -> u32 {
    cents - cents / 10
}

pub fn quote(items: u16) -> u32 {
    apply_discount(base_price(items))
}

pub fn receipt(items: u16) -> String {
    format!("items={items}, cents={}", quote(items))
}
