use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, Peek},
    parse_macro_input, Token,
};
// target syntax: `let squares_map = c![n*n for n in 0..100 if n % 5 != 0];`

// [5/1/2021]
// TODO: rename the macro from c! to exhaust!
// TODO: support multiple fors
// TODO: support multiple ifs
// TODO: review the documentation

#[proc_macro]
pub fn c(input: TokenStream) -> TokenStream {
    let comprehension = parse_macro_input!(input as Comprehension);
    comprehension.expand().into()
}

/// The comprehesion of type `$var => $expr $fors`.
#[derive(Debug)]
struct Comprehension {
    /// The left side of the macro
    // var: syn::Ident,
    expr: proc_macro2::TokenStream,
    fors: Vec<For>,
}

impl Parse for Comprehension {
    fn parse(mut input: syn::parse::ParseStream) -> syn::Result<Self> {
        // println!("{:#?}", input);
        if input.is_empty() {
            // return syn::Result::Err(input.error("comprehension must not be empty"));
            return syn::Result::Err(syn::Error::new(
                input.span(),
                "comprehension must not be empty",
            ));
        }
        let expr = parse_until(&mut input, Token![for])?;
        // println!("expr {:#?}", expr);
        let mut fors = vec![input.parse::<For>()?];
        // println!("{:#?}", for_loop);
        // let expr = parse_until(&mut input, Token![if])?;
        while input.peek(Token![for]) {
            fors.push(input.parse::<For>()?)
        }
        syn::Result::Ok(Self {
            // var,
            expr,
            fors,
        })
    }
}

// TODO: convert this into `impl ToTokens`
impl Comprehension {
    fn expand(self) -> proc_macro2::TokenStream {
        // partially expand each for
        let partial_expansions = self.fors.iter().rev().map(|head| {
            move |inner| {
                let var = &head.var;
                let expr = &head.expr;
                if let Some(if_stmt) = &head.if_stmt {
                    quote!(#expr.filter(|#var| #if_stmt)

                        for #var in #expr {
                        if #if_stmt {
                            #inner
                        }
                    })
                } else {
                    quote!(for #var in #expr {
                        #inner
                    })
                }
            }
        });
        let expr = self.expr;
        let inner = quote!(#expr);
        partial_expansions.fold(inner, |acc, f| f(acc))
    }
}

fn parse_until<T: Peek>(
    input: &mut syn::parse::ParseStream,
    token: T,
) -> syn::Result<proc_macro2::TokenStream> {
    let mut expr = proc_macro2::TokenStream::new();
    // read until the stream is exhausted or the token is found
    while !input.is_empty() && !input.peek(token) {
        let e = input.parse::<proc_macro2::TokenTree>()?;
        // since extend_one is gated
        // https://github.com/rust-lang/rust/issues/72631
        expr.extend(std::iter::once(e));
    }
    syn::Result::Ok(expr)
}

/// The for loop representation.
/// Should probably nest the following `for` and `if`'s,
/// `syn::punctuated::Punctuated` style.
#[derive(Debug)]
struct For {
    var: syn::Ident,
    expr: syn::Expr,
    if_stmt: std::option::Option<syn::Expr>,
}

/// Implementation of `syn::Parse` for `For`.
///
/// This implementation enables the call `input.parse::<For>`.
impl Parse for For {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if let syn::Result::Err(_) = input.parse::<Token![for]>() {
            return syn::Result::Err(syn::Error::new(
                input.span(),
                "unexpected end of input, expecting `for`",
            ));
        }
        let var = input.parse::<syn::Ident>()?;
        input.parse::<Token![in]>()?;
        let expr = input.parse::<syn::Expr>()?;
        let if_stmt = if input.peek(Token![if]) {
            input.parse::<Token![if]>()?;
            std::option::Option::Some(input.parse::<syn::Expr>()?)
        } else {
            std::option::Option::None
        };
        syn::Result::Ok(Self { var, expr, if_stmt })
    }
}

struct Combine<A, B> where
A: Iterator,
B: Iterator,{
    a: A,
    b: B,
    current_a: A::Item,
}

impl<A, B> Iterator for Combine<A, B>
where
    A: Iterator,
    B: Iterator,
{
    type Item = (A::Item, B::Item);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(b_item) = self.b.next() {

        }
    }
}
