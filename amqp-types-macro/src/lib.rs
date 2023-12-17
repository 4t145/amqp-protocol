#[macro_use]
extern crate darling;
extern crate proc_macro;
mod composite;

use darling::FromDeriveInput;
use proc_macro2::TokenStream;
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
pub fn derive_types(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
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

    proc_macro::TokenStream::from(expanded.unwrap_or_default())
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
    Ok(expanded)
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
    let receiver = composite::CompositeOpts::from_derive_input(&input)?;
    let tokens = quote!(#receiver);
    Ok(tokens)
}
