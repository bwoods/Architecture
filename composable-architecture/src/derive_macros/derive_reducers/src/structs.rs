use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataStruct, Ident};

pub fn derive_macro(_identifier: Ident, data: DataStruct) -> TokenStream {
    let child_reducers = data
        .fields
        .iter()
        .filter(|field| {
            field.attrs.iter().all(|attr| {
                !attr.path().is_ident("reducer")
                    || attr
                        .parse_args::<Ident>()
                        .map(|arg| arg != "ignore")
                        .unwrap_or(true)
            })
        })
        .map(|field| {
            let name = &field.ident;

            quote! {
                if let Ok(action) = action.clone().try_into() {
                    composable::Reducer::reduce(&mut self.#name, action, send.scope());
                }
            }
        });

    // let trait_impl = quote! {
    //     trait Recurse {
    //         type Action;
    //
    //         fn reduce(&mut self,action: Self::Action,send: impl composable::Effects<Action = Self::Action>);
    //     }
    //
    //     #[automatically_derived]
    //     impl Recurse for #identifier
    //         where <Self as composable::Reducer>::Action: Clone
    //     {
    //         type Action = <Self as composable::Reducer>::Action;
    //
    //         fn reduce(&mut self, action: Self::Action, send: impl composable::Effects<Action = Self::Action>) {
    //             #( #child_reducers )*
    //
    //             Self::recurse(self, action, send);
    //         }
    //     }
    // };

    let trait_impl = quote! {
        #[automatically_derived]
        impl State
            where <Self as composable::Reducer>::Action: Clone
        {
            fn recurse(&mut self, action: <Self as composable::Reducer>::Action, send: impl composable::Effects<Action = <Self as composable::Reducer>::Action>) {
                #( #child_reducers )*
            }
        }
    };

    trait_impl
}
