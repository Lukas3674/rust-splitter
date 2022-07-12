
use syn::Ident;
use quote::quote;
use proc_macro::TokenStream;

pub fn generate(ident: Ident) -> TokenStream {
    TokenStream::from(quote! {
        impl<'_splitter, _SPLITTER> Info<'_splitter, _SPLITTER> for #ident {
            type Context = ();
            fn generate(_: &mut Self::Context, _: &'_splitter [_SPLITTER]) -> Self {
                Self {}
            }
        }
    })
}
