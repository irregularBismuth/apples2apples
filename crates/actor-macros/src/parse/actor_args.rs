use crate::ast::{args::ActorArgsRaw, pre_start::PreStart};
use proc_macro2::Span;
use syn::{
    Error, Ident, Result, Token, Type,
    parse::{Parse, ParseStream, Parser},
    punctuated::Punctuated,
    spanned::Spanned,
};

enum Val {
    Ty(Type),
    Id(Ident),
}

struct Item {
    key: Ident,
    val: Val,
}

impl Parse for Item {
    fn parse(input: ParseStream) -> Result<Self> {
        let key: Ident = input.parse()?;
        input.parse::<Token![=]>()?;

        // Decide what to parse based on the key name.
        let key_str = key.to_string();
        let val = match key_str.as_str() {
            "pre_start" => Val::Id(input.parse()?), // require bare ident
            "msg" | "state" | "args" => Val::Ty(input.parse()?), // accept any Type (paths, tuples, generics, ())
            other => {
                return Err(Error::new(
                    key.span(),
                    format!("unknown key `{other}` (expected: msg, state, args, pre_start)"),
                ));
            }
        };

        Ok(Item { key, val })
    }
}

pub fn parse_actor_args(attr_span: Span, ts: proc_macro2::TokenStream) -> Result<ActorArgsRaw> {
    // Use the parser combinator for Punctuated
    let parser = Punctuated::<Item, Token![,]>::parse_terminated;
    let items: Punctuated<Item, Token![,]> = parser.parse2(ts)?;

    let mut out = ActorArgsRaw::new(attr_span);

    for Item { key, val } in items.into_pairs().map(|p| p.into_value()) {
        match key.to_string().as_str() {
            "msg" => match val {
                Val::Ty(t) => out.msg = Some(t),
                Val::Id(id) => out.msg = Some(syn::parse_quote!(#id)),
            },
            "state" => match val {
                Val::Ty(t) => out.state = Some(t),
                Val::Id(id) => out.state = Some(syn::parse_quote!(#id)),
            },
            "args" => match val {
                Val::Ty(t) => out.args = Some(t),
                Val::Id(id) => out.args = Some(syn::parse_quote!(#id)),
            },
            "pre_start" => match val {
                Val::Id(id) => out.pre_start = Some(PreStart::MethodIdent(id)),
                Val::Ty(t) => {
                    return Err(Error::new(
                        t.span(),
                        "pre_start must be a bare method ident (e.g., `pre_start = on_start`)",
                    ));
                }
            },
            _ => unreachable!(),
        }
    }

    Ok(out)
}
