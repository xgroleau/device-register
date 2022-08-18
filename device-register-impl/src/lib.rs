use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput};
use darling::{FromDeriveInput, ToTokens, FromMeta};

enum Address {
    Lit(syn::Lit),
    Path(syn::Path),
}
impl ToTokens for Address {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Address::Lit(val) => val.to_tokens(tokens),
            Address::Path(val) => val.to_tokens(tokens),
        }
    }
}
impl FromMeta for Address {
    fn from_meta(item: &syn::Meta) -> darling::Result<Self> {
        (match &*item {
            syn::Meta::Path(p) => Ok(Address::Path(p.clone())),
            syn::Meta::List(ref value) => Self::from_list(
                &value
                    .nested
                    .iter()
                    .cloned()
                    .collect::<Vec<syn::NestedMeta>>()[..],
            ),
            syn::Meta::NameValue(ref value) => Self::from_value(&value.lit),
        })
        .map_err(|e| e.with_span(item))
    }

    fn from_value(value: &syn::Lit) -> darling::Result<Self> { 
        match value {
            syn::Lit::Str(str) => {
                let path: syn::Path = str.parse()?;
                Ok(Address::Path(path))
            },
            val => Ok(Address::Lit(val.clone()))
        }
    }

}
#[derive(darling::FromDeriveInput)]
#[darling(attributes(register))]
struct Register {
    addr: Address,
    ty: Option<syn::TypePath>,
}

fn impl_register(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let reg = Register::from_derive_input(ast).expect("Could not parse register attribute");
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
