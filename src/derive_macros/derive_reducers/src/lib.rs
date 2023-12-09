#![forbid(unsafe_code)]

use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput};

mod enums;
mod structs;

#[doc = include_str!("../../README.md")]
#[proc_macro_derive(RecursiveReducer, attributes(not_a_reducer))]
pub fn derive_recursive_reducers(input: TokenStream) -> TokenStream {
    let derived = input.clone();
    let derived = parse_macro_input!(derived as DeriveInput);

    match derived.data {
        Data::Struct(data) => structs::derive_macro(derived.ident, data),
        Data::Enum(_) => enums::derive_macro(input),
        _ => panic!("untagged unions are not supported"),
    }
}
