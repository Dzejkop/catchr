extern crate proc_macro;

use catchr_core::{CatchrMode, Section};
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;

#[proc_macro]
pub fn describe(input: TokenStream) -> TokenStream {
    let section_body = parse_macro_input!(input as Section);

    let output = section_body.to_token_stream();

    output.into()
}

#[proc_macro]
pub fn describe_tokio(input: TokenStream) -> TokenStream {
    let section_body = parse_macro_input!(input as Section);

    let output = section_body.with_mode(CatchrMode::Tokio).to_token_stream();

    output.into()
}
