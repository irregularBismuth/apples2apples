use super::pre_start::PreStart;
use proc_macro2::Span;
use syn::Type;

#[derive(Clone)]
pub struct ActorArgsRaw {
    pub msg: Option<Type>,
    pub state: Option<Type>,
    pub args: Option<Type>,
    pub pre_start: Option<PreStart>,
    pub span: Span,
}

impl ActorArgsRaw {
    pub fn new(span: Span) -> Self {
        Self {
            msg: None,
            state: None,
            args: None,
            pre_start: None,
            span,
        }
    }
}
#[derive(Clone)]
pub struct ValidatedActorArgs {
    pub msg: Type,
    pub state: Type,
    pub args: Type,
    pub pre_start: Option<PreStart>,
    pub span: Span,
}

impl ValidatedActorArgs {
    pub fn has_pre_start(&self) -> bool {
        self.pre_start.is_some()
    }
}
