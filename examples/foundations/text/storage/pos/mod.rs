//! The types in this module are exported by its parent therefore any module
//! documentation placed here will not be visible in the documentation.
#![allow(dead_code)]

pub mod rle;
pub mod strategy;

use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[derive(Clone)]
#[doc = include_str!("README.md")]
pub enum Position {
    #[doc(hidden)]
    S {
        /// see [Position::source]
        source: u16,
        /// see [Position::clock]
        clock: u16,

        offset: i64,
    },
    #[doc(hidden)]
    M {
        /// see [Position::source]
        source: u16,
        /// see [Position::clock]
        clock: u16,

        upper: u16,
        array: [u32; Self::LIMIT],
    },
    #[doc(hidden)]
    L {
        /// see [Position::source]
        source: u16,
        /// see [Position::clock]
        clock: u16,

        upper: u16,
        boxed: Box<[u32]>,
    },
}

impl Position {
    const LIMIT: usize = 4;

    pub(crate) fn identifier(&self) -> (i64, &[u32]) {
        fn combine_all<'a>(upper: &'a u16, slice: &'a [u32]) -> (i64, &'a [u32]) {
            match slice {
                [lower, tail @ ..] => (((*upper as i64) << 32) | *lower as i64, tail),
                _ => unreachable!(), // the slice will ALWAYS contain (at least) lower
            }
        }

        match self {
            Position::S { offset, .. } => (*offset, &[]),
            Position::L { upper, boxed, .. } => combine_all(upper, boxed),
            Position::M { upper, array, .. } => {
                let len = array[1..] // skip “lower”
                    .iter()
                    .position(|n| *n == 0) // exclude trailing zeros
                    .unwrap_or(array.len() - 1);
                combine_all(upper, &array[..=len])
            }
        }
    }

    /// An integer representing the source of the element at this position.
    pub fn source(&self) -> u16 {
        match self {
            Position::S { source, .. } => *source,
            Position::M { source, .. } => *source,
            Position::L { source, .. } => *source,
        }
    }

    /// A [source] specific logical clock representing when an element was created.
    /// Since the clock is incremented on each edit, an insert-delete-insert at the
    /// same location will not result in the same `Position`.
    ///
    /// [source]: Position::source
    pub fn clock(&self) -> u16 {
        match self {
            Position::S { clock, .. } => *clock,
            Position::M { clock, .. } => *clock,
            Position::L { clock, .. } => *clock,
        }
    }

    /// Creates a [`Medium`][medium] or [`Large`][large] position depending on
    /// the length of the identifier.
    ///
    /// [medium]: #medium
    /// [large]: #large
    pub(crate) fn from(first: i64, rest: &[u32], source: u16, clock: u16) -> Self {
        const MIN: i64 = STX.0;
        const MAX: i64 = ETX.0;

        match first {
            MIN => Position::first(), // preserve boundary cases
            MAX => Position::last(),
            _ if rest.len() < Position::LIMIT => Position::medium(
                source,
                clock,
                first as u64, // SAFETY: MIN checked above
                #[allow(clippy::get_first)]
                *rest.get(0).unwrap_or(&0),
                *rest.get(1).unwrap_or(&0),
                *rest.get(2).unwrap_or(&0),
            ),
            _ => Position::large(
                source,
                clock,
                first as u64, // SAFETY: MIN checked above
                rest.into(),
            ),
        }
    }

    /// Converts a file offset into a [`Small`][small] position.
    ///
    /// # Errors
    /// Returns [Err], containing the requested offset, if that offset is
    /// greater than 250 TiB. It is expected that performance concerns will
    /// become a problem at that size long before this limitation does.
    ///
    /// [small]: #small
    /// [Err]: Result
    #[inline]
    pub fn from_offset(offset: usize) -> Result<Self, usize> {
        if offset < ETX.0 as usize {
            Ok(Self::small(0, 0, offset as i64))
        } else {
            Err(offset)
        }
    }

    fn small(source: u16, clock: u16, offset: i64) -> Self {
        Self::S {
            source,
            clock,
            offset,
        }
    }

    fn medium(source: u16, clock: u16, first: u64, second: u32, third: u32, fourth: u32) -> Self {
        let upper = (first >> 32) as u16;
        let lower = first as u32;
        let array = [lower, second, third, fourth];

        Self::M {
            source,
            clock,
            upper,
            array,
        }
    }

    fn large(source: u16, clock: u16, first: u64, mut rest: Vec<u32>) -> Self {
        let upper = (first >> 32) as u16;
        let lower = first as u32;
        rest.insert(0, lower);
        let boxed = rest.into_boxed_slice();

        Self::L {
            source,
            clock,
            upper,
            boxed,
        }
    }
}

/// The “start of text” position (i.e. the position before the first possible position)
pub(crate) const STX: (i64, &[u32]) = (-1, &[]);
/// The “end of text” position (i.e. the position _after_ the last position)
pub(crate) const ETX: (i64, &[u32]) = (0xFA00_0000_0001, &[]); // 250 TiB + 1

impl Position {
    /// The `Position` before the first character in the text.
    #[inline]
    pub(crate) fn first() -> Position {
        Position::small(0, 0, STX.0)
    }

    /// The `Position` after the last character in the text.
    #[inline]
    pub(crate) fn last() -> Position {
        Position::small(0, 0, ETX.0)
    }
}

impl PartialOrd for Position {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Position {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        // `clock` must be included for `BtreeMap::remove()` to work properly
        let lhs = (self.identifier(), self.source(), self.clock());
        let rhs = (other.identifier(), other.source(), other.clock());

        lhs.cmp(&rhs)
    }
}

impl PartialEq for Position {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Position {}

impl Hash for Position {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.identifier().hash(state)
    }
}

#[test]
fn size_check() {
    assert_eq!(size_of::<Position>(), 24);
}
