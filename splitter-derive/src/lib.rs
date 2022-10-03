
//! `splitter` derive macro

mod info;
mod str_info;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

/// derive macro for `Info` trait
#[proc_macro_derive(Info, attributes(splitter))]
pub fn info(input: TokenStream) -> TokenStream {
    info::parse(parse_macro_input!(input as DeriveInput))
}

/// derive macro for `StrInfo` trait
#[proc_macro_derive(StrInfo, attributes(splitter))]
pub fn str_info(input: TokenStream) -> TokenStream {
    str_info::parse(parse_macro_input!(input as DeriveInput))
}
