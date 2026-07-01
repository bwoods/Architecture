use composable_views::{Bounds, Event, Output, Size, View, include_snapshot};
use itertools::Itertools;
use std::ops::{RangeFrom, RangeTo, RangeToInclusive};

#[cfg(test)]
pub mod test;

/// Presents multiple rows of data.
///
/// # Note
/// `Table` alone can only do discrete scrolling, like a terminal.
/// A (wrapping) `Scrolling` view will be needed for continuous scrolling.
pub struct Table<K> {
    pub(super) offset: K, // top row
}

impl<K> Table<K> {
    /// If the index type of the Table (`K`) is not [`Default`], an initial
    /// offset value must be passed in. Otherwise [`Table::default()`] is
    /// usually used to construct a `Table` (that begins at its first row).
    pub fn with_offset(offset: K) -> Self {
        Self { offset }
    }

    /// Returns a `View` for the visible rows of the `Table`.
    ///
    /// `Table` neither owns the data it presents, nor has any opinions on the
    ///  appearance of those rows. The `FnMut` parameter `f` returns  an
    /// [`Iterator`] over the `View`s to display for the [range][`RangeFrom`]
    ///  of data the `Table`’s view needs to show.
    ///
    /// # Example
    /// A table showing lines of text from a Markdown file containing
    /// <u>Alice's Adventures in Wonderland (1865)</u>:
    ///
    #[doc = include_snapshot!("test/snapshots/foundations__table__test__start.snap")]
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
                height < within.height
            })
            .collect();

        Rows(visible)
    }

    /// Scrolls the `Table` forward one row.
    ///
    /// # Note
    /// Calls to `step_forward` will allow overscroll until there is a single
    /// row in view.
    ///
    /// # Example
    /// The table after a `step_forward`:
    ///
    #[doc = include_snapshot!("test/snapshots/foundations__table__test__step_forward.snap")]
    pub fn step_forward<F, I>(&mut self, mut f: F)
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

    /// Scrolls the `Table` backward one row.
    ///
    /// # Example
    /// Then performing a `step_backward` undoes the previous [`step_forward`][Self::step_forward]:
    ///
    #[doc = include_snapshot!("test/snapshots/foundations__table__test__step_backward.snap")]
    pub fn step_backward<F, I>(&mut self, mut f: F)
    where
        K: Ord + Clone,
        F: FnMut(RangeTo<K>) -> I,
        I: Iterator<Item = K> + DoubleEndedIterator,
    {
        if let Some(offset) = f(..self.offset.clone()).next_back() {
            self.offset = offset;
        }
    }

    /// Attempts to move the last row currently visible to the first row.
    ///
    /// # Note
    /// This will allow overscroll until there is a single row in `View`.
    ///
    /// # Example
    /// A `jump_forward` reveals the next batch of rows:
    ///
    #[doc = include_snapshot!("test/snapshots/foundations__table__test__jump_forward.snap")]
    pub fn jump_forward<F, I, V>(&mut self, within: Size, mut f: F)
    where
        K: Ord + Clone,
        F: FnMut(RangeFrom<K>) -> I,
        I: Iterator<Item = (K, V)>,
        V: View,
    {
        let range = self.offset.clone()..;

        let mut prev = self.offset.clone();
        let mut height = 0.0;

        let offset = f(range.clone())
            .by_ref()
            .find_map(|(offset, view)| {
                height += view.size(within).height;

                if height > within.height {
                    Some(prev.clone())
                } else {
                    prev = offset.clone();
                    None
                }
            })
            .unwrap_or(prev);

        if offset != self.offset {
            self.offset = offset;
        } else {
            // either the current row is too tall for jump_forward to work,
            // or we’re at the bottom; try a row down
            if let Some((offset, _)) = f(range).nth(1) {
                self.offset = offset;
            }
        }
    }

    /// Attempts to move the first row currently visible to the last row.
    ///
    /// # Example
    /// Then performing a `jump_backward` undoes the previous [`jump_forward`][Self::jump_forward]:
    ///
    #[doc = include_snapshot!("test/snapshots/foundations__table__test__jump_backward.snap")]
    pub fn jump_backward<F, I, V>(&mut self, within: Size, mut f: F)
    where
        K: Ord + Clone,
        F: FnMut(RangeToInclusive<K>) -> I,
        I: Iterator<Item = (K, V)> + DoubleEndedIterator,
        V: View,
    {
        let range = ..=self.offset.clone();

        let mut prev = self.offset.clone();
        let mut height = 0.0;

        let offset = f(range.clone())
            .by_ref()
            .rev() // ⬅︎
            .find_map(|(offset, view)| {
                height += view.size(within).height;

                if height > within.height {
                    Some(prev.clone())
                } else {
                    prev = offset.clone();
                    None
                }
            })
            .unwrap_or(prev);

        if offset != self.offset {
            self.offset = offset;
        } else {
            // either the current row is too tall for jump_forward to work,
            // or we’re at the bottom; try a row down
            if let Some((offset, _)) = f(range).nth(1) {
                self.offset = offset;
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
