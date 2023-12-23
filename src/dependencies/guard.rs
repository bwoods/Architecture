#![allow(unused_imports)]

use std::any::Any;
use std::any::TypeId;
use std::cell::RefCell;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::rc::Rc;

use smallvec::SmallVec;

pub struct Guard<T: 'static> {
    _marker: PhantomData<*const T>, // !Send
}

thread_local! {
    #[allow(clippy::type_complexity)]
    static DEPS: RefCell<UnhashMap<TypeId, SmallVec<Rc<dyn Any + 'static>, 2>>> = Default::default();
}

impl<T: 'static> Guard<T> {
    pub(crate) fn new(value: T) -> Self {
        DEPS.with_borrow_mut(|deps| {
            deps.entry(TypeId::of::<T>())
                .or_default()
                .push(Rc::new(value))
        });

        Self {
            _marker: PhantomData,
        }
    }

    pub(crate) fn get() -> Option<Rc<T>> {
        DEPS.with_borrow(|deps| {
            deps.get(&TypeId::of::<T>())
                .and_then(|vec| vec.last())
                .and_then(|ptr| ptr.clone().downcast::<T>().ok())
        })
    }
}

impl<T: 'static> Drop for Guard<T> {
    fn drop(&mut self) {
        DEPS.with_borrow_mut(|deps| // remove the top of stack 
            deps.get_mut(&TypeId::of::<T>()).and_then(|vec| vec.pop()));
    }
}

/// `TypeId`s are already hashed.
pub type UnhashMap<K, V> = HashMap<K, V, BuildHasherDefault<Unhasher>>;
use std::hash::{BuildHasherDefault, Hasher};

#[derive(Default)]
pub struct Unhasher {
    value: u64,
}

// https://doc.rust-lang.org/nightly/nightly-rustc/rustc_data_structures/unhash/index.html
impl Hasher for Unhasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.value
    }

    fn write(&mut self, _bytes: &[u8]) {
        unimplemented!();
    }

    #[inline]
    fn write_u64(&mut self, value: u64) {
        debug_assert_eq!(0, self.value);
        self.value = value;
    }
}
