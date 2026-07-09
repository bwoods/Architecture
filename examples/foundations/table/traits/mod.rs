use composable_views::{View, include_snapshot};
use std::ops::RangeBounds;

mod btree;
mod slice;

/// The `table::Data` trait is used to supply values for [`Table`]s to display
#[doc(hidden)]
pub trait Data {
    type Key;
    type Value;

    fn range(
        &self,
        range: impl RangeBounds<Self::Key>,
    ) -> impl DoubleEndedIterator<Item = (Self::Key, &Self::Value)>;
}

/// Presents multiple rows of data
///
/// returned by [`table::State::view`][super::State::view].
///
/// # Note
/// A `Table` alone can only do discrete scrolling, like a terminal.
/// A (wrapping) `Scrolling` view will be needed for continuous scrolling.
///
/// # Example
/// A table showing lines of text from a Markdown file containing
/// <u>Alice's Adventures in Wonderland (1865)</u>:
///
#[doc = include_snapshot!("../test/snapshots/foundations__table__test__start.snap")]
#[doc(hidden)]
pub trait Table: View {
    /// Scrolls the `Table` forward one row.
    ///
    /// # Note
    /// Calls to `step_forward` will allow overscroll until there is a single
    /// row in view.
    ///
    /// # Example
    /// The table after a `step_forward`:
    ///
    #[doc = include_snapshot!("../test/snapshots/foundations__table__test__step_forward.snap")]
    fn step_forward(&mut self);
    /// Scrolls the `Table` backward one row.
    ///
    /// # Example
    /// Then performing a `step_backward` undoes the previous [`step_forward`][Self::step_forward]:
    ///
    #[doc = include_snapshot!("../test/snapshots/foundations__table__test__step_backward.snap")]
    fn step_backward(&mut self);
    /// Attempts to move the last row currently visible to the first row.
    ///
    /// # Note
    /// This will allow overscroll until there is a single row in `View`.
    ///
    /// # Example
    /// A `jump_forward` reveals the next batch of rows:
    ///
    #[doc = include_snapshot!("../test/snapshots/foundations__table__test__jump_forward.snap")]
    fn jump_forward(&mut self);
    /// Attempts to move the first row currently visible to the last row.
    ///
    /// # Example
    /// Then performing a `jump_backward` undoes the previous [`jump_forward`][Self::jump_forward]:
    ///
    #[doc = include_snapshot!("../test/snapshots/foundations__table__test__jump_backward.snap")]
    fn jump_backward(&mut self);
}
