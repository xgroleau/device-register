//! A crate for the macro used in the device-register crate. See device-register crate for more information.
#![deny(unsafe_code, missing_docs)]

use darling::{FromDeriveInput, FromMeta, ToTokens};
use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

/// The valid values of an address
enum Address {
    /// A literal (float, int, bytestring, etc)
    Lit(syn::Lit),

    /// A path to support enums
    Pat(syn::Pat),
}

impl ToTokens for Address {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Address::Lit(val) => val.to_tokens(tokens),
            Address::Pat(val) => val.to_tokens(tokens),
        }
    }
}
impl FromMeta for Address {
    fn from_meta(item: &syn::Meta) -> darling::Result<Self> {
        match item {
            syn::Meta::Path(_) => Self::from_word(),
            syn::Meta::List(ref value) => Self::from_list(
                &value
                    .nested
                    .iter()
                    .cloned()
                    .collect::<Vec<syn::NestedMeta>>()[..],
            ),
            syn::Meta::NameValue(ref value) => Self::from_value(&value.lit),
        }
        .map_err(|e| e.with_span(item))
    }

    fn from_value(value: &syn::Lit) -> darling::Result<Self> {
        match value {
            syn::Lit::Str(str) => {
                let pattern: syn::Pat = str.parse()?;
                Ok(Address::Pat(pattern))
            }
            val => Ok(Address::Lit(val.clone())),
        }
    }
}

/// The arguments passed to the register helper attribute
#[derive(darling::FromDeriveInput)]
#[darling(attributes(register))]
struct Register {
    /// The value of the address
    addr: Address,

    /// The type of the address, defaults to a u8
    ty: Option<syn::Type>,
}

fn impl_register(ast: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = &ast.ident;
    let reg = Register::from_derive_input(ast)?;
    let addr = reg.addr;
    let ty = reg.ty.unwrap_or_else(|| syn::parse_str("u8").unwrap());
    let (impl_gen, type_gen, where_gen) = &ast.generics.split_for_impl();
    Ok(quote! {
        #[allow(dead_code)]
        impl #impl_gen device_register::Register for #name #type_gen #where_gen {
            type Address = #ty;
            const ADDRESS: Self::Address = #addr;
        }
    })
}

/// Create a read only register
#[proc_macro_derive(RORegister, attributes(register))]
pub fn ro_register(input: TokenStream) -> TokenStream {
    // Parse the representation
    // let args = parse_macro_input!(input as AttributeArgs);
    let ast: DeriveInput = syn::parse(input).unwrap();

    // Build the impl

    let mut output = impl_register(&ast).unwrap_or_else(syn::Error::into_compile_error);
    output.extend(impl_ro_register(&ast));
    output.into()
}

fn impl_ro_register(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let (impl_gen, type_gen, where_gen) = &ast.generics.split_for_impl();

    quote! {
        #[allow(dead_code)]
        impl #impl_gen device_register::ReadableRegister for #name #type_gen #where_gen{}
    }
}

/// Create an edit only register
#[proc_macro_derive(EORegister, attributes(register))]
pub fn eo_register(input: TokenStream) -> TokenStream {
    // Parse the representation
    let ast = syn::parse(input).unwrap();

    // Build the impl
    let mut output = impl_register(&ast).unwrap_or_else(syn::Error::into_compile_error);
    output.extend(impl_eo_register(&ast));
    output.into()
}

/// Create a read/edit register
#[proc_macro_derive(RERegister, attributes(register))]
pub fn re_register(input: TokenStream) -> TokenStream {
    // Parse the representation
    let ast = syn::parse(input).unwrap();

    // Build the impl
    let mut output = impl_register(&ast).unwrap_or_else(syn::Error::into_compile_error);
    output.extend(impl_ro_register(&ast));
    output.extend(impl_eo_register(&ast));
    output.into()
}

fn impl_eo_register(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let (impl_gen, type_gen, where_gen) = &ast.generics.split_for_impl();
    quote! {
        #[allow(dead_code)]
        impl #impl_gen device_register::EditableRegister for #name #type_gen #where_gen {}
    }
}

/// Create a write only register
#[proc_macro_derive(WORegister, attributes(register))]
pub fn wo_register(input: TokenStream) -> TokenStream {
    // Parse the representation
    let ast = syn::parse(input).unwrap();

    // Build the impl
    let mut output = impl_register(&ast).unwrap_or_else(syn::Error::into_compile_error);
    output.extend(impl_wo_register(&ast));
    output.into()
}

/// Create a read/write register
#[proc_macro_derive(RWRegister, attributes(register))]
pub fn rw_register(input: TokenStream) -> TokenStream {
    // Parse the representation
    let ast = syn::parse(input).unwrap();

    // Build the impl
    let mut output = impl_register(&ast).unwrap_or_else(syn::Error::into_compile_error);
    output.extend(impl_ro_register(&ast));
    output.extend(impl_eo_register(&ast));
    output.extend(impl_wo_register(&ast));
    output.into()
}

fn impl_wo_register(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let (impl_gen, type_gen, where_gen) = &ast.generics.split_for_impl();
    quote! {
        #[allow(dead_code)]
        impl #impl_gen device_register::WritableRegister for #name #type_gen #where_gen {}
    }
}
