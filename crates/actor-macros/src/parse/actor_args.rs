use crate::ast::{args::ActorArgsRaw, pre_start::PreStart};
use crate::kw;
use proc_macro2::Span;
use syn::{
    parse::{Parse, ParseStream, Parser},
    punctuated::Punctuated,
    spanned::Spanned,
    Error, Ident, Result, Token, Type,
};

enum Val {
    Ty(Type),
    Id(Ident),
}

enum ItemKey {
    Msg(kw::msg),
    State(kw::state),
    Args(kw::args),
    PreStart(kw::pre_start),
}

struct Item {
    key: ItemKey,
    val: Val,
}

impl Parse for Item {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        let key = if lookahead.peek(kw::msg) {
            ItemKey::Msg(input.parse()?)
        } else if lookahead.peek(kw::state) {
            ItemKey::State(input.parse()?)
        } else if lookahead.peek(kw::args) {
            ItemKey::Args(input.parse()?)
        } else if lookahead.peek(kw::pre_start) {
            ItemKey::PreStart(input.parse()?)
        } else {
            return Err(lookahead.error());
        };

        input.parse::<Token![=]>()?;

        let val = match &key {
            ItemKey::PreStart(_) => Val::Id(input.parse()?),
            ItemKey::Msg(_) | ItemKey::State(_) | ItemKey::Args(_) => Val::Ty(input.parse()?),
        };

        Ok(Item { key, val })
    }
}
// Helper that improves DRY
macro_rules! assign_type_or_ident {
    ($field:expr, $val:expr) => {
        match $val {
            Val::Ty(t) => $field = Some(t),
            Val::Id(id) => $field = Some(syn::parse_quote!(#id)),
        }
    };
}

pub fn parse_actor_args(attr_span: Span, ts: proc_macro2::TokenStream) -> Result<ActorArgsRaw> {
    let parser = Punctuated::<Item, Token![,]>::parse_terminated;
    let items: Punctuated<Item, Token![,]> = parser.parse2(ts)?;

    let mut out = ActorArgsRaw::new(attr_span);

    for Item { key, val } in items.into_pairs().map(|p| p.into_value()) {
        match key {
            ItemKey::Msg(_) => assign_type_or_ident!(out.msg, val),
            ItemKey::State(_) => assign_type_or_ident!(out.state, val),
            ItemKey::Args(_) => assign_type_or_ident!(out.args, val),
            ItemKey::PreStart(_) => match val {
                Val::Id(id) => out.pre_start = Some(PreStart::MethodIdent(id)),
                Val::Ty(t) => {
                    return Err(Error::new(
                        t.span(),
                        "pre_start must be a bare method ident (e.g., `pre_start = on_start`)",
                    ));
                }
            },
        }
    }

    Ok(out)
}
