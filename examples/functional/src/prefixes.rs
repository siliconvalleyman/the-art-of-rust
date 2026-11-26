/// Yield the first count values at least minimum, in input order.
pub fn qualified_prefix(
    input: &[u16],
    minimum: u16,
    count: usize,
) -> impl Iterator<Item = u16> + '_ {
    input
        .iter()
        .copied()
        .filter(move |&value| value >= minimum)
        .take(count)
}
