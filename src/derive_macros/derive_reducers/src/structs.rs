use proc_macro::TokenStream;

use quote::quote;
use syn::{DataStruct, Ident};

pub fn derive_macro(identifier: Ident, data: DataStruct) -> TokenStream {
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
        impl composable::Reducer for #identifier
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
