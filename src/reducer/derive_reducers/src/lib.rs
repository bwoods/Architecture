use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

/// Macro used to derive `Reducer` conformance for structs containing child reducers.
///
///
///
/// # Requirements
/// - All fields in the struct must, themselves, implement `Reducer`.
/// - The parent type’s `Action` must be `Clone`.
///
/// If any of these requirement are not met the macro will issue a compilation error.
#[proc_macro_derive(RecursiveReducer, attributes(not_a_reducer))]
pub fn derive_reducers(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let parent_reducer = input.ident;

    let data = if let Data::Struct(data) = input.data {
        data
    } else {
        panic!("The Reducers derive macro is for structs (with named fields)");
    };

    #[rustfmt::skip]
    let child_reducers = data.fields.iter().filter(|field| {
            field.attrs.iter().all(|attr| !attr.path().is_ident("not_a_reducer"))
        })
        .map(|field| {
            let name = &field.ident;
            quote! {
                if let Ok(action) = action.clone().try_into() {
                    self.#name.reduce(action, effects.scope());
                }
            }
        });

    let expanded = quote! {
        impl composable::Reducer for #parent_reducer
            where self::Action: Clone
        {
            type Action = self::Action;
            type Output = ();

            fn into_inner(self) -> Self::Output { }

            fn reduce(
                &mut self,
                action: Self::Action,
                effects: impl composable::Effects<Action = Self::Action>,
            ) {
                Self::reduce(self, action.clone(), effects.clone());

                 #(
                    #child_reducers
                )*
            }
        }
    };

    TokenStream::from(expanded)
}
