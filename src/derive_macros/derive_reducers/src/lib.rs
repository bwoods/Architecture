#![forbid(unsafe_code)]

use proc_macro::TokenStream;

mod recusive;

#[doc = include_str!("../../README.md")]
#[proc_macro_derive(RecursiveReducer, attributes(not_a_reducer))]
pub fn derive_recursive_reducers(input: TokenStream) -> TokenStream {
    recusive::derive_macro(input)
}
