#![forbid(unsafe_code)]

use proc_macro::TokenStream;

mod ouroboros;
mod recusive;

#[doc = include_str!("../../README.md")]
#[proc_macro_derive(RecursiveReducer, attributes(not_a_reducer))]
pub fn derive_recursive_reducers(input: TokenStream) -> TokenStream {
    recusive::derive_macro(input)
}

#[cfg(feature = "ouroboros")]
#[proc_macro_derive(SelfReferentialRecursiveReducer, attributes(not_a_reducer))]
/// `RecursiveReducer` support for [`ouroboros`] managed self-referential structs.
///
/// Note that the `SelfReferentialRecursiveReducer` **must** be placed above the
/// `self_referencing` macro:
///
/// ```ignore
/// #[derive(SelfReferentialRecursiveReducer)]
/// #[self_referencing]
/// pub struct State {
/// ```
/// The `SelfReferentialRecursiveReducer` macro is otherwise used exactly like the
/// [`RecursiveReducer`] macro.
///
/// [`ouroboros`]: https://docs.rs/ouroboros/latest/ouroboros/index.html
pub fn derive_recursive_reducers_for_ouroboros(input: TokenStream) -> TokenStream {
    ouroboros::derive_macro(input)
}
