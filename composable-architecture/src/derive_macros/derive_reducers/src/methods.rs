use heck::ToSnakeCase;
use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::{DataEnum, Fields, Ident};

pub fn derive_macro(identifier: Ident, data: DataEnum) -> TokenStream {
    let methods = methods_decl(&identifier, &data);
    let reducer = reducer_impl(&identifier, &data);

    let expanded = quote! {
        #methods
        #reducer
    };

    expanded
}

fn methods_decl(_identifier: &Ident, data: &DataEnum) -> TokenStream {
    let methods = data.variants.iter().map(|variant| {
        let method = Ident::new(
            &variant.ident.to_string().to_snake_case(),
            Span::call_site(),
        );

        let args: Vec<_> = match &variant.fields {
            Fields::Named(fields) => fields
                .named
                .iter()
                .map(|field| {
                    let ty = &field.ty;

                    let name = Ident::new(
                        &field.ident.as_ref().unwrap().to_string().to_snake_case(),
                        Span::call_site(),
                    );

                    quote! { #name: #ty, }
                })
                .collect(),
            Fields::Unnamed(fields) => fields
                .unnamed
                .iter()
                .map(|field| {
                    let ty = &field.ty;

                    let string = ty.to_token_stream().to_string();
                    let string = string.rsplit_once(':').unwrap_or(("", &string)).1; // remove paths
                    let string = string.split_once("<").unwrap_or((string, "")).0; // remove generics
                    let name = Ident::new(&string.to_snake_case(), Span::call_site());

                    quote! { #name: #ty, }
                })
                .collect(),
            Fields::Unit => Default::default(),
        };

        quote! {
            fn #method(&mut self, #( #args )* send: impl Effects<Action = Self::Action>);
        }
    });

    let trait_definition = quote! {
        #[automatically_derived]
        trait Reducers {
            type Action;

            #( #methods )*
        }
    };

    trait_definition
}

fn reducer_impl(identifier: &Ident, data: &DataEnum) -> TokenStream {
    let match_arms = data.variants.iter().map(|variant| {
        let name = &variant.ident;
        let method = Ident::new(&name.to_string().to_snake_case(), Span::call_site());

        let args: Vec<_> = match &variant.fields {
            Fields::Named(fields) => fields
                .named
                .iter()
                .map(|field| {
                    let name = Ident::new(
                        &field.ident.as_ref().unwrap().to_string().to_snake_case(),
                        Span::call_site(),
                    );

                    quote! { #name, }
                })
                .collect(),
            Fields::Unnamed(fields) => fields
                .unnamed
                .iter()
                .map(|field| {
                    let ty = &field.ty;
                    let string = ty.to_token_stream().to_string();
                    let string = string.rsplit_once(':').unwrap_or(("", &string)).1; // remove paths
                    let string = string.split_once("<").unwrap_or((string, "")).0; // remove generics
                    let name = Ident::new(&string.to_snake_case(), Span::call_site());

                    quote! { #name, }
                })
                .collect(),
            Fields::Unit => Default::default(),
        };

        match &variant.fields {
            Fields::Named(_) => quote! {
                #identifier::#name { #( #args )* } => {
                    <Self as Reducers>::#method(self, #( #args )* send.clone())
                }
            },
            Fields::Unnamed(_) => quote! {
                #identifier::#name( #( #args )* ) => {
                    <Self as Reducers>::#method(self, #( #args )* send.clone())
                }
            },
            Fields::Unit => quote! {
                #identifier::#name => {
                    <Self as Reducers>::#method(self, #( #args )* send.clone())
                }
            },
        }
    });

    let trait_impl = quote! {
        #[automatically_derived]
        impl composable::Reducer for State
            where
                State: Reducers,
        {
            type Action = #identifier;

            fn reduce(&mut self, action: Self::Action, send: impl Effects<Action = Self::Action>) {
                #[allow(unreachable_patterns)]
                match action.clone() {
                    #( #match_arms )*
                    _ => {}
                }

                Self::recurse(self, action, send);
            }
        }
    };

    trait_impl
}
