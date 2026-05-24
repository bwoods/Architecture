#![allow(clippy::unreadable_literal)]

use quote::quote;
use syn::parse::Parser;

#[proc_macro]
/// Produces a unique id (at compile-time) for every call-site in the source code
/// ```
/// let a = ui_id::ui_id!();
/// let b = ui_id::ui_id!();
/// assert_ne!(a, b);
/// ```
///
/// More complex cases are handled by passing in optional runtime values to distinguish ids that are logically different but produced by the same location in the source code. Loops, for example, can be handled by passing in the loop index.
/// ```
/// use itertools::Itertools;
/// (0..1000)
///     .map(|n: usize| { return ui_id::ui_id!(n) })
///     .collect::<Vec<_>>()
///     .into_iter()
///     .combinations(2)
///     .into_iter()
///     .for_each(|pair| assert_ne!(pair[0], pair[1]));
/// ```
pub fn ui_id(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let bytes = fastrand::u128(..);

    let exprs = syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated
        .parse(tokens)
        .expect("ui_id arguments could not be parsed.")
        .into_iter();

    let tokens = quote! {
        {
            let mut hash = #bytes;

            // If runtime parameters where passed into the macro, perform a 128-bit FNV-1a
            // mix-step to combine them with the current `ui_id` to generate a new one.
            let prime = 0x0000000001000000000000000000013Bu128;
            #( hash = (hash ^ u128::try_from(#exprs).unwrap()).wrapping_mul(prime); )*

            unsafe { std::num::NonZeroU128::new_unchecked(hash | 0x1u128) }
        }
    };

    tokens.into()
}
