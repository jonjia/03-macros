// proc macro crate

// for enum, we'd like to generate From impls for each variant
// for struct we'd like to generate From impls for each field

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(EnumFrom)]
pub fn derive_enum_from(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    // print!("{:#?}", input);
    let name = &input.ident;
    let variants = match input.data {
        Data::Enum(ref data) => &data.variants,
        _ => panic!("EnumFrom can only be derived for enums"),
    };

    // for each variant, get the ident and fields, generate a From impl
    let from_impls = variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        match &variant.fields {
            syn::Fields::Unnamed(fields) => {
                if fields.unnamed.len() != 1 {
                    quote! {}
                } else {
                    let field = fields.unnamed.first().unwrap();
                    let ty = &field.ty;
                    quote! {
                        impl From<#ty> for #name {
                            fn from(e: #ty) -> Self {
                                #name::#variant_name(e)
                            }
                        }
                    }
                }
            }
            syn::Fields::Named(_fields) => quote! {},
            syn::Fields::Unit => quote! {},
        }
    });

    quote! {
        #(#from_impls)*
    }
    .into()
}
