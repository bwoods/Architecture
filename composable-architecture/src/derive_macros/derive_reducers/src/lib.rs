#![forbid(unsafe_code)]

mod methods;
mod structs;

use proc_macro::TokenStream;
use syn::{Data, DeriveInput, parse_macro_input};

/// Derives a `Reducer` implementation that recurses through child `Reducer`s
#[proc_macro_derive(Composable, attributes(reducer))]
pub fn derive_recursive_reducers(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match input.data {
        Data::Struct(data) => structs::derive_macro(input.ident, data).into(),
        Data::Enum(_data) => unreachable!(),
        _ => panic!("untagged unions are not supported"),
    }
}

/// Derives a new trait that contains a method for each Action.
#[proc_macro_derive(Reducers, attributes(reducer))]
pub fn derive_method_reducers(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match input.data {
        Data::Enum(data) => methods::derive_macro(input.ident, data).into(),
        _ => panic!("Actions must be Enums"),
    }
}
