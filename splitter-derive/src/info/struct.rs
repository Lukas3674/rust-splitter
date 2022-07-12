
use quote::{quote, format_ident};
use proc_macro::{TokenStream, Span};
use syn::{Ident, DataStruct, Generics, LifetimeDef, GenericParam, Lifetime, TypeParam};

pub fn parse<I: Iterator<Item = Ident>>(
    ident: Ident,
    data: DataStruct,
    mut generics: Generics,
    mut attrs: I,
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

        let attr_t = attrs.next();
        let next_t = generics.type_params().next();
        if !attr_t.is_some() && !next_t.is_some() {
            generics.params.push(GenericParam::Type(
                TypeParam::from(Ident::new("_SPLITTER", Span::call_site().into()))
            ));
        }

        let ref l = generics.lifetimes().next().expect("unreachable").lifetime;

        let t = attr_t.as_ref().unwrap_or_else(
            || &generics.type_params().next().expect("unreachable").ident
        );

        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        let types = data.fields.iter().map(|f| &f.ty).collect::<Vec<_>>();
        let idents = data.fields.iter().map(|f| &f.ident).collect::<Vec<_>>();

        TokenStream::from(quote! {
            mod #mident {
                use super::*;
                pub struct #cident #impl_generics #where_clause {
                    #(pub(super) #idents: <#types as Info<#l, #t>>::Context,)*
                }
                impl #impl_generics Default for #cident #ty_generics #where_clause {
                    fn default() -> Self {
                        Self { #(#idents: <#types as Info<#l, #t>>::Context::default(),)* }
                    }
                }
            }
            impl #impl_generics Info<#l, #t> for #ident #ty_generics #where_clause {
                type Context = #mident::#cident #ty_generics;
                fn generate(ctx: &mut Self::Context, ts: &#l [#t]) -> Self {
                    Self { #(#idents: <#types as Info<#l, #t>>::generate(&mut ctx.#idents, ts),)* }
                }
            }
        })
    }
}

fn _dbg(ts: TokenStream) -> TokenStream {
    println!("####\n\n{}\n\n####", ts);
    ts
}
