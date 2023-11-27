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
#[proc_macro_derive(Reducers)]
pub fn derive_reducers(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let parent_reducer = input.ident;

    let data = if let Data::Struct(data) = input.data {
        data
    } else {
        panic!("The Reducers derive macro as for structs (with named fields)");
    };

    let child_reducers =
        data.fields.iter().map(|field| {
            let name = &field.ident;
            quote! {
                if let Ok(action) = action.clone().try_into() {
                    self.#name.reduce_async(action, effects.scope()).await;
                }
            }
        });

    let expanded = quote! {
        impl Reducer for #parent_reducer
            where self::Action: Clone
        {
            type Action = self::Action;

            fn run_reducers(
                &mut self,
                action: Self::Action,
                effects: impl Effects<Action = Self::Action>,
            ) -> impl std::future::Future<Output = ()> {
                async move {
                    self.reduce_first(action.clone(), effects.clone()).await;
                     #(
                        #child_reducers
                    ),*
                }
            }

            type Output = ();

            fn into_inner(self) -> Self::Output { }
        }
    };

    TokenStream::from(expanded)
}
