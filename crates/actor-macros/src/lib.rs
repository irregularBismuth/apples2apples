mod ast;
mod expand;
mod kw;
mod parse;
mod validate;

use expand::expand::expand;
use parse::actor_args::parse_actor_args;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{DeriveInput, parse_macro_input, spanned::Spanned};
use validate::args::validate_actor_args;

#[proc_macro_attribute]
pub fn actor(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let attr_ts: TokenStream2 = attr.into();
    let out = || -> syn::Result<_> {
        let raw = parse_actor_args(input.span(), attr_ts)?;
        let val = validate_actor_args(raw)?;
        Ok(expand(&input, &val))
    }();

    match out {
        Ok(ts) => ts.into(),
        Err(e) => e.to_compile_error().into(),
    }
}
