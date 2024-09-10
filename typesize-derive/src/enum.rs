use proc_macro2::{Punct, Spacing, Span, TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn::{punctuated::Punctuated, Field, Ident, Token, Variant};

use crate::{extra_details_visit_fields, join_tokens, GenerationRet, PassMode};

fn gen_unnamed_ident(i: usize) -> Ident {
    Ident::new(&format!("var_field_{i}"), Span::call_site())
}

fn get_named_idents(fields: &Punctuated<Field, Token![,]>) -> TokenStream {
    let idents = fields.iter().map(|field| field.ident.as_ref().unwrap());
    join_tokens(idents, TokenTree::Punct(Punct::new(',', Spacing::Alone)))
}

fn gen_unnamed_idents(field_count: usize) -> TokenStream {
    let idents = (0..field_count).map(gen_unnamed_ident);
    join_tokens(idents, Punct::new(',', Spacing::Alone))
}

fn gen_match_arm(variant: &Variant, body: impl ToTokens) -> TokenStream {
    let variant_name = &variant.ident;
    let variant_pattern = match &variant.fields {
        syn::Fields::Named(fields) => {
            let field_names = get_named_idents(&fields.named);
            quote!({#field_names})
        }
        syn::Fields::Unnamed(fields) => {
            let field_names = gen_unnamed_idents(fields.unnamed.len());
            quote!((#field_names))
        }
        syn::Fields::Unit => TokenStream::new(),
    };

    quote!(Self::#variant_name #variant_pattern => #body,)
}

pub(crate) fn gen_enum(variants: impl Iterator<Item = Variant>, is_packed: bool) -> GenerationRet {
    assert!(!is_packed, "repr(packed) enums are not supported!");

    let arms: TokenStream = variants
        .map(|variant| {
            gen_match_arm(
                &variant,
                extra_details_visit_fields(
                    &variant.fields,
                    |ident| quote!(#ident),
                    |index| gen_unnamed_ident(index).to_token_stream(),
                    PassMode::AsIs,
                ),
            )
        })
        .collect();

    GenerationRet {
        extra_size: quote!(match self {#arms}),
        #[cfg(feature = "details")]
        details: None,
    }
}
