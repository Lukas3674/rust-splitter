
//! `Splitter` derive macro

use quote::{quote, format_ident};
use proc_macro::{TokenStream, Span};
use syn::{
    parse_macro_input, DeriveInput, Data, GenericParam, Lifetime, Ident,
};

/// derive macro for `Info` trait
#[proc_macro_derive(Info)]
pub fn info(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    if let Data::Struct(ref data) = input.data {
        let empty = data.fields.is_empty();

        let ls = input.generics.params.iter().filter_map(|g| match g {
            GenericParam::Lifetime(l) => Some(&l.lifetime), _ => None,
        }).collect::<Vec<_>>();
        let l = ls.first().map(|&l| l.clone()).unwrap_or_else(|| if empty {
            Lifetime::new("'_", Span::call_site().into())
        } else {
            Lifetime::new("'a", Span::call_site().into())
        });
        let ol = (ls.is_empty() && !empty).then(|| &l);
        let ol1 = ol.iter();
        let ol2 = ol.iter();
        let ol3 = ol.iter();
        let ol4 = ol.iter();
        let ol5 = ol.iter();

        let ts = input.generics.params.iter().filter_map(|g| match g {
            GenericParam::Type(t) => Some(&t.ident), _ => None,
        }).collect::<Vec<_>>();
        let t = ts.first().map(|&t| t.clone())
            .unwrap_or_else(|| Ident::new("T", Span::call_site().into()));
        let ot = ts.is_empty().then(|| &t);
        let ot = ot.iter();

        let bs = input.generics.params.iter().filter_map(|g| match g {
            GenericParam::Type(t) => Some(&t.bounds), _ => None,
        }).collect::<Vec<_>>();

        let cs = input.generics.params.iter().filter_map(|g| match g {
            GenericParam::Const(c) => Some(&c.ident), _ => None,
        }).collect::<Vec<_>>();
        let cts = input.generics.params.iter().filter_map(|g| match g {
            GenericParam::Const(c) => Some(&c.ty), _ => None,
        }).collect::<Vec<_>>();

        let ident = &input.ident;
        let cident = format_ident!("{}Ctx", ident);
        let mident = format_ident!("{}_ctx", ident.to_string().to_lowercase());

        let types = data.fields.iter().map(|t| &t.ty).collect::<Vec<_>>();
        let idents = data.fields.iter().map(|i| &i.ident).collect::<Vec<_>>();

        TokenStream::from(quote! {
            mod #mident {
                use super::*;

                pub struct #cident<#(#ol1,)* #(#ls,)* #(#ts: #bs,)* #(const #cs: #cts,)*> {
                    #(pub #idents: <#types as Info<#l, #t>>::Context,)*
                }

                impl<#(#ol2,)* #(#ls,)* #(#ts: #bs,)* #(const #cs: #cts,)*> Default
                for #cident<#(#ol3,)* #(#ls,)* #(#ts,)* #(#cs,)*> {
                    fn default() -> Self {
                        Self { #(#idents: <#types as Info<#l, #t>>::Context::default(),)* }
                    }
                }
            }

            impl<#(#ol4,)* #(#ls,)* #(#ot,)* #(#ts: #bs,)* #(const #cs: #cts,)*> Info<#l, #t>
            for #ident<#(#ls,)* #(#ts,)* #(#cs,)*> {
                type Context = #mident::#cident<#(#ol5,)* #(#ls,)* #(#ts,)* #(#cs,)*>;
                fn generate(ctx: &mut Self::Context, ts: &#l [#t]) -> Self {
                    Self {
                        #(#idents: <#types as Info<#l, #t>>::generate(&mut ctx.#idents, ts),)*
                    }
                }
            }
        })
    } else {
        panic!("Can only derive 'Info' on structs");
    }
}

/// derive macro for `StrInfo` trait
#[proc_macro_derive(StrInfo)]
pub fn str_info(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    if let Data::Struct(ref data) = input.data {
        let empty = data.fields.is_empty();

        let ls = input.generics.params.iter().filter_map(|g| match g {
            GenericParam::Lifetime(l) => Some(&l.lifetime), _ => None,
        }).collect::<Vec<_>>();
        let l = ls.first().map(|&l| l.clone()).unwrap_or_else(|| if empty {
            Lifetime::new("'_", Span::call_site().into())
        } else {
            Lifetime::new("'a", Span::call_site().into())
        });
        let ol = (ls.is_empty() && !empty).then(|| &l);
        let ol1 = ol.iter();
        let ol2 = ol.iter();
        let ol3 = ol.iter();
        let ol4 = ol.iter();
        let ol5 = ol.iter();

        let ts = input.generics.params.iter().filter_map(|g| match g {
            GenericParam::Type(t) => Some(&t.ident), _ => None,
        }).collect::<Vec<_>>();
        let bs = input.generics.params.iter().filter_map(|g| match g {
            GenericParam::Type(t) => Some(&t.bounds), _ => None,
        }).collect::<Vec<_>>();

        let cs = input.generics.params.iter().filter_map(|g| match g {
            GenericParam::Const(c) => Some(&c.ident), _ => None,
        }).collect::<Vec<_>>();
        let cts = input.generics.params.iter().filter_map(|g| match g {
            GenericParam::Const(c) => Some(&c.ty), _ => None,
        }).collect::<Vec<_>>();

        let ident = &input.ident;
        let cident = format_ident!("{}Ctx", ident);
        let mident = format_ident!("{}_ctx", ident.to_string().to_lowercase());

        let types = data.fields.iter().map(|t| &t.ty).collect::<Vec<_>>();
        let idents = data.fields.iter().map(|i| &i.ident).collect::<Vec<_>>();

        TokenStream::from(quote! {
            mod #mident {
                use super::*;

                pub struct #cident<#(#ol1,)* #(#ls,)* #(#ts: #bs,)* #(const #cs: #cts,)*> {
                    #(pub #idents: <#types as StrInfo<#l>>::Context,)*
                }

                impl<#(#ol2,)* #(#ls,)* #(#ts: #bs,)* #(const #cs: #cts,)*> Default
                for #cident<#(#ol3,)* #(#ls,)* #(#ts,)* #(const #cs,)*> {
                    fn default() -> Self {
                        Self { #(#idents: <#types as StrInfo<#l>>::Context::default(),)* }
                    }
                }
            }

            impl<#(#ol4,)* #(#ls,)* #(#ts: #bs,)* #(const #cs: #cts,)*> StrInfo<#l>
            for #ident<#(#ls,)* #(#ts,)* #(#cs,)*> {
                type Context = #mident::#cident<#(#ol5,)* #(#ls,)* #(#ts,)* #(#cs,)*>;
                fn generate(ctx: &mut Self::Context, ts: &#l str) -> Self {
                    Self {
                        #(#idents: <#types as StrInfo<#l>>::generate(&mut ctx.#idents, ts),)*
                    }
                }
            }
        })
    } else {
        panic!("Can only derive 'StrInfo' on structs");
    }
}

