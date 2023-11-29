use proc_macro2::{Ident, Punct, Spacing, Span, TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn::{parse_macro_input, punctuated::Punctuated, DeriveInput, Field, Token, Variant};

mod extra_size;
#[cfg(feature = "details")]
mod field_details;

#[derive(Clone, Copy)]
pub(crate) enum IdentMode {
    NoRef,
    InsertRef,
    Packed,
}

impl IdentMode {
    pub(crate) fn transform(self, func_name: &impl ToTokens, ident: &impl ToTokens) -> TokenStream {
        match self {
            IdentMode::InsertRef => quote!(#func_name(&#ident)),
            IdentMode::NoRef => quote!(#func_name(#ident)),
            IdentMode::Packed => {
                quote!(({
                    let __typesize_internal_temp = #ident;
                    #func_name(&__typesize_internal_temp)
                }))
            }
        }
    }
}

struct GenerationRet {
    extra_size: TokenStream,
    #[cfg(feature = "details")]
    details: Option<TokenStream>,
}

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
    transform_named: impl Fn(&'a Option<Ident>) -> TokenStream + 'a,
    common_body: impl Fn((TokenStream, TokenStream)) -> TokenStream + 'a,
) -> Option<impl ExactSizeIterator<Item = TokenStream> + 'a> {
    if named_fields.len() == 0 {
        return None;
    }

    Some(
        named_fields
            .map(|field| &field.ident)
            .map(move |ident| (transform_named(ident), quote!(#ident)))
            .map(common_body),
    )
}

fn gen_unnamed_exprs(
    field_count: usize,
    transform_unnamed: impl Fn(usize) -> TokenStream,
    common_body: impl Fn((TokenStream, TokenStream)) -> TokenStream,
) -> Option<impl ExactSizeIterator<Item = TokenStream>> {
    if field_count == 0 {
        return None;
    };

    Some(
        (0..field_count)
            .map(move |i| (transform_unnamed(i), i.to_token_stream()))
            .map(common_body),
    )
}

fn for_each_field<'a>(
    fields: &'a syn::Fields,
    join_with: Punct,
    transform_named: impl Fn(&'a Option<Ident>) -> TokenStream + 'a,
    transform_unnamed: impl Fn(usize) -> TokenStream + 'a,
    common_body: impl Fn((TokenStream, TokenStream)) -> TokenStream + 'a,
) -> Option<TokenStream> {
    match fields {
        syn::Fields::Named(fields) => Some(join_tokens(
            gen_named_exprs(fields.named.iter(), transform_named, common_body)?,
            join_with,
        )),
        syn::Fields::Unnamed(fields) => Some(join_tokens(
            gen_unnamed_exprs(fields.unnamed.len(), transform_unnamed, common_body)?,
            join_with,
        )),
        syn::Fields::Unit => None,
    }
}

fn check_repr_packed(attrs: Vec<syn::Attribute>) -> bool {
    attrs.into_iter().any(|attr| {
        let syn::Meta::List(meta) = attr.meta else {
            return false;
        };

        let Some(ident) = meta.path.get_ident() else {
            return false;
        };

        if ident != "repr" {
            return false;
        }

        let Ok(ident) = syn::parse::<syn::Ident>(meta.tokens.into()) else {
            return false;
        };

        ident == "packed"
    })
}

fn gen_struct(fields: &syn::Fields, is_packed: bool) -> GenerationRet {
    let transform_named = |ident| quote!(self.#ident);
    let transform_unnamed = |index| {
        let ident = syn::Index::from(index);
        quote!(self.#ident)
    };

    let ident_mode = if is_packed {
        IdentMode::Packed
    } else {
        IdentMode::InsertRef
    };

    GenerationRet {
        extra_size: extra_size::generate(fields, transform_named, transform_unnamed, ident_mode),
        #[cfg(feature = "details")]
        details: Some(field_details::generate(
            fields,
            transform_named,
            transform_unnamed,
            ident_mode,
        )),
    }
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

fn gen_enum(variants: impl Iterator<Item = Variant>, is_packed: bool) -> GenerationRet {
    assert!(!is_packed, "repr(packed) enums are not supported!");

    let arms: TokenStream = variants
        .map(|variant| {
            gen_match_arm(
                &variant,
                extra_size::generate(
                    &variant.fields,
                    |ident| quote!(#ident),
                    |index| gen_unnamed_ident(index).to_token_stream(),
                    IdentMode::NoRef,
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

#[proc_macro_derive(TypeSize)]
pub fn typesize_derive(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput {
        attrs,
        vis: _,
        ident,
        generics,
        data,
    } = parse_macro_input!(tokens as DeriveInput);

    let is_packed = check_repr_packed(attrs);

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let bodies = match data {
        syn::Data::Struct(data) => gen_struct(&data.fields, is_packed),
        syn::Data::Enum(data) => gen_enum(data.variants.into_iter(), is_packed),
        syn::Data::Union(_) => panic!("Unions are unsupported for typesize derive!"),
    };

    let extra_size = bodies.extra_size;
    #[cfg_attr(not(feature = "details"), allow(unused_mut))]
    let mut impl_body = quote!(
        fn extra_size(&self) -> usize {
            #extra_size
        }
    );

    #[cfg(feature = "details")]
    if let Some(details) = bodies.details {
        impl_body = quote!(
            #impl_body

            fn get_size_details(&self) -> Vec<::typesize::Field> {
                #details
            }
        )
    }

    let output = quote! {
        #[automatically_derived]
        impl #impl_generics ::typesize::TypeSize for #ident #ty_generics #where_clause {
            #impl_body
        }
    };

    output.into()
}
