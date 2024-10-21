extern crate proc_macro2;
extern crate syn;

use std::stringify;
use syn::{parse_macro_input, DeriveInput, Data};
use proc_macro::TokenStream;
use quote::{format_ident, quote};

#[proc_macro_attribute]
pub fn param_struct(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_input = parse_macro_input!(item as DeriveInput);

    let default_values = proc_macro2::TokenStream::from(attr).into_iter().enumerate().filter_map(|(i, x)| if i % 2 == 0 { Some(x) } else { None } );

    let struct_identifier = &item_input.ident;

    let hash_token = proc_macro2::Punct::new('#', proc_macro2::Spacing::Joint);

    let vary_ident = format_ident!("vary_{}", struct_identifier);

    match &item_input.data {
        Data::Struct(syn::DataStruct { fields, .. }) => {
            let mut all_types = proc_macro2::TokenStream::new();
            let mut value_all_fields = proc_macro2::TokenStream::new();
            let mut all_fields = proc_macro2::TokenStream::new();
            let mut default_field_values = proc_macro2::TokenStream::new();
            for (value, field) in default_values.zip(fields) {
                let ty = &field.ty;
                let ident = &field.ident;
                all_types.extend(quote! { #ty, });
                value_all_fields.extend(quote! { value.#ident, });
                all_fields.extend(quote! {#ident, });
                default_field_values.extend(quote! { #ident : #value, });
            }
            quote! {
                #item_input

                impl From<&#struct_identifier> for (#all_types) {
                    fn from(value : &#struct_identifier) -> (#all_types) {
                        (
                            #value_all_fields
                        )
                    }
                }

                impl Default for #struct_identifier {
                    fn default() -> Self {
                        #struct_identifier {
                            #default_field_values
                        }
                    }
                }

                #hash_token[macro_export]
                macro_rules! #vary_ident {
                    ( $i:ident, $s:expr, $($v:expr),+ ) => {
                        {
                            [
                                $( (std::format!($s, $v), #struct_identifier {
                                    $i : $v,
                                    ..Default::default()
                                })),+,
                            ]
                        }
                    };
                }
            }
        }
        _ => unimplemented!()
    }.into()
}