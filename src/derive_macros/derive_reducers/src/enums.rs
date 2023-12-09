use derive_utils::quick_derive;
use proc_macro::TokenStream;

pub fn derive_macro(input: TokenStream) -> TokenStream {
    quick_derive! {
        input,
        // trait path
        composable::Reducer,
        // trait definition
        trait Reducer {
            type Action;
            type Output;
            fn into_inner(self) -> Self::Output;
            fn reduce(&mut self, action: Action, effects: impl composable::Effects<Action>);
        }
    }
}
