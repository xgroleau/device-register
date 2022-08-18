#![doc(html_no_source)]

use std::{ops::Add, iter::Inspect};

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput};
use darling::{FromDeriveInput, ToTokens, FromMeta};


#[derive(FromDeriveInput)]
#[darling(attributes(register))]
struct Register {
    addr: syn::Lit,
    ty: Option<syn::Path>,
}

fn impl_register(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    
    let reg = Register::from_derive_input(ast).expect("Could not parse address");
    let addr = reg.addr;
    let ty = reg.ty.unwrap_or(syn::parse_str("u8").unwrap());
    quote! {
        #[allow(dead_code)]
        impl device_register::Register for #name {
            type Address = #ty;
            const ADDRESS: Self::Address = #addr;
        }
    }
}

/// Create a read only register
#[proc_macro_derive(RORegister, attributes(register))]
pub fn ro_register(input: TokenStream) -> TokenStream {
    // Parse the representation
    // let args = parse_macro_input!(input as AttributeArgs);
    let ast: DeriveInput = syn::parse(input).unwrap();

    // Build the impl

    let mut output = impl_register(&ast);
    output.extend(impl_ro_register(&ast));
    output.into()
}

fn impl_ro_register(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;

    quote! {
        #[allow(dead_code)]
        impl device_register::ReadableRegister for #name {}
    }
}

/// Create an edit only register
#[proc_macro_derive(EORegister, attributes(register))]
pub fn eo_register(input: TokenStream) -> TokenStream {
    // Parse the representation
    let ast = syn::parse(input).unwrap();

    // Build the impl
    let mut output = impl_register(&ast);
    output.extend(impl_eo_register(&ast));
    output.into()
}

/// Create a read/edit register
#[proc_macro_derive(RERegister, attributes(register))]
pub fn re_register(input: TokenStream) -> TokenStream {
    // Parse the representation
    let ast = syn::parse(input).unwrap();

    // Build the impl
    let mut output = impl_register(&ast);
    output.extend(impl_ro_register(&ast));
    output.extend(impl_eo_register(&ast));
    output.into()
}

fn impl_eo_register(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    quote! {
        #[allow(dead_code)]
        impl device_register::EditableRegister for #name {}
    }
}

/// Create a read/write register
#[proc_macro_derive(WORegister, attributes(register))]
pub fn wo_register(input: TokenStream) -> TokenStream {
    // Parse the representation
    let ast = syn::parse(input).unwrap();

    // Build the impl
    let mut output = impl_register(&ast);
    output.extend(impl_wo_register(&ast));
    output.into()
}

/// Create a read/write register
#[proc_macro_derive(RWRegister, attributes(register))]
pub fn rw_register(input: TokenStream) -> TokenStream {
    // Parse the representation
    let ast = syn::parse(input).unwrap();

    // Build the impl
    let mut output = impl_register(&ast);
    output.extend(impl_ro_register(&ast));
    output.extend(impl_eo_register(&ast));
    output.extend(impl_wo_register(&ast));
    output.into()
}

fn impl_wo_register(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    quote! {
        #[allow(dead_code)]
        impl device_register::WritableRegister for #name {}
    }
}
