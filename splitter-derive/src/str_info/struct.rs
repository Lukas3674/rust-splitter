
use quote::{quote, format_ident};
use proc_macro::{TokenStream, Span};
use syn::{Ident, DataStruct, Generics, LifetimeDef, GenericParam, Lifetime};

pub fn parse(
    ident: Ident,
    data: DataStruct,
    mut generics: Generics,
) -> TokenStream {
    if data.fields.is_empty() {
        super::empty::generate(ident)
    } else {
        let cident = format_ident!("{}Ctx", ident);
        let mident = format_ident!("{}_ctx", ident.to_string().to_lowercase());

        let next_l = generics.lifetimes().next();
        if !next_l.is_some() {
            generics.params.push(GenericParam::Lifetime(
                LifetimeDef::new(Lifetime::new("_splitter", Span::call_site().into()))
            ));
        }

        let ref l = generics.lifetimes().next().expect("unreachable").lifetime;

        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        let types = data.fields.iter().map(|f| &f.ty).collect::<Vec<_>>();
        let idents = data.fields.iter().map(|f| &f.ident).collect::<Vec<_>>();

        TokenStream::from(quote! {
            mod #mident {
                use super::*;
                pub struct #cident #impl_generics #where_clause {
                    #(pub(super) #idents: <#types as StrInfo<#l>>::Context,)*
                }
                impl #impl_generics Default for #cident #ty_generics #where_clause {
                    fn default() -> Self {
                        Self { #(#idents: <#types as StrInfo<#l>>::Context::default(),)* }
                    }
                }
            }
            impl #impl_generics StrInfo<#l> for #ident #ty_generics #where_clause {
                type Context = #mident::#cident #ty_generics;
                fn generate(ctx: &mut Self::Context, ts: &#l str) -> Self {
                    Self { #(#idents: <#types as StrInfo<#l>>::generate(&mut ctx.#idents, ts),)* }
                }
            }
        })
    }
}
