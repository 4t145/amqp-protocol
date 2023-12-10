extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Lit};
#[proc_macro_derive(Types, attributes(amqp))]
pub fn derive_types(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let data = &input.data;
    let data = match data {
        Data::Struct(data) => data,
        _ => panic!("Types can only be derived for structs"),
    };

    let generics = input.generics.clone();
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let fields_name = data
        .fields
        .iter()
        .enumerate()
        .map(|(idx, f)| {
            if let Some(ident) = &f.ident {
                quote!(#ident)
            } else {
                let index = syn::Index::from(idx);
                quote!(#index)
            }
        })
        .collect::<Vec<_>>();
    let from_primitive_block = match &data.fields {
        syn::Fields::Named(named) => {
            let fields = named.named.iter().map(|f| f.ident.clone());
            quote! {
                Self {
                    #(
                        #fields: amqp_types::types::Types::from_primitive(iter.next()?.construct()?)?,
                    )*
                }
            }
        }
        syn::Fields::Unnamed(unnamed) => {
            let types = unnamed.unnamed.iter().map(|_f| quote!());
            quote!(
                Self (
                    #(
                        #types amqp_types::types::Types::from_primitive(iter.next()?.construct()?)?,
                    )*
                )
            )
        }
        syn::Fields::Unit => {
            quote!(Self)
        }
    };

    let fields_count = data.fields.len();
    let mut descriptor = quote!();
    input
        .attrs
        .iter()
        .filter_map(|attr| {
            if attr.path().is_ident("amqp") {
                let result = attr.parse_nested_meta(|nested| {
                    if nested.path.is_ident("descriptor") {
                        let value = nested.value()?;
                        let value = value.parse::<Lit>()?;
                        descriptor = match value {
                            Lit::ByteStr(s) => {
                                quote!(const DESCRIPTOR: Option<amqp_types::Descriptor> = Some(amqp_types::Descriptor::symbol(#s));)
                            }
                            Lit::Int(i) => {
                                quote!(const DESCRIPTOR: Option<amqp_types::Descriptor> = Some(amqp_types::Descriptor::numeric(#i));)
                            }
                            _ => {
                                return Err(syn::Error::new_spanned(
                                    value,
                                    "expected byte string, or integer",
                                ));
                            }
                        }
                    }
                    Ok(())
                });
                return Some(result);
            }
            None
        })
        .collect::<syn::Result<()>>()
        .expect("fail to parse amqp attributes");

    let expanded = quote! {
        impl #impl_generics amqp_types::types::Types for #name #ty_generics #where_clause {
            #descriptor
            const FORMAT_CODE: amqp_types::FormatCode = amqp_types::FormatCode::LIST32;
            fn as_data(&self) -> amqp_types::bytes::Bytes {
                use amqp_types::bytes::BufMut;
                use amqp_types::codec::enc::Encode;
                let count = #fields_count as u32;
                let mut size: u32 = 0;
                let mut data = amqp_types::bytes::BytesMut::new();
                data.put_u32(size);
                data.put_u32(count);
                #(
                    self.#fields_name.as_value().encode(&mut data);
                )*
                size = data.len() as u32 - 4;
                data[0..4].copy_from_slice(&size.to_be_bytes());
                data.into()
            }

            fn from_primitive(value: amqp_types::Primitive) -> Option<Self> {
                match value {
                    amqp_types::Primitive::List(l) => {
                        let mut iter = l.into_iter();
                        Some(#from_primitive_block)
                    },
                    _ => None,
                }
            }
        }
    };

    expanded.into()
}
