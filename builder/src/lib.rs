use proc_macro::TokenStream;
use quote;
use syn::{parse_macro_input, spanned::Spanned};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let st = parse_macro_input!(input as syn::DeriveInput);
    match do_expand(&st) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.into_compile_error().into(),
    }
}

type StructFields = syn::punctuated::Punctuated<syn::Field, syn::Token!(,)>;

fn get_field_from_derive_input(st: &syn::DeriveInput) -> syn::Result<&StructFields> {
    if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = st.data
    {
        return Ok(named);
    }

    Err(syn::Error::new_spanned(st, "error"))
}

fn generate_builder_struct_fields_def(
    st: &syn::DeriveInput,
) -> syn::Result<proc_macro2::TokenStream> {
    let fields = get_field_from_derive_input(&st)?;

    let indent: Vec<_> = fields.iter().map(|f| &f.ident).collect();
    let types: Vec<_> = fields.iter().map(|f| &f.ty).collect();
    let ret = quote::quote! {
        #(#indent: std::option::Option<#types>),*
    };
    return Ok(ret);
}

fn generate_builder_struct_fields_init(st:&syn::DeriveInput) -> syn::Result<Vec<proc_macro2::TokenStream>> {


    let fields = get_field_from_derive_input(&st)?;

    let ret:Vec<_> = fields.iter().map(|f| {

        let indent = &f.ident;

        quote::quote! {
            #indent: std::option::Option::None
        }
    }).collect();

    Ok(ret)
}

fn do_expand(st: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let struct_name_literal = st.ident.to_string();
    let builder_name_literal = format!("{}Builder", struct_name_literal);
    let struct_ident = st.ident.clone();

    let builder_name_ident = syn::Ident::new(&builder_name_literal, st.span());

    let builder_struct_fields_def = generate_builder_struct_fields_def(&st)?;
    let builder_struct_fields_init = generate_builder_struct_fields_init(&st)?;
    let ret = quote::quote! (
        pub struct #builder_name_ident {
            #builder_struct_fields_def
        }
        impl #struct_ident {
            pub fn builder() -> #builder_name_ident {
                #builder_name_ident {
                    #(#builder_struct_fields_init),*
                }
            }
        }
    );
    Ok(ret)
}
