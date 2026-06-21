use itertools::Itertools;
use std::cmp::Ordering;
use std::ops::Range;

#[derive(Default)]
#[allow(clippy::upper_case_acronyms)]
pub struct RLE(Vec<i64>);

impl RLE {
    #[allow(clippy::bool_comparison)]
    pub fn contains(&self, value: i64) -> bool {
        self.0
            .binary_search(&(value))
            .unwrap_or_else(|next| next)
            .is_multiple_of(2)
            == false
        // every even offset is the end of a range of bits set to false;
        // e.g. self.0[0] is the end of the first (implicit) false block
    }

    pub fn range(&self, range: Range<i64>) -> impl Iterator<Item = i64> {
        self.bitwise(|a, b| a & b, range).into_iter()
    }

    pub fn into_iter(self) -> impl Iterator<Item = i64> {
        self.0
            .into_iter()
            .map(|value| value + 1) // ends → starts
            .batching(|it| {
                it.next() // map pairs of values into ranges
                    .and_then(|x| it.next().map(|y| x..y))
            })
            .flatten()
    }

    pub fn insert(&mut self, value: i64) {
        self.insert_range(value..value + 1);
    }

    pub fn insert_range(&mut self, range: Range<i64>) {
        self.bitwise_assign(|a, b| a | b, range);
    }

    pub fn remove(&mut self, value: i64) {
        self.remove_range(value..value + 1);
    }

    pub fn remove_range(&mut self, range: Range<i64>) {
        self.bitwise_assign(|a, b| a & !b, range);
    }

    fn bitwise_assign<LogicalOp>(&mut self, op: LogicalOp, range: Range<i64>)
    where
        LogicalOp: Fn(bool, bool) -> bool,
    {
        *self = self.bitwise(op, range);
    }

    fn bitwise<LogicalOp>(&self, op: LogicalOp, range: Range<i64>) -> Self
    where
        LogicalOp: Fn(bool, bool) -> bool,
    {
        // if a single split is required (e.g. for an insert or delete)
        // it will add two more values
        let mut new = Vec::with_capacity(self.0.len() + 2);
        let out = |n| new.push(n);

        // RLE indexes point to the ends of (previous) runs. not the beginnings
        let range = [range.start - 1, range.end - 1];

        bitwise(op, out, self.0.iter().copied(), range.into_iter());
        Self(new)
    }
}

fn bitwise<LogicalOp, Output, Left, Right>(
    op: LogicalOp,
    mut out: Output,
    mut left: Left,
    mut right: Right,
) where
    Left: Iterator<Item = i64>,
    Right: Iterator<Item = i64>,
    LogicalOp: Fn(bool, bool) -> bool,
    Output: FnMut(i64),
{
    let mut a = false;
    let mut b = false;
    let mut current = op(a, b);

    const END: i64 = i64::MAX;
    let mut lhs = left.next().unwrap_or(END);
    let mut rhs = right.next().unwrap_or(END);

    loop {
        match lhs.cmp(&rhs) {
            Ordering::Less => {
                a = !a;

                match op(a, b) {
                    value if value != current => {
                        current = value;
                        out(lhs);
                    }
                    _ => {}
                };

                match left.next() {
                    Some(n) => lhs = n,
                    None if rhs != END => lhs = END,
                    _ => break,
                }
            }
            Ordering::Greater => {
                b = !b;

                match op(a, b) {
                    value if value != current => {
                        current = value;
                        out(rhs);
                    }
                    _ => {}
                };

                match right.next() {
                    Some(n) => rhs = n,
                    None if lhs != END => rhs = END,
                    _ => break,
                }
            }
            Ordering::Equal => {
                a = !a;
                b = !b;

                match op(a, b) {
                    value if value != current => {
                        current = value;
                        out(lhs);
                    }
                    _ => {}
                };

                match left.next() {
                    Some(n) => lhs = n,
                    None if rhs != END => lhs = END,
                    _ => break,
                }

                match right.next() {
                    Some(n) => rhs = n,
                    None if lhs != END => rhs = END,
                    _ => break,
                }
            }
        }
    }
}

#[cfg(test)]
impl From<&str> for RLE {
    fn from(str: &str) -> Self {
        let mut new = Self::default();

        for offset in str
            .chars() //
            .enumerate()
            .filter_map(|(n, ch)| match ch {
                '0' => None,
                '1' => Some(n),
                _ => unreachable!(),
            })
        {
            new.insert(offset as i64);
        }

        new
    }
}

#[cfg(test)]
impl std::fmt::Debug for RLE {
    #[allow(clippy::bool_comparison)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();

        if self.0.is_empty() == false {
            let mut value = false;
            let mut prev = -1;

            for offset in self.0.iter().copied() {
                let ch = (b'0' + value as u8) as char;
                string.extend(std::iter::repeat_n(ch, (offset - prev) as usize));

                value = !value;
                prev = offset;
            }
        }

        f.write_str(&string.trim_end_matches('0')) // remove any trailing zeros
    }
}

/// The Optimized D-Gap coding example from
/// [_dGap compression of inverted lists_][dgap].
///
/// [dgap]: http://bitmagic.io/dgap
#[test]
fn optimized_dap_example() {
    let str = "0001000111001111";
    let rle = RLE::from(str);

    assert_eq!(format!("{:?}", rle), str);
    assert_eq!(rle.0, vec![2, 3, 6, 9, 11, 15]);
}

use quickcheck_macros::quickcheck;

#[quickcheck]
fn property_testing(offsets: std::collections::BTreeSet<u8>) {
    let mut set = bit_set::BitSet::new();
    set.extend(offsets.iter().map(|n| *n as usize));

    let mut rle = RLE::default();
    for value in offsets {
        rle.insert(value as i64);
    }

    for value in rle.range(0..256) {
        assert!(set.contains(value as usize));
    }

    for (value, present) in set.into_bit_vec().iter().enumerate() {
        assert_eq!(rle.contains(value as i64), present);
    }
}
