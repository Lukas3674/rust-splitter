
mod empty;
mod r#struct;

use proc_macro::TokenStream;
use syn::{DeriveInput, Data, Ident};


pub fn parse(input: DeriveInput) -> TokenStream {
    let attrs = input.attrs.into_iter()
        .filter_map(|attr| attr.path.is_ident("splitter").then(|| {
            attr.parse_args::<Ident>().expect("TODO")
        }));
    match input.data {
        Data::Struct(data) => r#struct::parse(input.ident, data, input.generics, attrs),
        Data::Enum(_data) => panic!("Info trait is not allowed on enums"),
        Data::Union(_data) => panic!("Info trait is not allowed on unions"),
    }
}
