#![forbid(unsafe_code)]

use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[doc = include_str!("../../README.md")]
#[proc_macro_derive(RecursiveReducer, attributes(not_a_reducer))]
pub fn derive_reducers(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let parent_reducer = input.ident;

    let data = if let Data::Struct(data) = input.data {
        data
    } else {
        panic!("The RecursiveReducer derive macro is for structs (with named fields)");
    };

    #[rustfmt::skip]
    let child_reducers = data.fields.iter().filter(|field| {
            field.attrs.iter().all(|attr| !attr.path().is_ident("not_a_reducer"))
        })
        .map(|field| {
            let name = &field.ident;
            quote! {
                if let Ok(action) = action.clone().try_into() {
                    composable::Reducer::reduce(&mut self.#name, action, effects.scope());
                }
            }
        });

    let expanded = quote! {
        impl composable::Reducer for #parent_reducer
            where self::Action: Clone
        {
            type Action = <Self as RecursiveReducer>::Action;
            type Output = ();

            fn into_inner(self) -> Self::Output { }

            fn reduce(
                &mut self,
                action: Self::Action,
                effects: impl composable::Effects<Action = Self::Action>,
            ) {
                <Self as RecursiveReducer>::reduce(self, action.clone(), effects.clone());

                 #(
                    #child_reducers
                )*
            }
        }
    };

    TokenStream::from(expanded)
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
pub fn derive_reducers_for_ouroboros(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let parent_reducer = input.ident;

    let data = if let Data::Struct(data) = input.data {
        data
    } else {
        panic!(
            "The SelfReferentialRecursiveReducer derive macro is for structs (with named fields)"
        );
    };

    #[rustfmt::skip]
    let child_reducers = data.fields.iter().filter(|field| {
            field.attrs.iter().all(|attr| !attr.path().is_ident("not_a_reducer"))
        })
        .map(|field| {
            let method = syn::Ident::new(
                &format!("with_{}_mut", field.ident.as_ref().unwrap()),
                proc_macro2::Span::call_site(),
            );

            quote! {
                if let Ok(action) = action.clone().try_into() {
                    self.#method(|state| {
                        composable::Reducer::reduce(state, action, effects.scope());
                    })
                }
            }
        });

    let expanded = quote! {
        impl composable::Reducer for #parent_reducer
            where self::Action: Clone
        {
            type Action = <Self as RecursiveReducer>::Action;
            type Output = ();

            fn into_inner(self) -> Self::Output { }

            fn reduce(
                &mut self,
                action: Self::Action,
                effects: impl composable::Effects<Action = Self::Action>,
            ) {
                <Self as RecursiveReducer>::reduce(self, action.clone(), effects.clone());
                 #(
                    #child_reducers
                )*
            }
        }
    };

    TokenStream::from(expanded)
}
