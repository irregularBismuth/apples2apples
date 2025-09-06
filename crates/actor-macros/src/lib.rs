use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote,
    punctuated::Punctuated,
    spanned::Spanned,
    DeriveInput, Ident, Result, Token, Type,
};

mod ast;
mod expand;
mod kw;
mod parse;
mod validate;
struct KV {
    key: Ident,
    _eq: Token![=],
    ty: Type,
}

impl Parse for KV {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            key: input.parse()?,
            _eq: input.parse()?,
            ty: input.parse()?,
        })
    }
}

struct ActorArgs {
    kvs: Punctuated<KV, Token![,]>,
}
impl Parse for ActorArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            kvs: Punctuated::parse_terminated(input)?,
        })
    }
}
///   #[actor(msg=MyMsg, state=MyState, args=MyArgs, pre_start=init_fn)]
///   struct MyActor;
#[proc_macro_attribute]
pub fn actor(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as ActorArgs);
    let input = parse_macro_input!(item as DeriveInput);

    let mut msg: Option<Type> = None;
    let mut state: Option<Type> = None;
    let mut args_ty: Option<Type> = None;
    let mut pre_start_fn: Option<Ident> = None;

    for kv in args.kvs {
        match kv.key.to_string().as_str() {
            "msg" => msg = Some(kv.ty),
            "state" => state = Some(kv.ty),
            "args" => args_ty = Some(kv.ty),
            "pre_start" => {
                // Only allow a bare ident for pre_start
                if let Type::Path(tp) = kv.ty.clone() {
                    if let Some(seg) = tp.path.segments.last() {
                        pre_start_fn = Some(seg.ident.clone());
                    } else {
                        return syn::Error::new(kv.ty.span(), "invalid pre_start target")
                            .to_compile_error()
                            .into();
                    }
                } else {
                    return syn::Error::new(kv.ty.span(), "pre_start must be an ident")
                        .to_compile_error()
                        .into();
                }
            }
            _ => {
                return syn::Error::new(
                    kv.key.span(),
                    "expected one of: msg, state, args, pre_start",
                )
                .to_compile_error()
                .into();
            }
        }
    }

    let msg = require_field(msg, input.span(), "missing `msg=...`").unwrap();
    let state = require_field(state, input.span(), "missing `state=...`").unwrap();

    let args_ty: Type = args_ty.unwrap_or_else(|| parse_quote! { () });

    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let pre_start_body = if let Some(f) = pre_start_fn {
        quote!( self.#f(myself, args).await )
    } else {
        // Defaults to Into<State> from Arguments
        quote!(::core::result::Result::Ok(args.into()))
    };

    let expanded = quote! {
        #input

        impl #impl_generics ractor::Actor for #name #ty_generics #where_clause {
            type Msg = #msg;
            type State = #state;
            type Arguments = #args_ty;

            async fn pre_start(
                &self,
                myself: ractor::ActorRef<Self::Msg>,
                args: Self::Arguments,
            ) -> ::core::result::Result<Self::State, ractor::ActorProcessingErr> {
                #pre_start_body
            }

            async fn handle(
                &self,
                myself: ractor::ActorRef<Self::Msg>,
                msg: Self::Msg,
                state: &mut Self::State,
            ) -> ::core::result::Result<(), ractor::ActorProcessingErr> {
                self.handle_msg(myself, msg, state).await
            }
        }
    };

    expanded.into()
}
