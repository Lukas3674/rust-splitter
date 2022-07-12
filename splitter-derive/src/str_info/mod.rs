
mod empty;
mod r#struct;

use proc_macro::TokenStream;
use syn::{DeriveInput, Data};

pub fn parse(input: DeriveInput) -> TokenStream {
    match input.data {
        Data::Struct(data) => r#struct::parse(input.ident, data, input.generics),
        Data::Enum(_data) => panic!("StrInfo trait is not allowed on enums"),
        Data::Union(_data) => panic!("StrInfo trait is not allowed on unions"),
    }
}
