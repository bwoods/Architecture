use composable_views::{Bounds, Event, Output, Size, View};
use itertools::Itertools;
use std::ops::{RangeFrom, RangeTo, RangeToInclusive};

/// # Note
/// `Table` alone can only do discrete scrolling, like the terminal does.
/// A (wrapping) `Scrolling` view will be needed for continuous scrolling.
pub struct Table<K> {
    pub(super) offset: K, // top row
}

impl<K> Table<K> {
    pub fn with_offset(offset: K) -> Self {
        Self { offset }
    }

    pub fn view<F, I, V>(&self, within: Size, mut f: F) -> impl View
    where
        K: Ord + Clone,
        F: FnMut(RangeFrom<K>) -> I,
        I: Iterator<Item = V>,
        V: View,
    {
        let mut height = 0.0;
        let visible = f(self.offset.clone()..)
            .by_ref()
            .take_while_inclusive(|view| {
                height += view.size(within).height;
                height <= within.height
            })
            .collect();

        Rows(visible)
    }

    pub fn row_up<F, I>(&mut self, mut f: F)
    where
        K: Ord + Clone,
        F: FnMut(RangeTo<K>) -> I,
        I: Iterator<Item = K> + DoubleEndedIterator,
    {
        if let Some(offset) = f(..self.offset.clone()).next_back() {
            self.offset = offset;
        }
    }

    /// # Note
    /// This will allow overscroll until there is a single row in view.
    pub fn row_down<F, I>(&mut self, mut f: F)
    where
        K: Ord + Clone,
        F: FnMut(RangeFrom<K>) -> I,
        I: Iterator<Item = K>,
    {
        // there is no `RangeFromExclusive`
        if let Some(offset) = f(self.offset.clone()..).nth(1) {
            self.offset = offset;
        }
    }

    /// # Note
    /// This will allow overscroll until there is a single row in view.
    pub fn page_down<F, I, V>(&mut self, within: Size, mut f: F)
    where
        K: Ord + Clone,
        F: FnMut(RangeFrom<K>) -> I,
        I: Iterator<Item = (K, V)>,
        V: View,
    {
        let mut height = 0.0;
        if let Some((offset, _)) = //
            f(self.offset.clone()..) //
                .by_ref()
                .find_or_last(|(_, view)| {
                    height += view.size(within).height;
                    height > within.height
                })
        {
            if offset != self.offset {
                self.offset = offset;
            } else {
                // either the current row is too tall for page_down to work, or
                // we’re at the bottom; try a row_down
                if let Some((offset, _)) = f(self.offset.clone()..).nth(1) {
                    self.offset = offset;
                }
            }
        }
    }

    pub fn page_up<F, I, V>(&mut self, within: Size, mut f: F)
    where
        K: Ord + Clone,
        F: FnMut(RangeToInclusive<K>) -> I,
        I: Iterator<Item = (K, V)> + DoubleEndedIterator,
        V: View,
    {
        let mut height = 0.0;
        if let Some((offset, _)) =
            f(..=self.offset.clone())
                .by_ref()
                .rev()
                .find_or_last(|(_, view)| {
                    height += view.size(within).height;
                    height > within.height
                })
        {
            if offset != self.offset {
                self.offset = offset;
            } else {
                // either the current row is too tall for page_up to work, or
                // we’re at the top; try a row_up
                if let Some((offset, _)) = f(..=self.offset.clone()).rev().nth(1) {
                    self.offset = offset;
                }
            }
        }
    }
}

impl<K> Default for Table<K>
where
    K: Default,
{
    fn default() -> Self {
        Self {
            offset: Default::default(),
        }
    }
}

impl<K> std::hash::Hash for Table<K>
where
    K: std::hash::Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.offset.hash(state)
    }
}

struct Rows<V>(Vec<V>); // TODO: do we replace the Vec with an Rc<trait>?

impl<V> View for Rows<V>
where
    V: View,
{
    fn size(&self, within: Size) -> Size {
        self.0.as_slice().size(within)
    }

    fn event(&self, event: Event, bounds: Bounds) {
        self.0.as_slice().event(event, bounds)
    }

    fn draw(&self, bounds: Bounds, onto: &mut impl Output) {
        self.0.as_slice().draw(bounds, onto)
    }
}
