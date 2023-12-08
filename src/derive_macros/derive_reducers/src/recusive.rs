use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

pub fn derive_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let parent_reducer = input.ident;

    let data = if let Data::Struct(data) = input.data {
        data
    } else {
        panic!("No struct fields found");
    };

    let child_reducers = data
        .fields
        .iter()
        .filter(|field| {
            field.attrs.iter().all(|attr| {
                // reduce(none)
                !attr.path().is_ident("not_a_reducer")
            })
        })
        .map(|field| {
            let name = &field.ident;
            // reduce(with_getter)
            quote! {
                if let Ok(action) = action.clone().try_into() {
                    composable::Reducer::reduce(&mut self.#name, action, effects.scope());
                }
            }
        });

    let expanded = quote! {
        #[automatically_derived]
        impl composable::Reducer for #parent_reducer
            where self::Action: Clone
        {
            type Action = <Self as RecursiveReducer>::Action;
            type Output = ();

            fn into_inner(self) -> Self::Output { }

            fn reduce(
                &mut self,
                action: Self::Action,
                effects: impl composable::Effects<Self::Action>,
            ) {
                <Self as RecursiveReducer>::reduce(self, action.clone(), effects.clone());
                #( #child_reducers )*
            }
        }
    };

    TokenStream::from(expanded)
}
