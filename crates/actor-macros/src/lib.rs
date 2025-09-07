mod ast;
mod expand;
mod kw;
mod parse;
mod validate;

use expand::expand::expand;
use parse::actor_args::parse_actor_args;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{parse_macro_input, spanned::Spanned, DeriveInput};
use validate::args::validate_actor_args;

/// Generates a complete Actor trait implementation for a struct.
///
/// This attribute macro creates all the necessary boilerplate for implementing
/// the `ractor::Actor` trait, including type definitions and lifecycle methods.
///
/// # Syntax
/// ```rust,ignore
/// #[actor(msg = MessageType, state = StateType, args = ArgumentsType, pre_start = method_name)]
/// struct MyActor;
/// ```
///
/// # Required Parameters
/// - `msg` - The message type this actor handles
/// - `state` - The internal state type for this actor
///
/// # Optional Parameters
/// - `args` - Arguments type for initialization (defaults to `()`)
/// - `pre_start` - Custom initialization method name
///
/// # Examples
///
/// Basic actor with no arguments:
/// ```rust
/// # use actor_macros::actor;
/// #[derive(Debug)]
/// enum PingMsg { Ping }
///
/// #[actor(msg = PingMsg, state = ())]
/// struct PingActor;
/// ```
///
/// Actor with state and arguments:
/// ```rust
/// # use actor_macros::actor;
/// #[derive(Debug)]
/// enum CounterMsg { Increment, GetCount }
///
/// #[derive(Debug)]
/// struct CounterState { count: i32 }
///
/// #[actor(msg = CounterMsg, state = CounterState, args = i32)]
/// struct CounterActor;
/// ```
///
/// Actor with custom initialization:
/// ```rust
/// # use actor_macros::actor;
/// #[derive(Debug)]
/// enum GameMsg { Start, Stop }
///
/// #[derive(Debug)]
/// struct GameState { running: bool }
///
/// #[actor(msg = GameMsg, state = GameState, args = String, pre_start = initialize)]
/// struct GameActor;
/// ```
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

/// Generates a custom actor initialization method.
///
/// Creates an `on_start` method that gets called by the actor's `pre_start` implementation
/// when using `pre_start = method_name` in the `#[actor(...)]` attribute.
///
/// # Generated Method Signature
/// ```rust,ignore
/// pub async fn on_start(
///     &self,
///     myself: ractor::ActorRef<Self::Msg>,
///     args: Self::Arguments,
/// ) -> Result<Self::State, ractor::ActorProcessingErr>
/// ```
///
/// # Available Variables
/// - `this` - `&self` reference to the actor
/// - `myself` - `ActorRef<Self::Msg>` for sending messages to this actor
/// - `args` - `Self::Arguments` initialization arguments
///
/// # Returns
/// Must return `Ok(initial_state)` or `Err(ractor::ActorProcessingErr)`
///
/// # Examples
///
/// Basic initialization:
/// ```rust
/// # use actor_macros::{actor, actor_pre_start};
/// # use ractor::ActorProcessingErr;
/// #[derive(Debug)]
/// struct GameState { score: i32 }
///
/// #[derive(Debug)]
/// enum GameMsg { Start }
///
/// #[actor(msg = GameMsg, state = GameState, args = i32, pre_start = init)]
/// struct GameActor;
///
/// impl GameActor {
///     actor_pre_start!({
///         println!("Starting with score: {}", args);
///         Ok(GameState { score: args })
///     });
/// }
/// ```
///
/// With error handling:
/// ```rust
/// # use actor_macros::{actor, actor_pre_start};
/// # use ractor::ActorProcessingErr;
/// impl GameActor {
///     actor_pre_start!({
///         if args < 0 {
///             return Err(ActorProcessingErr::from("Score cannot be negative"));
///         }
///         Ok(GameState { score: args })
///     });
/// }
/// ```
#[proc_macro]
pub fn actor_pre_start(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let body: syn::Block = syn::parse(input).expect("expected a block: {{ ... }}");
    quote::quote! {
        pub async fn on_start(
            &self,
            myself: ractor::ActorRef<<Self as ractor::Actor>::Msg>,
            args: <Self as ractor::Actor>::Arguments,
        ) -> ::core::result::Result<
            <Self as ractor::Actor>::State,
            ractor::ActorProcessingErr
        > {
            let this = self;
            #body
        }
    }
    .into()
}

/// Generates the message handling method for an actor.
///
/// Creates a `handle_msg` method that processes incoming messages. This method
/// is called by the generated `Actor::handle` implementation.
///
/// # Generated Method Signature
/// ```rust,ignore
/// pub async fn handle_msg(
///     &self,
///     myself: ractor::ActorRef<Self::Msg>,
///     msg: Self::Msg,
///     state: &mut Self::State,
/// ) -> Result<(), ractor::ActorProcessingErr>
/// ```
///
/// # Available Variables
/// - `this` - `&self` reference to the actor
/// - `myself` - `ActorRef<Self::Msg>` for sending messages to this actor
/// - `msg` - `Self::Msg` the received message
/// - `state` - `&mut Self::State` mutable reference to actor state
///
/// # Returns
/// Must return `Ok(())` or `Err(ractor::ActorProcessingErr)`
///
/// # Examples
///
/// Basic message handling:
/// ```rust
/// # use actor_macros::{actor, actor_handle};
/// # use ractor::ActorProcessingErr;
/// #[derive(Debug)]
/// struct Counter { count: i32 }
///
/// #[derive(Debug)]
/// enum CounterMsg { Increment, Decrement }
///
/// #[actor(msg = CounterMsg, state = Counter)]
/// struct CounterActor;
///
/// impl CounterActor {
///     actor_handle!({
///         match msg {
///             CounterMsg::Increment => {
///                 state.count += 1;
///                 println!("Count: {}", state.count);
///             },
///             CounterMsg::Decrement => {
///                 state.count -= 1;
///                 println!("Count: {}", state.count);
///             },
///         }
///         Ok(())
///     });
/// }
/// ```
///
/// With error handling:
/// ```rust
/// # use actor_macros::{actor, actor_handle};
/// # use ractor::{cast, ActorProcessingErr};
/// impl CounterActor {
///     actor_handle!({
///         match msg {
///             CounterMsg::Increment => {
///                 state.count += 1;
///                 if state.count > 100 {
///                     return Err(ActorProcessingErr::from("Counter overflow"));
///                 }
///             },
///             CounterMsg::Decrement => {
///                 state.count -= 1;
///             },
///         }
///         Ok(())
///     });
/// }
/// ```
#[proc_macro]
pub fn actor_handle(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let body: syn::Block = syn::parse(input).expect("expected a block: {{ ... }}");
    quote::quote! {
        pub async fn handle_msg(
            &self,
            myself: ractor::ActorRef<<Self as ractor::Actor>::Msg>,
            msg: <Self as ractor::Actor>::Msg,
            state: &mut <Self as ractor::Actor>::State,
        ) -> ::core::result::Result<(), ractor::ActorProcessingErr> {
            let this = self;
            #body
        }
    }
    .into()
}
