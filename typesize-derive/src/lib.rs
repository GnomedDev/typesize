use either::Either;
use proc_macro2::{Ident, Punct, Spacing, Span, TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn::{parse_macro_input, punctuated::Punctuated, DeriveInput, Field, Token, Variant};

fn join_tokens(
    exprs: impl ExactSizeIterator<Item = impl ToTokens>,
    sep: impl ToTokens,
) -> TokenStream {
    let expr_count = exprs.len();
    let mut out_tokens = TokenStream::new();
    for (i, expr) in exprs.enumerate() {
        expr.to_tokens(&mut out_tokens);
        if expr_count != i + 1 {
            sep.to_tokens(&mut out_tokens);
        }
    }

    out_tokens
}

fn gen_unnamed_ident(i: usize) -> Ident {
    Ident::new(&format!("var_field_{i}"), Span::call_site())
}

fn gen_named_exprs<'a>(
    named_fields: impl ExactSizeIterator<Item = &'a Field> + 'a,
    include_self: bool,
) -> impl ExactSizeIterator<Item = TokenStream> + 'a {
    if named_fields.len() == 0 {
        return Either::Left(std::iter::once(quote!(0_usize)));
    }

    Either::Right(named_fields.map(|field| &field.ident).map(move |ident| {
        if include_self {
            quote!(::typesize::TypeSize::extra_size(&self.#ident))
        } else {
            quote!(::typesize::TypeSize::extra_size(#ident))
        }
    }))
}

fn gen_unnamed_exprs(
    field_count: usize,
    include_self: bool,
) -> impl ExactSizeIterator<Item = TokenStream> {
    if field_count == 0 {
        return Either::Left(std::iter::once(quote!(0_usize)));
    };

    Either::Right((0..field_count).map(move |i| {
        if include_self {
            let ident = syn::Index::from(i);
            quote!(::typesize::TypeSize::extra_size(&self.#ident))
        } else {
            let ident = gen_unnamed_ident(i);
            quote!(::typesize::TypeSize::extra_size(#ident))
        }
    }))
}

fn gen_sum_exprs(exprs: impl ExactSizeIterator<Item = impl ToTokens>) -> TokenStream {
    join_tokens(exprs, Punct::new('+', Spacing::Alone))
}

fn gen_comma_exprs(exprs: impl ExactSizeIterator<Item = impl ToTokens>) -> TokenStream {
    join_tokens(exprs, Punct::new(',', Spacing::Alone))
}

fn gen_struct(data: &syn::Fields, include_self: bool) -> TokenStream {
    match data {
        syn::Fields::Named(fields) => {
            gen_sum_exprs(gen_named_exprs(fields.named.iter(), include_self))
        }
        syn::Fields::Unnamed(fields) => {
            gen_sum_exprs(gen_unnamed_exprs(fields.unnamed.len(), include_self))
        }
        syn::Fields::Unit => quote!(0_usize),
    }
}

fn get_named_idents(fields: &Punctuated<Field, Token![,]>) -> TokenStream {
    let idents = fields.iter().map(|field| field.ident.as_ref().unwrap());
    join_tokens(idents, TokenTree::Punct(Punct::new(',', Spacing::Alone)))
}

fn gen_unnamed_idents(field_count: usize) -> TokenStream {
    let idents = (0..field_count).map(gen_unnamed_ident);
    gen_comma_exprs(idents)
}

fn gen_enum(variants: impl Iterator<Item = Variant>) -> TokenStream {
    let match_arms: TokenStream = variants
        .map(|variant| {
            let variant_name = &variant.ident;
            let variant_body = gen_struct(&variant.fields, false);
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

            quote!(Self::#variant_name #variant_pattern => #variant_body,)
        })
        .collect();

    quote!(
        match self {
            #match_arms
        }
    )
}

#[proc_macro_derive(TypeSize)]
pub fn typesize_derive(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput {
        attrs: _,
        vis: _,
        ident,
        generics,
        data,
    } = parse_macro_input!(tokens as DeriveInput);

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let body = match data {
        syn::Data::Struct(data) => gen_struct(&data.fields, true),
        syn::Data::Enum(data) => gen_enum(data.variants.into_iter()),
        syn::Data::Union(_) => panic!("Unions are unsupported for typesize derive!"),
    };

    let output = quote! {
        #[automatically_derived]
        impl #impl_generics ::typesize::TypeSize for #ident #ty_generics #where_clause {
            fn extra_size(&self) -> usize {
                #body
            }
        }
    };

    output.into()
}
