//! Bounded representations and arithmetic from Chapter 2.

pub fn byte_bits(value: u8) -> [u8; 8] {
    let mut quotient = value;
    let mut bits = [0; 8];
    for bit in bits.iter_mut().rev() {
        *bit = quotient % 2;
        quotient /= 2;
    }
    bits
}

fn hex_digit(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        _ => None,
    }
}

/// Decode exactly two ASCII hex digits.
pub fn parse_hex_byte(bytes: &[u8]) -> Option<u8> {
    let [high, low] = bytes else {
        return None;
    };
    Some(16 * hex_digit(*high)? + hex_digit(*low)?)
}

// ANCHOR: add_byte
/// Return the low byte of an unsigned sum and its outgoing carry.
pub fn add_byte(a: u8, b: u8) -> (u8, bool) {
    let mut sum = 0;
    let mut carry = 0;
    for position in 0..8 {
        let a_bit = (a >> position) & 1;
        let b_bit = (b >> position) & 1;
        let total = a_bit + b_bit + carry;
        sum |= (total & 1) << position;
        carry = total >> 1;
    }
    (sum, carry != 0)
}
// ANCHOR_END: add_byte

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ByteOrder {
    Big,
    Little,
}

/// Decode a two-byte field in the stated order.
pub fn read_u16(bytes: &[u8], order: ByteOrder) -> Option<u16> {
    let [first, second] = bytes else {
        return None;
    };
    let (high, low) = match order {
        ByteOrder::Big => (*first, *second),
        ByteOrder::Little => (*second, *first),
    };
    Some(u16::from(high) * 256 + u16::from(low))
}

pub fn checked_byte_sum(a: u8, b: u8) -> Option<u8> {
    let (sum, carry) = add_byte(a, b);
    if carry {
        None
    } else {
        Some(sum)
    }
}
