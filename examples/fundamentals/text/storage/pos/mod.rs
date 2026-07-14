//! The types in this module are exported by its parent therefore any module
//! documentation placed here will not be visible in the documentation.
#![allow(dead_code)]

use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

pub mod rle;
pub mod strategy;

#[derive(Clone)]
#[doc = include_str!("README.md")]
pub enum Position {
    #[doc(hidden)]
    S {
        /// see [Position::source]
        source: u16,
        /// see [Position::clock]
        clock: u32,

        offset: u64,
    },
    #[doc(hidden)]
    M {
        /// see [Position::source]
        source: u16,
        /// see [Position::clock]
        clock: u32,

        array: [u32; Self::LIMIT],
    },
    #[doc(hidden)]
    L {
        /// see [Position::source]
        source: u16,
        /// see [Position::clock]
        clock: u32,

        boxed: Box<[u32]>,
    },
}

impl Position {
    const LIMIT: usize = 4;

    pub(crate) fn len(&self) -> usize {
        match self {
            Position::S { .. } => 1, // special value
            Position::L { boxed, .. } => boxed.len(),
            Position::M { array, .. } => {
                array[2..] // skip upper and lower
                    .iter()
                    .position(|n| *n == 0) // exclude trailing zeros
                    .map(|n| n + 2)
                    .unwrap_or(Position::LIMIT)
            }
        }
    }

    pub(crate) fn identifier(&self) -> (u64, &[u32]) {
        fn combine(slice: &[u32]) -> (u64, &[u32]) {
            match slice {
                [upper, lower, tail @ ..] => (((*upper as u64) << 32) | *lower as u64, tail),
                _ => unreachable!(), // the slice will ALWAYS contain (at least) upper and lower
            }
        }

        match self {
            Position::S { offset, .. } => (*offset, &[]),
            Position::L { boxed, .. } => combine(boxed),
            Position::M { array, .. } => combine(&array[..self.len()]),
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
    pub fn clock(&self) -> u32 {
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
    pub(crate) fn from(first: u64, rest: &[u32], source: u16, clock: u32) -> Self {
        if rest.len() < Position::LIMIT - 1 {
            //  Note: < Position::LIMIT - 1 because level₀ is 64-bits
            Position::medium(
                source,
                clock,
                first,
                #[allow(clippy::get_first)]
                *rest.get(0).unwrap_or(&0),
                *rest.get(1).unwrap_or(&0),
            )
        } else {
            Position::large(source, clock, first, rest.into())
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
    pub(crate) fn from_offset(offset: u64) -> Result<Self, u64> {
        if offset < ETX.0 {
            Ok(Self::small(0, 1, offset)) // clock: 1 always > STX
        } else {
            Err(offset)
        }
    }

    const fn small(source: u16, clock: u32, offset: u64) -> Self {
        Self::S {
            source,
            clock,
            offset,
        }
    }

    fn medium(source: u16, clock: u32, first: u64, second: u32, third: u32) -> Self {
        let upper = (first >> 32) as u32;
        let lower = first as u32;
        let array = [upper, lower, second, third];

        Self::M {
            source,
            clock,
            array,
        }
    }

    fn large(source: u16, clock: u32, first: u64, mut rest: Vec<u32>) -> Self {
        let upper = (first >> 32) as u32;
        let lower = first as u32;

        rest.push(upper);
        rest.push(lower);
        rest.rotate_right(2);

        let boxed = rest.into_boxed_slice();

        Self::L {
            source,
            clock,
            boxed,
        }
    }
}

/// The “start of text” position (i.e. the position before the first possible position)
pub(crate) const STX: (u64, &[u32]) = (0, &[]);
/// The “end of text” position (i.e. the position _after_ the last position)
pub(crate) const ETX: (u64, &[u32]) = (0xFA00_0000_0001, &[]); // 250 TiB + 1

impl Position {
    pub(crate) const MIN: Position = Position::small(0, 0, STX.0);
    pub(crate) const MAX: Position = Position::small(0, 0, ETX.0);

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
