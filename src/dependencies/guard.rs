#![allow(unused_imports)]

use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::rc::Rc;

pub struct Guard<T: 'static> {
    _marker: PhantomData<*const T>, // !Send
}

thread_local! {
    static PER_THREAD: RefCell<UnhashMap<TypeId, Vec<Rc<dyn Any + 'static>>>> = Default::default();
}

impl<T: 'static> Guard<T> {
    pub(crate) fn new(value: T) -> Self {
        PER_THREAD.with_borrow_mut(|map| {
            map.entry(TypeId::of::<T>())
                .or_default()
                .push(Rc::new(value))
        });

        Self {
            _marker: PhantomData,
        }
    }

    pub(crate) fn get() -> Option<Rc<T>> {
        PER_THREAD.with_borrow(|map| {
            map.get(&TypeId::of::<T>())
                .and_then(|vec| vec.last())
                .and_then(|ptr| Rc::clone(ptr).downcast().ok())
        })
    }
}

impl<T: 'static> Drop for Guard<T> {
    fn drop(&mut self) {
        // There is no need to handle Guards being used in anything other than a strictly stack-like
        // manner as they are a private implementation-detail and are only used that way internally.
        PER_THREAD.with_borrow_mut(|map| map.get_mut(&TypeId::of::<T>()).and_then(|vec| vec.pop()));
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
    fn finish(&self) -> u64 {
        self.value
    }

    // hashing a `TypeId` just calls `write_u64` with the bottom 64-bits
    fn write(&mut self, _bytes: &[u8]) {
        unimplemented!();
    }

    fn write_u64(&mut self, value: u64) {
        debug_assert_eq!(0, self.value);
        self.value = value;
    }
}
