use super::Position;
use fastrand::Rng;
use std::collections::BTreeMap;
use std::ops::{Range, RangeBounds};

/// Text position generation strategies
///
/// > # LSEQ: an Adaptive Structure for Sequences in Distributed Collaborative Editing
/// > In this paper, we propose a new approach, called LSEQ, that belongs to the
/// > variable-size identifiers class of sequence CRDTs.
pub enum Strategy {
    #[doc(hidden)]
    BoundaryPlus(Rng, u32),
    #[doc(hidden)]
    BoundaryMinus(Rng, u32),
    #[doc(hidden)]
    Boundaries(Rng, u32, BTreeMap<u32, bool>),
}

impl Default for Strategy {
    fn default() -> Self {
        Self::boundaries(1024)
    }
}

impl Strategy {
    /// The naive strategy: always choose the next position after p in (p, q).
    pub fn boundary() -> Self {
        Strategy::BoundaryPlus(Rng::default(), 1)
    }

    /// The 1<sup>st</sup> LSEQ strategy: Choose a position close to p in (p, q).
    pub fn boundary_plus(gap: u32) -> Self {
        Strategy::BoundaryPlus(Rng::default(), gap)
    }

    /// The 2<sup>nd</sup> LSEQ strategy: Choose a position close to q in (p, q).
    pub fn boundary_minus(gap: u32) -> Self {
        Strategy::BoundaryMinus(Rng::default(), gap)
    }

    /// The optimal LSEQ strategy: Randomly choose between using
    /// boundary+ and boundary− at each level. Once a decision is
    /// made for a given depth it is always used.
    pub fn boundaries(gap: u32) -> Self {
        Strategy::Boundaries(Default::default(), gap, [(0, false)].into()) // boundary+ for level₀
    }

    /// Creates an iterator that generates positions within (p, q).
    pub(crate) fn generate(
        &mut self,
        clock: u32,
        p: (u64, &[u32]),
        q: (u64, &[u32]),
    ) -> impl Iterator<Item = Position> {
        let mut p = Position::from(p.0, p.1, 0, clock);

        std::iter::repeat_with(move || {
            p = self.generate_one(clock, p.identifier(), q);
            p.clone()
        })
    }

    /// Generates a position within (p, q).
    pub(crate) fn generate_one(
        &mut self,
        clock: u32,
        p: (u64, &[u32]),
        q: (u64, &[u32]),
    ) -> Position {
        self.next_u64(p.0, q.0, clock)
            .unwrap_or_else(|| self.next_u32s(p.0, p.1, q.1, clock))
    }

    fn next_u64(&mut self, lhs: u64, rhs: u64, clock: u32) -> Option<Position> {
        if rhs - lhs <= 1 {
            return None;
        }

        let next = self.next(lhs + 1..rhs, 0);
        let medium = Position::medium(0, clock, next, 0, 0);
        Some(medium) // we deliberately do not create a `small` here…
    }

    // FIXME: inserting between STX and [0] is broken
    fn next_u32s(&mut self, first: u64, p: &[u32], q: &[u32], clock: u32) -> Position {
        let mut p = Vec::from(p); // TODO: TinyVec?
        let mut len = 0;

        loop {
            let lhs = 1 + *(p.get(len).unwrap_or(&u32::MIN));
            let rhs = *(q.get(len).unwrap_or(&u32::MAX));

            if lhs >= rhs {
                len += 1;
                continue;
            }

            let next = self.next(lhs..rhs, len + 1); // first is level₀
            p.truncate(len);
            p.push(next);

            return Position::from(first, &p, 0, clock);
        }
    }

    /// Handles both u64 and u32 values
    fn next<T>(&mut self, range: Range<T>, depth: usize) -> T
    where
        T: Boundary,
    {
        match self {
            Self::BoundaryPlus(rng, limit) => Boundary::plus(rng, *limit, range),
            Self::BoundaryMinus(rng, limit) => Boundary::minus(rng, *limit, range),
            Self::Boundaries(rng, limit, choices) => {
                match choices.entry(depth as u32).or_insert_with(|| rng.bool()) {
                    false => Boundary::plus(rng, *limit, range),
                    true => Boundary::minus(rng, *limit, range),
                }
            }
        }
    }
}

trait Boundary: Copy + From<u32> + Ord {
    fn within(rng: &mut Rng, range: impl RangeBounds<Self>) -> Self;

    fn saturating_add(self, other: Self) -> Self;

    /// Unlike the builtin [`saturating_sub`][iu64::saturating_sub],
    /// this always saturates to zero; even for signed types
    fn saturating_sub(self, other: Self) -> Self;

    fn plus(rng: &mut Rng, limit: u32, range: Range<Self>) -> Self {
        Self::within(
            rng,
            Range {
                start: range.start,
                end: Self::min(range.end, Self::saturating_add(range.start, limit.into())),
            },
        )
    }

    fn minus(rng: &mut Rng, limit: u32, range: Range<Self>) -> Self {
        Self::within(
            rng,
            Range {
                start: Self::max(range.start, Self::saturating_sub(range.end, limit.into())),
                end: range.end,
            },
        )
    }
}

impl Boundary for u32 {
    fn within(rng: &mut Rng, range: impl RangeBounds<Self>) -> Self {
        rng.u32(range)
    }

    fn saturating_add(self, other: Self) -> Self {
        u32::saturating_add(self, other)
    }

    fn saturating_sub(self, other: Self) -> Self {
        u32::saturating_sub(self, other)
    }
}

impl Boundary for u64 {
    fn within(rng: &mut Rng, range: impl RangeBounds<Self>) -> Self {
        rng.u64(range)
    }

    fn saturating_add(self, other: Self) -> Self {
        u64::saturating_add(self, other)
    }

    fn saturating_sub(self, other: Self) -> Self {
        u64::max(0, self - other)
    }
}

#[test]
fn exhausting_level_zero() {
    use crate::text::storage::Storage;
    use crate::text::storage::pos::{ETX, Position};

    let mut storage = Storage::with_algorithm(Strategy::boundary());

    // start with a letter near the end of level₀
    let pos = Position::small(0, 0, ETX.0 - 3);
    storage.characters.insert(pos, '0');

    // now add more characters than will fit in the remaining space
    let string = "abcdef";
    storage.extend(string.chars());

    assert_eq!(storage.string(..), "0abcdef".to_owned());

    // for ch in storage.characters(..) {
    //     println!("{:x?} {:?}", ch.0.path(), ch.1);
    // }
}

/// Logoot/LSEQ have a weakness to distributed edits at the same Position
///
/// https://stackoverflow.com/q/45722742
#[test]
#[ignore]
pub fn interleaving_anomaly() {
    use crate::text::Position;
    use crate::text::storage::Storage;

    let mut storage = Storage::with_algorithm(Strategy::boundary());

    let a = Position::from(1, &[], 0, 1);
    let c = Position::from(1, &[], 1, 2);

    storage.characters.insert(a, 'a');
    storage.characters.insert(c.clone(), 'c');

    // insert 'b' between a and c…
    storage.insert_before(c.clone(), std::iter::once('b'));

    // 'c' will be second, rather than third
    assert_eq!(storage.string(..), "acb");

    // for ch in storage.characters(..) {
    //     println!("{:x?} {:?}", ch.0.path(), ch.1);
    // }
}
