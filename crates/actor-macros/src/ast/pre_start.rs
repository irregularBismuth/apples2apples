use syn::Ident;
#[derive(Debug, Clone)]
pub enum PreStart {
    MethodIdent(Ident),
}
