
use proc_macro::{TokenStream, Span};
use quote::{quote, format_ident, __private::TokenStream as TS};
use syn::{
    Ident, DataStruct, Generics, LifetimeDef, GenericParam, Lifetime,
    Fields, FieldsNamed, FieldsUnnamed, Index,
};

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
                LifetimeDef::new(Lifetime::new("'_splitter", Span::call_site().into()))
            ));
        }

        let ref l = generics.lifetimes().next().expect("unreachable").lifetime;

        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        let (ctx, ctx_con, ge_con) = match data.fields {
            Fields::Unnamed(fields) => unnamed(fields, l),
            Fields::Named(fields) => named(fields, l),
            Fields::Unit => unreachable!(),
        };

        TokenStream::from(quote! {
            mod #mident {
                use super::*;
                pub struct #cident #impl_generics #where_clause #ctx
                impl #impl_generics Default for #cident #ty_generics #where_clause {
                    fn default() -> Self { #ctx_con }
                }
            }
            impl #impl_generics StrInfo<#l> for #ident #ty_generics #where_clause {
                type Context = #mident::#cident #ty_generics;
                fn generate(ctx: &mut Self::Context, s: &#l str) -> Self { #ge_con }
            }
        })
    }
}

fn named(fields: FieldsNamed, l: &Lifetime) -> (TS, TS, TS) {
    let (idents, types): (Vec<_>, Vec<_>) = fields.named.into_iter()
        .map(|f| (f.ident.expect("unreachable"), f.ty)).unzip();
    (
        quote! { { #(pub(super) #idents: <#types as StrInfo<#l>>::Context,)* } },
        quote! { Self { #(#idents: <#types as StrInfo<#l>>::Context::default(),)* } },
        quote! { Self { #(#idents: <#types as StrInfo<#l>>::generate(&mut ctx.#idents, s),)* } },
    )
}

fn unnamed(fields: FieldsUnnamed, l: &Lifetime) -> (TS, TS, TS) {
    let (idents, types): (Vec<_>, Vec<_>) = fields.unnamed.into_iter()
        .enumerate().map(|(i, f)| (Index::from(i), f.ty)).unzip();
    (
        quote! { ( #(pub(super) <#types as StrInfo<#l>>::Context,)* ); },
        quote! { Self ( #(<#types as StrInfo<#l>>::Context::default(),)* ) },
        quote! { Self ( #(<#types as StrInfo<#l>>::generate(&mut ctx.#idents, s),)* ) },
    )
}
