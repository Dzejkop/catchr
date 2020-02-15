extern crate proc_macro;

use catchr_core::{CatchrMod, Section};
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
pub fn module(input: TokenStream) -> TokenStream {
    let catchr_mod = parse_macro_input!(input as CatchrMod);

    let output = catchr_mod.to_token_stream();

    output.into()
}
