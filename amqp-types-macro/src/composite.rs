use darling::{ast, FromMeta};
use quote::{quote, ToTokens};
use syn::{parse_quote, Expr};

#[derive(FromDeriveInput)]
#[darling(attributes(amqp), forward_attrs(allow, doc, cfg), supports(struct_any))]
pub struct CompositeOpts {
    ident: syn::Ident,
    attrs: Vec<syn::Attribute>,
    generics: syn::Generics,
    data: ast::Data<(), CompositeFieldOpts>,
    descriptor: Option<Expr>,
}

#[derive(Debug, FromField)]
#[darling(attributes(amqp), forward_attrs(allow, doc, cfg))]
pub struct CompositeFieldOpts {
    ident: Option<syn::Ident>,
    ty: syn::Type,
    default: Option<syn::Expr>,
}

impl ToTokens for CompositeOpts {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let CompositeOpts {
            ident,
            attrs,
            generics,
            data,
            descriptor,
        } = self;
        let (r#impl, r#type, r#where) = generics.split_for_impl();
        let descriptor = match descriptor {
            Some(code) => {
                quote! {
                    Some(amqp_types::Descriptor::numeric(#code))
                }
            }
            None => {
                quote! {
                    None
                }
            }
        };
        let fields = data.as_ref().take_struct().expect("should be struct");
        let field_try_from = fields.fields.clone().into_iter().map(|opt| {
            let CompositeFieldOpts { ident, default, ty } = opt;
            let ident = ident.as_ref().expect("should be named");
            match default {
                Some(default_value) => {
                    quote! {
                        #ident: Option::<#ty>::try_from_value(list.next().transpose()?.unwrap_or_default())?.unwrap_or(#default_value),
                    }
                },
                None => {
                    quote! {
                        #ident: <#ty>::try_from_value(list.next().transpose()?.unwrap_or_default())?,
                    }
                },
            }
        });
        let field_len = fields.fields.len();
        let field_len: Expr = parse_quote!(#field_len);
        let field_encode = fields.fields.into_iter().map(|opt| {
            let CompositeFieldOpts { ident, .. } = opt;
            let ident = ident.as_ref().expect("should be named");
            quote! {
                self.#ident.encode_default(writer)?;
            }
        });
        let try_from = quote!(
            fn try_from_value(value: amqp_types::Value<'amqp>) -> Result<Self, std::io::Error> {
                let Some(mut list) = value.construct()?.as_list() else {
                    return Err(std::io::Error::other(amqp_types::error::UNEXPECTED_TYPE))
                };
                Ok(Self {
                    #(#field_try_from)*
                })
            }
        );
        tokens.extend(quote!(
            impl #r#impl amqp_types::types::Restrict for #ident #r#type #r#where {
                type Source = Self;
                fn restrict(source: Self::Source) -> Result<Self, Self::Source> {
                    Ok(source)
                }
                fn source(self) -> Self::Source {
                    self
                }
            }
            impl #r#impl amqp_types::types::Multiple for #ident #r#type #r#where {}
            impl #r#impl amqp_types::codec::Encode for #ident #r#type #r#where {
                const DESCRIPTOR: Option<amqp_types::Descriptor<'static>> = #descriptor;
                const ENCODE_DEFAULT_FORMAT_CODE: amqp_types::FormatCode = amqp_types::FormatCode::LIST32;
                fn encode_data(self, format_code: amqp_types::FormatCode, writer: &mut amqp_types::codec::Writer) -> std::io::Result<()> {
                    debug_assert_eq!(format_code, Self::ENCODE_DEFAULT_FORMAT_CODE);
                    writer.write_items_32(move |writer: &mut amqp_types::codec::Writer| {
                        #(#field_encode)*
                        std::io::Result::Ok(#field_len)
                    })?;
                    Ok(())
                }
            }
            impl #r#impl amqp_types::types::Type<'amqp> for #ident #r#type #r#where {
                #try_from
            }
        ))
    }
}
