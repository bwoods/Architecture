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
        panic!("The Reducers derive macro is for structs (with named fields)");
    };

    let child_reducers = data.fields.iter().map(|field| {
        let name = &field.ident;
        quote! {
            if let Ok(action) = action.clone().try_into() {
                self.#name.reduce_async(action, effects.scope()).await;
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

            async fn reduce_async(
                &mut self,
                action: Self::Action,
                effects: impl composable::Effects<Action = Self::Action>,
            ) {
                self.reduce_async(action.clone(), effects.clone()).await;
                 #(
                    #child_reducers
                )*
            }
        }
    };

    TokenStream::from(expanded)
}
