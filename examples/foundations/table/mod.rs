use composable_views::{Bounds, Event, Output, Size, Spacer, View};
use itertools::Itertools;
use std::cell::RefCell;
use std::ops::RangeBounds;
use traits::{Data, Table};

mod test;
pub mod traits;

/// Presents multiple rows of data.
///
/// # Note
/// `Table` alone can only do discrete scrolling, like a terminal.
/// A (wrapping) `Scrolling` view will be needed for continuous scrolling.
pub struct State<T: Data + ?Sized> {
    offset: RefCell<<T as Data>::Key>,
}

impl<T> State<T>
where
    T: Data + ?Sized,
    <T as Data>::Key: PartialOrd + Clone,
{
    pub fn view<'a, F, V>(&'a self, values: &'a T, views: F) -> impl Table + 'a
    where
        F: Fn(&<T as Data>::Value) -> V + 'a,
        V: View,
    {
        Inner {
            spacer: Spacer::default(),
            offset: &self.offset,
            values,
            views,
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
            offset: self.offset.borrow().clone().into(),
        }
    }
}

impl<T> Default for State<T>
where
    T: Data + ?Sized,
    <T as Data>::Key: Default,
{
    fn default() -> Self {
        Self {
            offset: Default::default(),
        }
    }
}

struct Inner<'a, T, F>
where
    T: Data + ?Sized,
{
    pub offset: &'a RefCell<<T as Data>::Key>,
    values: &'a T,
    spacer: Spacer,
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
        let offset = self
            .map(self.offset.borrow().clone().., |_| ())
            .nth(1) // next() would just return the current
            .map(|(offset, _)| offset);

        if let Some(offset) = offset {
            *self.offset.borrow_mut() = offset;
        }
    }

    fn step_backward(&mut self) {
        let offset = self
            .map(..self.offset.borrow().clone(), |_| ())
            .next_back()
            .map(|(offset, _)| offset);

        if let Some(offset) = offset {
            *self.offset.borrow_mut() = offset;
        }
    }

    fn jump_forward(&mut self, size: Size) {
        let mut total = 0.0;
        let flexible = Size::splat(f32::INFINITY);

        let offset = self
            .map(self.offset.borrow().clone().., |val| {
                (self.views)(val).size(flexible).height
            })
            .take_while(|(_, current)| {
                total += current;
                total < size.height
            })
            .map(|(offset, _)| offset)
            .last();

        if let Some(offset) = offset {
            *self.offset.borrow_mut() = offset;
        } else {
            // either the current row is too tall for jump_forward to work,
            // or we’re at the bottom; try step_forward
            self.step_forward()
        }
    }

    fn jump_backward(&mut self, size: Size) {
        let mut total = 0.0;
        let flexible = Size::splat(f32::INFINITY);

        let offset = self
            .map(..=self.offset.borrow().clone(), |val| {
                (self.views)(val).size(flexible).height
            })
            .rev()
            .take_while(|(_, current)| {
                total += current;
                total < size.height
            })
            .map(|(offset, _)| offset)
            .last();

        if let Some(offset) = offset {
            *self.offset.borrow_mut() = offset;
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
        let within = self.spacer.size(bounds.size());
        let mut height = 0.0;

        self.values
            .range(self.offset.borrow().clone()..)
            .map(|(_, val)| (self.views)(val))
            .take_while_inclusive(|view| {
                height += view.size(bounds.size()).height;
                height < within.height
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
