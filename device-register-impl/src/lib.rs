#![doc(html_no_source)]

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, LitInt};

fn impl_register(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let mut address: Option<u8> = None;

    for attr in ast.clone().attrs {
        if attr.path.is_ident("address") {
            let addr: LitInt = attr
                .parse_args()
                .expect("The `address` attribute is required");
            let num: u8 = addr
                .base10_parse()
                .expect("The `address` attribute must have a u8 as argument");

            if address.is_some() {
                panic!("Multiple `address` defined")
            }
            address = Some(num);
        }
    }

    let addr: u8 = address.expect("The `address` attribute is not defined");
    quote! {
        #[allow(dead_code)]
        impl device_register::Register for #name {
            type Address = u8;
            const ADDRESS: Self::Address = #addr;
        }
    }
}

/// Create a read only register
#[proc_macro_derive(RORegister, attributes(address))]
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
#[proc_macro_derive(EORegister, attributes(address))]
pub fn eo_register(input: TokenStream) -> TokenStream {
    // Parse the representation
    let ast = syn::parse(input).unwrap();

    // Build the impl
    let mut output = impl_register(&ast);
    output.extend(impl_eo_register(&ast));
    output.into()
}

/// Create a read/edit register
#[proc_macro_derive(RERegister, attributes(address))]
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
#[proc_macro_derive(WORegister, attributes(address))]
pub fn wo_register(input: TokenStream) -> TokenStream {
    // Parse the representation
    let ast = syn::parse(input).unwrap();

    // Build the impl
    let mut output = impl_register(&ast);
    output.extend(impl_wo_register(&ast));
    output.into()
}

/// Create a read/write register
#[proc_macro_derive(RWRegister, attributes(address))]
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
