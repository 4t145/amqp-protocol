extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, token::Colon, Data, DeriveInput, Expr, Field, Fields, Lit};
mod consts;
use consts::AMQP_DOMAIN;

enum Descriptor {
    Symbol(syn::LitByteStr),
    Numeric(syn::LitInt, syn::LitInt),
}

impl syn::parse::Parse for Descriptor {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(syn::LitByteStr) {
            let s = input.parse::<syn::LitByteStr>()?;
            Ok(Self::Symbol(s))
        } else if lookahead.peek(syn::LitInt) {
            let i = input.parse::<syn::LitInt>()?;
            let _colon = input.parse::<syn::Token![:]>()?;
            let j = input.parse::<syn::LitInt>()?;
            Ok(Self::Numeric(i, j))
        } else {
            Err(lookahead.error())
        }
    }
}

struct AmqpStructAttr {
    pub descriptor: Option<Descriptor>,
}

struct AmqpEnumAttr {
    pub descriptor: Option<Descriptor>,
    pub source: Option<syn::Type>,
}
#[derive(Default)]
struct AmqpNewTypeAttr {
    pub descriptor: Option<Descriptor>,
    pub validation: Option<syn::Expr>,
}

//
///
///
///
///
/// ```rust
/// #[derive(Type)]
/// #[amqp(restrict(source = u8))]
/// pub enum MyEnum {
///     #[amqp(choice = 0x12)]
///     Variant1,
///     #[amqp(choice = 0x13)]
///     Variant2,
///     #[amqp(choice = 0x14)]
///     Variant3,
/// }
///
/// ```
///
/// ```rust
/// #[derive(Type)]
/// #[amqp(restrict(source = u64, validation = |x| x!=0))]
/// pub struct MyNewType(u64);
/// ```
///
///
///
///

#[proc_macro_derive(Type, attributes(amqp))]
pub fn derive_types(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let input_raw = input.clone();
    let data = input.data;
    let expanded = match data {
        Data::Struct(s) => match s.fields {
            Fields::Named(_) => derive_types_for_struct(input_raw),
            Fields::Unnamed(_) => derive_types_for_new_type(input_raw),
            Fields::Unit => panic!("Type can only be derived for structs with named fields"),
        },
        Data::Enum(_) => derive_types_for_enum(input_raw),
        Data::Union(_) => panic!("Type can only be derived for structs and enums"),
    };

    expanded.expect("failed to expand")
}

