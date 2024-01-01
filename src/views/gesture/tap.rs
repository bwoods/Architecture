use crate::views::gesture::Id;

pub struct TapGesture<V, F> {
    pub(crate) id: Id,
    pub(crate) view: V,
    pub(crate) action: std::cell::RefCell<F>,
}
