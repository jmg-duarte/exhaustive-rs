use std::convert::TryFrom;

use paste::paste;
use proc_macro2::{Ident, Span};
use quote::ToTokens;
use syn::parse::Parse;

macro_rules! gen_token {
    ($ident:ident) => {
        paste! {
            pub struct [<$ident:camel Token>] {
                span: ::proc_macro2::Span,
            }

            impl Parse for [<$ident:camel Token>] {
                fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
                    let ident = input.parse::<Ident>()?;
                    if ident == stringify!($ident) {
                        Ok(Self { span: ident.span() })
                    } else {
                        Err(syn::Error::new_spanned(ident, format!("expected \'{}\'", stringify!($ident))))
                    }
                }
            }

            impl ToTokens for [<$ident:camel Token>] {
                fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                    tokens.extend(proc_macro2::TokenStream::from(self.into_token_stream()))
                }
            }

            impl TryFrom<Ident> for [<$ident:camel Token>] {
                type Error = Span;

                fn try_from(value: Ident) -> Result<Self, Self::Error> {
                    if value == stringify!($ident) {
                        Ok(Self { span: value.span() })
                    } else {
                        Err(value.span())
                    }
                }
            }
        }
    };
}

gen_token!(if);
gen_token!(for);
gen_token!(in);
