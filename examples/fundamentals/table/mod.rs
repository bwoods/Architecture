use composable_views::{Bounds, Event, Output, Size, Spacer, View};
use itertools::Itertools;
use std::cell::RefCell;
use std::ops::RangeBounds;
#[doc(inline)]
pub use traits::*;

mod test;
mod traits;

/// The `State` of a `Table` that must persist between drawing/event cycles
pub struct State<T: Data + ?Sized> {
    shared: RefCell<(Spacer, <T as Data>::Key)>,
}

impl<T> State<T>
where
    T: Data + ?Sized,
    <T as Data>::Key: PartialOrd + Clone,
{
    /// Returns a `View` for the visible rows of the table.
    ///
    /// The table neither owns the data it presents, nor has any opinions on
    /// the appearance of those rows. The `values` to be displayed are passed
    /// in, as well as the function that the table is to use to produces
    /// `views` for those `Data::Value`s
    pub fn view<'a, F, V>(&'a self, values: &'a T, views: F) -> impl Table
    where
        F: Fn(&<T as Data>::Value) -> V,
        V: View,
    {
        Inner {
            shared: &self.shared,
            spacer: Default::default(),
            values,
            views,
        }
    }

    pub fn offset(&self) -> <T as Data>::Key {
        self.shared.borrow().1.clone()
    }

    pub fn set_offset(&mut self, offset: <T as Data>::Key) {
        self.shared.borrow_mut().1 = offset;
    }
}

impl<T> Default for State<T>
where
    T: Data + ?Sized,
    <T as Data>::Key: Default,
{
    fn default() -> Self {
        Self {
            shared: Default::default(),
        }
    }
}

impl<T> Clone for State<T>
where
    T: Data + ?Sized,
    <T as Data>::Key: Clone,
{
    fn clone(&self) -> Self {
        Self {
            shared: self.shared.borrow().clone().into(),
        }
    }
}

struct Inner<'a, T, F>
where
    T: Data + ?Sized,
{
    shared: &'a RefCell<(Spacer, <T as Data>::Key)>,
    spacer: Spacer, // always acts as a flexible view
    values: &'a T,
    views: F,
}

impl<'a, T, F, V> Table for Inner<'a, T, F>
where
    T: Data + ?Sized,
    <T as Data>::Key: PartialOrd + Clone,
    F: Fn(&<T as Data>::Value) -> V,
    V: View,
{
    fn step_forward(&mut self) {
        let start = self.shared.borrow().1.clone();
        let offset = self
            .map(start.., |_| ())
            .nth(1) // next() would just return the current
            .map(|(offset, _)| offset);

        if let Some(offset) = offset {
            self.shared.borrow_mut().1 = offset;
        }
    }

    fn step_backward(&mut self) {
        let start = self.shared.borrow().1.clone();
        let offset = self
            .map(..start, |_| ())
            .next_back()
            .map(|(offset, _)| offset);

        if let Some(offset) = offset {
            self.shared.borrow_mut().1 = offset;
        }
    }

    fn jump_forward(&mut self) {
        let (visible, start) = {
            let borrow = self.shared.borrow();
            (borrow.0.get(), borrow.1.clone())
        };

        let mut height = 0.0;
        let offset = self
            .map(start.., |val| (self.views)(val).size(visible).height)
            .take_while(|(_, current)| {
                height += current;
                height < visible.height
            })
            .map(|(offset, _)| offset)
            .last();

        if let Some(offset) = offset {
            self.shared.borrow_mut().1 = offset;
        } else {
            // either the current row is too tall for jump_forward to work,
            // or we’re at the bottom; try step_forward
            self.step_forward()
        }
    }

    fn jump_backward(&mut self) {
        let (visible, start) = {
            let borrow = self.shared.borrow();
            (borrow.0.get(), borrow.1.clone())
        };

        let mut height = 0.0;
        let offset = self
            .map(..=start, |val| (self.views)(val).size(visible).height)
            .rev()
            .take_while(|(_, current)| {
                height += current;
                height < visible.height
            })
            .map(|(offset, _)| offset)
            .last();

        if let Some(offset) = offset {
            self.shared.borrow_mut().1 = offset;
        } else {
            // either the current row is too tall for jump_backward to work,
            // or we’re at the top; try step_backward
            self.step_backward()
        }
    }
}

impl<'a, T, F, V> View for Inner<'a, T, F>
where
    T: Data + ?Sized,
    <T as Data>::Key: PartialOrd + Clone,
    F: Fn(&<T as Data>::Value) -> V,
    V: View,
{
    fn size(&self, within: Size) -> Size {
        self.spacer.size(within)
    }

    fn event(&self, event: Event, bounds: Bounds) {
        self.visible(bounds).as_ref().event(event, bounds)
    }

    fn draw(&self, bounds: Bounds, onto: &mut impl Output) {
        // always cached the last size it was drawn at
        self.shared.borrow_mut().0.update(|_| bounds.size());

        self.visible(bounds).as_ref().draw(bounds, onto)
    }

    fn adjust_width(&self, width: f32) {
        self.spacer.adjust_width(width)
    }

    fn adjust_height(&self, height: f32) {
        self.spacer.adjust_height(height)
    }
}

impl<'a, T, F, V> Inner<'a, T, F>
where
    T: Data + ?Sized,
    <T as Data>::Key: PartialOrd + Clone,
    F: Fn(&<T as Data>::Value) -> V,
    V: View,
{
    fn visible(&self, bounds: Bounds) -> impl AsRef<[V]> {
        let (visible, start) = {
            let borrow = self.shared.borrow();
            (borrow.0.get(), borrow.1.clone())
        };

        let mut height = 0.0;
        self.values
            .range(start..)
            .map(|(_, val)| (self.views)(val))
            .take_while_inclusive(|view| {
                height += view.size(bounds.size()).height;
                height < visible.height
            })
            .collect_vec()
    }

    fn map<M, R>(
        &self,
        range: impl RangeBounds<<T as Data>::Key>,
        map: M,
    ) -> impl DoubleEndedIterator<Item = (<T as Data>::Key, R)>
    where
        M: Fn(&<T as Data>::Value) -> R,
    {
        self.values.range(range).map(move |(k, v)| (k, map(v)))
    }
}
