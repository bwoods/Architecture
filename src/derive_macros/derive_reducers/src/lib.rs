#![forbid(unsafe_code)]

use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput};

mod enums;
mod structs;

/// ## Compiler Errors
///
/// The are a few common mistakes that will produce well-known compiler errors
///
///
///
/// ### the trait bound `winit::State: composable::RecursiveReducer` is not satisfied
///
/// ```sh
/// | #[derive(RecursiveReducer)]
/// |          ^^^^^^^^^^^^^^^^ the trait `composable::RecursiveReducer` is not implemented for `State`
/// |
/// = note: this error originates in the derive macro `RecursiveReducer`
/// ```
///
/// **Cause**: You haven't yet written an `impl RecursiveReducer` for the type you added `#[derive(RecursiveReducer)]` to.
///
/// <br />
///
///
/// ### conflicting implementation for `State`
///
/// ```sh
/// | #[derive(RecursiveReducer)]
/// |          ^^^^^^^^^^^^^^^^ conflicting implementation for `State`
/// ...
/// | impl Reducer for State {
/// | ---------------------- first implementation here
/// |
/// = note: this error originates in the derive macro `RecursiveReducer`
/// ```
///
/// **Cause**: You declared an `impl Reducer`, perhaps out of habit, rather than an `impl RecursiveReducer`.
///
/// <br />
///
/// ### the trait bound `…: composable::Reducer` is not satisfied
///
/// ```sh
/// | #[derive(RecursiveReducer)]
/// |          ^^^^^^^^^^^^^^^^ the trait `composable::Reducer` is not implemented for `…`
/// |
/// = help: the following other types implement trait `composable::Reducer`:
///           ⋮
/// = note: this error originates in the derive macro `RecursiveReducer`
/// ```
///
/// where `…`  is replaced with the type of one of the struct's fields in the error message.
///
/// **Cause**: … `#[not_a_reducer]`
///
/// <br />
///
/// ###  type mismatch resolving `<impl Effects<Action = Action> as Effects>::Action == Action`
///
/// ```sh
/// | #[derive(RecursiveReducer)]
/// |          ^^^^^^^^^^^^^^^^ expected `child::Action`, found `parent::Action`
/// |
/// = note: `parent::Action` and `child::Action` have similar names, but are actually distinct types
/// ```
///
/// **Cause**: … `From`
///
/// <br />
///
/// ### the trait bound `menu::Action: composable::From<winit::Action>` is not satisfied
///
/// ```sh
/// | #[derive(RecursiveReducer)]
/// |          ^^^^^^^^^^^^^^^^ the trait `composable::From<parent::Action>` is not implemented for `child::Action`
/// |
/// ```
///
/// **Cause**: … `TryInto`
///
/// - Or there is no wrapper around a child action for the `From` macro to wrap a child action with
///
/// <br />
///
#[proc_macro_derive(RecursiveReducer, attributes(reducer))]
pub fn derive_recursive_reducers(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match input.data {
        Data::Struct(data) => structs::derive_macro(input.ident, data),
        Data::Enum(data) => enums::derive_macro(input.ident, data),
        _ => panic!("untagged unions are not supported"),
    }
}
