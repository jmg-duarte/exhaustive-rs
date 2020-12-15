use proc_macro::TokenStream;
use syn::{parse::Parse, parse_macro_input, Token};

// target syntax: `let squares_map = c![n => n*n for n in 0..100 if n % 5 != 0];`

#[proc_macro]
pub fn c(input: TokenStream) -> TokenStream {
    let output = input.clone();
    parse_macro_input!(input as Comprehension);
    output
}

#[allow(dead_code)]
struct Comprehension {
    var: syn::Ident,
    body: syn::Expr,
}

impl Parse for Comprehension {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            // return syn::Result::Err(input.error("comprehension must not be empty"));
            return syn::Result::Err(syn::Error::new(
                input.span(),
                "comprehension must not be empty",
            ));
        }
        println!("{:#?}", input);
        let var = input.parse()?;
        input.parse::<Token![=>]>()?;
        let body = input.parse()?;
        syn::Result::Ok(Self { var, body })
    }
}
