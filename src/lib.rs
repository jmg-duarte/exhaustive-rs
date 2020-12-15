use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, Peek},
    parse_macro_input, Token,
};
// target syntax: `let squares_map = c![n*n for n in 0..100 if n % 5 != 0];`

#[proc_macro]
pub fn c(input: TokenStream) -> TokenStream {
    let comprehension = parse_macro_input!(input as RawComprehension);
    match comprehension.expand() {
        Ok(c) => {
            println!("{}", c);
            c.into()
        }
        Err(e) => e.to_compile_error().into(),
    }
}

/// The comprehesion of type `$var => $expr $fors`.
#[derive(Debug)]
struct RawComprehension {
    /// The left side of the macro
    // var: syn::Ident,
    expr: proc_macro2::TokenStream,
    for_loop: For,
    // if_stmt: std::option::Option<proc_macro2::TokenStream>,
}

impl Parse for RawComprehension {
    fn parse(mut input: syn::parse::ParseStream) -> syn::Result<Self> {
        println!("{:#?}", input);
        if input.is_empty() {
            // return syn::Result::Err(input.error("comprehension must not be empty"));
            return syn::Result::Err(syn::Error::new(
                input.span(),
                "comprehension must not be empty",
            ));
        }
        let expr = parse_until(&mut input, Token![for])?;
        println!("expr {:#?}", expr);
        let for_loop = input.parse::<For>()?;
        // println!("{:#?}", for_loop);
        // let expr = parse_until(&mut input, Token![if])?;
        syn::Result::Ok(Self {
            // var,
            expr,
            for_loop,
            // if_stmt: std::option::Option::Some(proc_macro2::TokenStream::new()),
        })
    }
}

impl RawComprehension {
    fn expand(self) -> syn::Result<proc_macro2::TokenStream> {
        let expr = self.expr;
        println!("{}", expr);
        let var = self.for_loop.var;
        let range = self.for_loop.range;
        syn::Result::Ok(quote!(
            (#range).map(|#var| {#expr})
        ))
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
    range: Range,
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
        let range = input.parse::<Range>()?;
        syn::Result::Ok(Self { var, range })
    }
}

/// The for loop Range structure.
///
/// ```rust
/// c![v in v for 1..10];
///               ^^^^^
///               | the corresponding `Range`
/// ```
struct Range {
    // consider adding a span
    // span: proc_macro2::Span,
    start: syn::LitInt,
    end: syn::LitInt,
    inclusive: bool,
}

impl ToTokens for Range {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let start = &self.start;
        let end = &self.end;
        tokens.extend(if self.inclusive {
            quote!(#start ..= #end)
        } else {
            quote!(#start .. #end)
        })
    }
}

impl Parse for Range {
    /// # TODO
    /// Consider missing limits such as:
    /// - `..10` - Missing start
    /// - `10..` - Missing end
    /// - `..` - Missing both
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let start = input.parse::<syn::LitInt>()?;
        let inclusive = if input.peek(Token![..=]) {
            input.parse::<Token![..=]>()?;
            true
        } else {
            input.parse::<Token![..]>()?;
            false
        };
        let end = input.parse::<syn::LitInt>()?;
        syn::Result::Ok(Self {
            start,
            inclusive,
            end,
        })
    }
}

impl std::fmt::Debug for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Range")
            .field(
                "start",
                &format!("LitInt {{ {} }}", self.start.base10_digits()),
            )
            .field("end", &format!("LitInt {{ {} }}", self.end.base10_digits()))
            .field("inclusive", &format!("{}", self.inclusive))
            .finish()
    }
}
