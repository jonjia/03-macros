use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

pub(crate) fn process_enum_from(input: DeriveInput) -> TokenStream {
    let name = input.ident;
    let generics = input.generics;
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
                    let field = fields.unnamed.first().expect("should have 1 field");
                    let ty = &field.ty;
                    quote! {
                        impl #generics From<#ty> for #name #generics {
                            fn from(e: #ty) -> Self {
                                #name::#variant_name(e)
                            }
                        }
                    }
                }
            }
            _ => quote! {},
        }
    });

    quote! {
        #(#from_impls)*
    }
}
