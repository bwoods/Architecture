use proc_macro::TokenStream;

use quote::quote;
use syn::{DataStruct, Ident};

pub fn derive_macro(identifier: Ident, data: DataStruct) -> TokenStream {
    let child_reducers = data
        .fields
        .iter()
        .filter(|field| {
            field.attrs.iter().all(|attr| {
                !attr.path().is_ident("reducer")
                    || attr
                        .parse_args::<Ident>()
                        .map(|arg| arg != "skip")
                        .unwrap_or(true)
            })
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
        #[automatically_derived]
        impl composable::Reducer for #identifier
            where <Self as RecursiveReducer>::Action: Clone
        {
            type Action = <Self as RecursiveReducer>::Action;
            type Output = Self;

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
