
mod empty;
mod r#enum;
mod r#struct;

use proc_macro::TokenStream;
use syn::{DeriveInput, Data};

pub fn parse(input: DeriveInput) -> TokenStream {
    match input.data {
        Data::Struct(data) => r#struct::parse(input.ident, data, input.generics),
        Data::Enum(data) => r#enum::parse(input.ident, data),
        Data::Union(_data) => panic!("StrInfo trait is not allowed on unions"),
    }
}
