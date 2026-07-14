use itertools::Itertools;
use std::cmp::Ordering;
use std::ops::Range;

#[derive(Default)]
#[allow(clippy::upper_case_acronyms)]
pub struct RLE(Vec<u64>);

impl RLE {
    #[allow(clippy::bool_comparison)]
    pub fn contains(&self, value: u64) -> bool {
        match self.0.binary_search(&value) {
            Ok(value) => value.is_multiple_of(2) == true,
            Err(value) => value.is_multiple_of(2) == false,
        }
    }

    pub fn range(&self, range: Range<u64>) -> impl Iterator<Item = u64> {
        self.bitwise(|a, b| a & b, range).into_iter()
    }

    pub fn into_iter(self) -> impl Iterator<Item = u64> {
        self.0
            .into_iter()
            .batching(|iter| {
                // map consecutive pairs of values into ranges
                iter.next_tuple().map(|(min, max)| min..max)
            })
            .flatten()
    }

    pub fn insert(&mut self, value: u64) {
        self.insert_range(value..value + 1);
    }

    pub fn insert_range(&mut self, range: Range<u64>) {
        self.bitwise_assign(|a, b| a | b, range);
    }

    pub fn remove(&mut self, value: u64) {
        self.remove_range(value..value + 1);
    }

    pub fn remove_range(&mut self, range: Range<u64>) {
        self.bitwise_assign(|a, b| a & !b, range);
    }

    fn bitwise_assign<LogicalOp>(&mut self, op: LogicalOp, range: Range<u64>)
    where
        LogicalOp: Fn(bool, bool) -> bool,
    {
        *self = self.bitwise(op, range);
    }

    fn bitwise<LogicalOp>(&self, op: LogicalOp, range: Range<u64>) -> Self
    where
        LogicalOp: Fn(bool, bool) -> bool,
    {
        // if a split is required (e.g. for an insert or delete) it will add two more values
        let mut vec = Vec::with_capacity(self.0.len() + 2);
        let out = |n| vec.push(n);

        let bounds = [range.start, range.end];
        bitwise(op, out, self.0.iter().copied(), bounds.into_iter());
        Self(vec)
    }
}

fn bitwise<LogicalOp, Output, Left, Right>(
    op: LogicalOp,
    mut out: Output,
    mut left: Left,
    mut right: Right,
) where
    Left: Iterator<Item = u64>,
    Right: Iterator<Item = u64>,
    LogicalOp: Fn(bool, bool) -> bool,
    Output: FnMut(u64),
{
    let mut a = false;
    let mut b = false;
    let mut current = op(a, b);

    const END: u64 = u64::MAX;
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
            new.insert(offset as u64);
        }

        new
    }
}

#[cfg(test)]
impl std::fmt::Display for RLE {
    #[allow(clippy::bool_comparison)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();
        let mut value = false;
        let mut prev = 0;

        for offset in self.0.iter().copied() {
            string.extend(std::iter::repeat_n(
                (b'0' + value as u8) as char,
                (offset - prev) as usize,
            ));

            value = !value;
            prev = offset;
        }

        f.write_str(&string)
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

    assert_eq!(format!("{}", rle), str);
    assert_eq!(rle.0, vec![3, 4, 7, 10, 12, 16]);
}

use quickcheck_macros::quickcheck;

#[quickcheck]
fn property_testing(offsets: std::collections::BTreeSet<u8>) {
    let mut set = bit_set::BitSet::new();
    set.extend(offsets.iter().map(|n| *n as usize));

    let mut rle = RLE::default();
    for value in offsets {
        rle.insert(value as u64);
    }

    // RLE iterator only returns present values
    for value in rle.range(0..256) {
        assert!(set.contains(value as usize));
    }

    // bitset iterator returns whether a value is contained
    for (value, contains) in set.into_bit_vec().iter().enumerate() {
        assert_eq!(rle.contains(value as u64), contains);
    }
}
