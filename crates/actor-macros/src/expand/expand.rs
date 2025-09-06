use crate::ast::{
    args::{ActorArgsRaw, ValidatedActorArgs},
    pre_start::PreStart,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::DeriveInput;
use syn::Type;

fn is_unit(ty: &Type) -> bool {
    matches!(ty, Type::Tuple(t) if t.elems.is_empty())
}

pub fn expand(input: &DeriveInput, v: &ValidatedActorArgs) -> proc_macro2::TokenStream {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let msg = &v.msg;
    let state = &v.state;
    let args_ty = &v.args;

    // pre_start logic:
    let pre_start_body = match &v.pre_start {
        Some(PreStart::MethodIdent(f)) => {
            quote! { self.#f(myself, args).await }
        }
        None => {
            if is_unit(args_ty) {
                // Arguments = () → require State: Default
                quote! { ::core::result::Result::Ok(::core::default::Default::default()) }
            } else {
                // General case → require State: From<Arguments>
                quote! { ::core::result::Result::Ok(<Self::State as ::core::convert::From<Self::Arguments>>::from(args)) }
            }
        }
    };

    quote! {
        #input

        #[ractor::async_trait]
        impl #impl_generics ractor::Actor for #name #ty_generics #where_clause {
            type Msg = #msg;
            type State = #state;
            type Arguments = #args_ty;

            async fn pre_start(
                &self,
                myself: ractor::ActorRef<Self::Msg>,
                args: Self::Arguments
            ) -> ::core::result::Result<Self::State, ractor::ActorProcessingErr> {
                #pre_start_body
            }

            async fn handle(
                &self,
                myself: ractor::ActorRef<Self::Msg>,
                msg: Self::Msg,
                state: &mut Self::State
            ) -> ::core::result::Result<(), ractor::ActorProcessingErr> {
                self.handle_msg(myself, msg, state).await
            }
        }
    }
}
