#![forbid(unsafe_code)]

use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput};

mod enums;
mod structs;

#[doc = include_str!("../../README.md")]
#[proc_macro_derive(RecursiveReducer, attributes(not_a_reducer))]
pub fn derive_recursive_reducers(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match input.data {
        Data::Struct(data) => structs::derive_macro(input.ident, data),
        Data::Enum(data) => enums::derive_macro(input.ident, data),
        _ => panic!("untagged unions are not supported"),
    }
}
