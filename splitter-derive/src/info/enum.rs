
use proc_macro::{TokenStream, Span};
use quote::{quote, ToTokens, __private::TokenStream as TS};
use syn::{Ident, DataEnum, Variant, parse::Parse, Expr, punctuated::Punctuated, Token};

pub fn parse<I: Iterator<Item = Ident>>(
    ident: Ident,
    data: DataEnum,
    mut attrs: I,
) -> TokenStream {
    let mut was_none = false;
    let t = attrs.next().unwrap_or_else(|| {
        was_none = true;
        Ident::new("_SPLITTER", Span::call_site().into())
    });
    let opt_t = was_none.then(|| &t);
    let opt_t = opt_t.iter();

    let variants = data.variants.into_iter().map(pares_variant);
    TokenStream::from(quote! {
        impl<'_splitter #(, #opt_t)*> Info<'_splitter, #t> for #ident {
            type Context = ();
            fn generate(_: &mut Self::Context, _splitter_str: &'_splitter [#t]) -> Self {
                match _splitter_str { #(#variants)* }
            }
        }
    })
}

struct Pat {
    strs: Punctuated<Expr, Token![|]>,
    guard: Option<Expr>,
}

impl Parse for Pat {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let strs = Punctuated::parse_separated_nonempty(input)?;
        let guard = if input.peek(Token![if]) {
            input.parse::<Token![if]>()?;
            Some(input.parse()?)
        } else { None };
        Ok(Self { strs, guard })
    }
}

impl ToTokens for Pat {
    fn to_tokens(&self, tokens: &mut TS) {
        let mut iter = self.strs.iter();
        if let Some(first) = iter.next() {
            tokens.extend(quote!(#first));
            tokens.extend(iter.map(|s| quote!{ | #s }));
            if let Some(guard) = self.guard.as_ref() {
                tokens.extend(quote! { if #guard });
            }
        }
    }
}

fn pares_variant(variant: Variant) -> TS {
    if !variant.fields.is_empty() {
        panic!("variants can't have fields")
    }

    let mut pat = variant.attrs.into_iter()
        .filter_map(|attr| attr.path.is_ident("splitter").then(|| {
            attr.parse_args::<Pat>().expect("the splitter attribute has to include a pattern")
        }));

    let ident = variant.ident;
    if let Some(pat) = pat.next() {
        quote! { #pat => Self::#ident, }
    } else {
        panic!("every variant needs a splitter attribute")
    }
}
