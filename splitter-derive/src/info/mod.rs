
mod empty;
mod r#enum;
mod r#struct;

use proc_macro::TokenStream;
use syn::{DeriveInput, Data, Ident};


pub fn parse(input: DeriveInput) -> TokenStream {
    let attrs = input.attrs.into_iter()
        .filter_map(|attr| attr.path.is_ident("splitter").then(|| {
            attr.parse_args::<Ident>().expect("splitter attribute has to include a type")
        }));
    match input.data {
        Data::Struct(data) => r#struct::parse(input.ident, data, input.generics, attrs),
        Data::Enum(data) => r#enum::parse(input.ident, data, attrs),
        Data::Union(_data) => panic!("Info trait is not allowed on unions"),
    }
}
