use darling::{ast, FromMeta};
use quote::{quote, ToTokens};
use syn::{parse_quote, Expr};

#[derive(FromDeriveInput)]
#[darling(attributes(amqp), forward_attrs(allow, doc, cfg), supports(enum_unit))]
pub struct EnumRestrictedOpts {
    ident: syn::Ident,
    attrs: Vec<syn::Attribute>,
    generics: syn::Generics,
    data: ast::Data<ChoiceOpts, ()>,
    descriptor: Option<Expr>,
}
#[derive(FromVariant)]
#[darling(attributes(amqp), forward_attrs(allow, doc, cfg))]

pub struct ChoiceOpts {
    ident: syn::Ident
}