use proc_macro::TokenStream;

use quote::quote;
use syn::{DataEnum, Ident};

pub fn derive_macro(identifier: Ident, data: DataEnum) -> TokenStream {
    let child_reducers = data
        .variants
        .iter()
        .filter(|variant| {
            variant
                .attrs
                .iter()
                .all(|attr| !attr.path().is_ident("not_a_reducer"))
        })
        .map(|variant| {
            let name = &variant.ident;
            quote! {
                #identifier::#name(state) => {
                    if let Ok(action) = action.clone().try_into() {
                        composable::Reducer::reduce(state, action, effects.scope());
                    }
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

                #[allow(unreachable_patterns)]
                match self {
                    #( #child_reducers )*
                    _ => {}
                }
            }
        }
    };

    TokenStream::from(expanded)
}
