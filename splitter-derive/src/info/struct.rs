
use proc_macro::{TokenStream, Span};
use quote::{quote, format_ident, __private::TokenStream as TS};
use syn::{
    Ident, DataStruct, Generics, LifetimeDef, GenericParam, Lifetime,
    TypeParam, Fields, FieldsNamed, FieldsUnnamed, Index,
};

pub fn parse<I: Iterator<Item = Ident>>(
    ident: Ident,
    data: DataStruct,
    base_generics: Generics,
    mut attrs: I,
) -> TokenStream {
    if data.fields.is_empty() {
        super::empty::generate(ident)
    } else {
        let attr = attrs.next();

        let cident = format_ident!("{}Ctx", ident);
        let mident = format_ident!("{}_ctx", ident.to_string().to_lowercase());

        let mut generics = base_generics.clone();
        if base_generics.lifetimes().next().is_none() {
            generics.params.push(GenericParam::Lifetime(LifetimeDef::new(
                Lifetime::new("'_splitter", Span::call_site().into())
            )));
        }
        if attr.is_none() && base_generics.type_params().next().is_none() {
            generics.params.push(GenericParam::Type(TypeParam::from(Ident::new(
                "_SPLITTER", Span::call_site().into()
            ))));
        }

        let l = &generics.lifetimes().next().expect("unreachable").lifetime;
        let t = attr.as_ref().unwrap_or_else(
            || &generics.type_params().next().expect("unreachable").ident
        );

        let (_, base_ty, _) = base_generics.split_for_impl();
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        let (ctx, ctx_con, ge_con) = match data.fields {
            Fields::Unnamed(fields) => unnamed(fields, l, t),
            Fields::Named(fields) => named(fields, l, t),
            Fields::Unit => unreachable!("a"),
        };

        TokenStream::from(quote! {
            mod #mident {
                use super::*;
                pub struct #cident #impl_generics #where_clause #ctx
                impl #impl_generics Default for #cident #ty_generics #where_clause {
                    fn default() -> Self { #ctx_con }
                }
            }
            impl #impl_generics Info<#l, #t> for #ident #base_ty #where_clause {
                type Context = #mident::#cident #ty_generics;
                fn generate(ctx: &mut Self::Context, ts: &#l [#t]) -> Self { #ge_con }
            }
        })
    }
}

fn named(fields: FieldsNamed, l: &Lifetime, t: &Ident) -> (TS, TS, TS) {
    let (idents, types): (Vec<_>, Vec<_>) = fields.named.into_iter()
        .map(|f| (f.ident.expect("unreachable"), f.ty)).unzip();
    (
        quote! { { #(pub(super) #idents: <#types as Info<#l, #t>>::Context,)* } },
        quote! { Self { #(#idents: <#types as Info<#l, #t>>::Context::default(),)* } },
        quote! { Self { #(#idents: <#types as Info<#l, #t>>::generate(&mut ctx.#idents, ts),)* } },
    )
}

fn unnamed(fields: FieldsUnnamed, l: &Lifetime, t: &Ident) -> (TS, TS, TS) {
    let (idents, types): (Vec<_>, Vec<_>) = fields.unnamed.into_iter()
        .enumerate().map(|(i, f)| (Index::from(i), f.ty)).unzip();
    (
        quote! { ( #(pub(super) <#types as Info<#l, #t>>::Context,)* ); },
        quote! { Self ( #(<#types as Info<#l, #t>>::Context::default(),)* ) },
        quote! { Self ( #(<#types as Info<#l, #t>>::generate(&mut ctx.#idents, ts),)* ) },
    )
}