fn derive_types_for_new_type(input: DeriveInput) -> syn::Result<TokenStream> {
    let name = input.ident;
    let data = input.data;
    let data = match data {
        Data::Struct(data) => data,
        _ => panic!("Type can only be derived for structs"),
    };
    let field = match data.fields {
        Fields::Unnamed(unnamed) => {
            if unnamed.unnamed.len() != 1 {
                panic!("Type can only be derived for newtypes with one field");
            }
            unnamed.unnamed.into_iter().next().unwrap()
        }
        _ => panic!("Type can only be derived for newtypes with one field"),
    };
    let field_ty = field.ty;
    let new_type_attr = input
        .attrs
        .iter()
        .find(|a| a.path().is_ident(AMQP_DOMAIN))
        .map(|a| {
            let mut descriptor = None;
            let mut validation = None;
            a.parse_nested_meta(|nested| {
                if nested.path.is_ident("descriptor") {
                    let value = nested.value()?;
                    let value = value.parse::<Descriptor>()?;
                    descriptor.replace(value);
                } else if nested.path.is_ident("restrict") {
                    // retrieve the source type
                    nested.parse_nested_meta(|nested| {
                        if nested.path.is_ident("validation") {
                            let value = nested.value()?;
                            validation = Some(value.parse::<syn::Expr>()?);
                        }
                        Ok(())
                    })?;
                }
                Ok(())
            })
            .map(|_| AmqpNewTypeAttr {
                descriptor,
                validation,
            })
        })
        .transpose()?
        .unwrap_or_default();
    let validation = new_type_attr.validation.unwrap_or_else(|| {
        syn::parse_quote! {
            |_| true
        }
    });
    let descriptor_block = match new_type_attr.descriptor {
        Some(Descriptor::Symbol(s)) => {
            quote!(const DESCRIPTOR: Option<amqp_types::Descriptor> = Some(amqp_types::Descriptor::symbol(#s));)
        }
        Some(Descriptor::Numeric(i, j)) => {
            quote!(const DESCRIPTOR: Option<amqp_types::Descriptor> = Some(amqp_types::Descriptor::numeric((((#i as u64) << 32) | (#j as u64))));)
        }
        None => quote!(
            const DESCRIPTOR: Option<amqp_types::Descriptor> = None;
        ),
    };

    let expanded = quote! {
        impl amqp_types::types::Type for #name {
            #descriptor_block
            type Source = #field_ty;
            const FORMAT_CODE: amqp_types::FormatCode = Self::Source::FORMAT_CODE;
            fn as_data(&self) -> amqp_types::bytes::Bytes {
                self.unrestrict().as_data()
            }

            fn from_primitive(value: amqp_types::Primitive) -> Option<Self::Source> {
                Self::Source::from_primitive(value)
            }

            fn restrict(source: Self::Source) -> Option<Self> {
                if (#validation)(&source) {
                    Some(Self(source))
                } else {
                    None
                }
            }

            fn unrestrict(&self) -> &Self::Source {
                &self.0
            }
        }
    };
    Ok(expanded.into())
}

fn derive_types_for_enum(input: DeriveInput) -> syn::Result<TokenStream> {
    let name = input.ident;
    let data = input.data;
    let data = match data {
        Data::Enum(data) => data,
        _ => panic!("Type can only be derived for enums"),
    };
    let descriptor = input
        .attrs
        .iter()
        .find(|a| a.path().is_ident(AMQP_DOMAIN))
        .map(|a| {
            let mut descriptor = None;
            let mut source = None;
            a.parse_nested_meta(|nested| {
                if nested.path.is_ident("descriptor") {
                    let value = nested.value()?;
                    let value = value.parse::<Descriptor>()?;
                    descriptor.replace(value);
                } else if nested.path.is_ident("restrict") {
                    // retrieve the source type
                    nested.parse_nested_meta(|nested| {
                        if nested.path.is_ident("source") {
                            let value = nested.value()?;
                            source = Some(value.parse::<syn::Type>()?);
                        }
                        Ok(())
                    })?;
                }
                Ok(())
            })
            .map(|_| AmqpEnumAttr { descriptor, source })
        })
        .transpose()?
        .ok_or(syn::Error::new_spanned(
            "",
            "expected `amqp(descriptor = <descriptor>)`",
        ))?;

    let variants = data
        .variants
        .iter()
        .map(|v| {
            let choice = v
                .attrs
                .iter()
                .find_map(|attr| {
                    if attr.path().is_ident(AMQP_DOMAIN) {
                        let mut expr = None;
                        let result = attr.parse_nested_meta(|nested| {
                            if nested.path.is_ident("choice") {
                                let value = nested.value()?;
                                let value = value.parse::<Expr>()?;
                                expr.replace(value);
                            }
                            Ok(())
                        });
                        return expr;
                    }
                    None
                })
                .ok_or(syn::Error::new_spanned(
                    v,
                    "expected `amqp(choice = <expr>)`",
                ))?;
            Ok((v.ident.clone(), choice))
        })
        .collect::<syn::Result<Vec<_>>>()?;

    let descriptor_block = match descriptor.descriptor {
        Some(Descriptor::Symbol(s)) => {
            quote!(const DESCRIPTOR: Option<amqp_types::Descriptor> = Some(amqp_types::Descriptor::symbol(#s));)
        }
        Some(Descriptor::Numeric(i, j)) => {
            quote!(const DESCRIPTOR: Option<amqp_types::Descriptor> = Some(amqp_types::Descriptor::numeric((((#i as u64) << 32) | (#j as u64))));)
        }
        None => quote!(
            const DESCRIPTOR: Option<amqp_types::Descriptor> = None;
        ),
    };
    let source = descriptor.source.unwrap_or_else(|| {
        syn::parse_quote! {
            u8
        }
    });
    let restrict_match = variants.iter().map(|(ident, choice)| {
        quote! {
            if #choice == source {
                return Some(#name::#ident)
            }
        }
    });
    let unrestrict_match = variants.iter().map(|(ident, choice)| {
        quote! {
            &#name::#ident => {
                static choice: #source = #choice;
                return &choice
            },
        }
    });

    let from_primitive_block = quote! {
        Self::Source::from_primitive(value)
    };

    let expanded = quote! {
        impl amqp_types::types::Type for #name {
            #descriptor_block
            type Source = #source;
            const FORMAT_CODE: amqp_types::FormatCode = Self::Source::FORMAT_CODE;
            fn as_data(&self) -> amqp_types::bytes::Bytes {
                self.unrestrict().as_data()
            }

            fn constructor(&self) -> amqp_types::Constructor {
                self.unrestrict().constructor()
            }

            fn from_primitive(value: amqp_types::Primitive) -> Option<Self::Source> {
                #from_primitive_block
            }

            fn restrict(source: Self::Source) -> Option<Self> {
                #(
                    #restrict_match
                )*
                return None
            }

            fn unrestrict(&self) -> &Self::Source {
                match self {
                    #(
                        #unrestrict_match
                    )*
                }
            }
        }
    };

    Ok(expanded.into())
}

fn derive_types_for_struct(input: DeriveInput) -> syn::Result<TokenStream> {
    let name = input.ident;
    let data = input.data;
    let data = match data {
        Data::Struct(data) => data,
        _ => panic!("Type can only be derived for structs"),
    };
    let named_fields = match data.fields {
        Fields::Named(named) => named,
        _ => panic!("Type can only be derived for structs with named fields"),
    };
    let fields = named_fields.named.iter().map(|f| f.ident.clone());
    let fields_default = named_fields.named.iter().map(|f| {
        let mut default = None;
        for attr in &f.attrs {
            if !attr.path().is_ident("amqp") {
                continue;
            }
            attr.parse_nested_meta(|meta| {
                if !meta.path.is_ident("default") {
                    return Ok(())
                }
                let value = meta.value()?.parse::<Expr>()?;
                default.replace(value);
                Ok(())
            })?;
        }
        Ok((f.ident.clone().expect("named struct field should have a ident"), f.ty.clone(), default))
    }).collect::<syn::Result<Vec<_>>>()?;

    let from_primitive_items = fields_default.into_iter().map(|(ident, ty, default)| {
        if let Some(default) = default {
            quote!(#ident: Option::<#ty>::from_value(iter.next()?)?.unwrap_or(#default),)
        } else {
            quote!(#ident: amqp_types::types::Type::from_value(iter.next()?)?,)
        }
    });
    let len = named_fields.named.len();
    let count: syn::Expr = syn::parse_quote!(#len);
    let as_data_quote = quote! {
        use amqp_types::bytes::BufMut;
        use amqp_types::codec::enc::Encode;
        let mut size: u32 = 0;
        let mut data = amqp_types::bytes::BytesMut::new();
        data.put_u32(size);
        data.put_u32(#count as u32);
        #(
            self.#fields.as_value().encode(&mut data);
        )*
        size = data.len() as u32 - 4;
        data[0..4].copy_from_slice(&size.to_be_bytes());
        data.into()
    };
    let from_primitive_block = quote! {
        match value {
            amqp_types::Primitive::List(l) => {
                let mut iter = l.into_iter();
                Some(Self {
                    #(#from_primitive_items)*
                })
            },
            _ => None,
        }
    };

    let descriptor = input
        .attrs
        .iter()
        .find(|a| a.path().is_ident(AMQP_DOMAIN))
        .map(|a| {
            let mut descriptor = None;
            a.parse_nested_meta(|nested| {
                if nested.path.is_ident("descriptor") {
                    let value = nested.value()?;
                    let value = value.parse::<Descriptor>()?;
                    descriptor.replace(value);
                }
                Ok(())
            })
            .map(|_| descriptor)
        })
        .transpose()?
        .flatten();

    let descriptor_block = match descriptor {
        Some(Descriptor::Symbol(s)) => {
            quote!(const DESCRIPTOR: Option<amqp_types::Descriptor> = Some(amqp_types::Descriptor::symbol(#s));)
        }
        Some(Descriptor::Numeric(i, j)) => {
            quote!(const DESCRIPTOR: Option<amqp_types::Descriptor> = Some(amqp_types::Descriptor::numeric((((#i as u64) << 32) | (#j as u64))));)
        }
        None => quote!(
            const DESCRIPTOR: Option<amqp_types::Descriptor> = None;
        ),
    };

    let expanded = quote! {
        impl amqp_types::types::Type for #name {
            #descriptor_block
            amqp_types::no_restrict!{}
            const FORMAT_CODE: amqp_types::FormatCode = amqp_types::FormatCode::LIST32;
            fn as_data(&self) -> amqp_types::bytes::Bytes {
                #as_data_quote
            }

            fn from_primitive(value: amqp_types::Primitive) -> Option<Self> {
                #from_primitive_block
            }
        }
    };

    Ok(expanded.into())
}
