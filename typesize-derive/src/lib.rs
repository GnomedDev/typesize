use proc_macro2::{Ident, Punct, Spacing, TokenStream};
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, Field};

mod r#enum;
mod r#struct;

use r#enum::gen_enum;
use r#struct::gen_struct;

#[derive(Clone, Copy)]
struct SkipField(bool);

#[derive(Clone, Copy)]
pub(crate) enum PassMode {
    AsIs,
    InsertRef,
    Packed,
}

fn gen_call_with_arg(
    func_name: &TokenStream,
    arg: &TokenStream,
    pass_mode: PassMode,
) -> TokenStream {
    match pass_mode {
        PassMode::AsIs => quote!(#func_name(#arg)),
        PassMode::InsertRef => quote!(#func_name(&#arg)),
        PassMode::Packed => {
            quote!(({
                let __typesize_internal_temp = #arg;
                #func_name(&__typesize_internal_temp)
            }))
        }
    }
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

fn gen_named_exprs<'a>(
    named_fields: syn::punctuated::Iter<'a, Field>,
    transform_named: impl Fn(&'a Ident) -> TokenStream + 'a,
    common_body: impl Fn(TokenStream, TokenStream, SkipField) -> TokenStream + 'a,
) -> Option<impl ExactSizeIterator<Item = TokenStream> + 'a> {
    if named_fields.len() == 0 {
        return None;
    }

    Some(named_fields.map(move |field| {
        let ident = field.ident.as_ref().unwrap();
        let skip_field = check_extrasize_skip(&field.attrs);
        common_body(transform_named(ident), quote!(#ident), skip_field)
    }))
}

fn gen_unnamed_exprs<'a>(
    unnamed_fields: syn::punctuated::Iter<'a, Field>,
    transform_unnamed: impl Fn(usize) -> TokenStream + 'a,
    common_body: impl Fn(TokenStream, TokenStream, SkipField) -> TokenStream + 'a,
) -> Option<impl ExactSizeIterator<Item = TokenStream> + 'a> {
    if unnamed_fields.len() == 0 {
        return None;
    };

    let enumerated_iter = unnamed_fields.enumerate();
    Some(enumerated_iter.map(move |(i, field)| {
        let skip_field = check_extrasize_skip(&field.attrs);
        common_body(transform_unnamed(i), quote!(#i), skip_field)
    }))
}

fn for_each_field<'a>(
    fields: &'a syn::Fields,
    join_with: Punct,
    transform_named: impl Fn(&'a Ident) -> TokenStream + 'a,
    transform_unnamed: impl Fn(usize) -> TokenStream + 'a,
    common_body: impl Fn(TokenStream, TokenStream, SkipField) -> TokenStream + 'a,
) -> Option<TokenStream> {
    match fields {
        syn::Fields::Named(fields) => Some(join_tokens(
            gen_named_exprs(fields.named.iter(), transform_named, common_body)?,
            join_with,
        )),
        syn::Fields::Unnamed(fields) => Some(join_tokens(
            gen_unnamed_exprs(fields.unnamed.iter(), transform_unnamed, common_body)?,
            join_with,
        )),
        syn::Fields::Unit => None,
    }
}

fn extra_details_visit_fields<'a>(
    fields: &'a syn::Fields,
    transform_named: impl Fn(&'a Ident) -> TokenStream + 'a,
    transform_unnamed: impl Fn(usize) -> TokenStream + 'a,
    pass_mode: PassMode,
) -> TokenStream {
    for_each_field(
        fields,
        Punct::new('+', Spacing::Alone),
        transform_named,
        transform_unnamed,
        move |ident, _name, skip| {
            if skip.0 {
                quote!(0)
            } else {
                gen_call_with_arg(&quote!(::typesize::TypeSize::extra_size), &ident, pass_mode)
            }
        },
    )
    .unwrap_or_else(|| quote!(0_usize))
}

fn check_metalist_attr(attrs: &[syn::Attribute], expected_ident: &str, inner_ident: &str) -> bool {
    attrs.iter().any(|attr| {
        let syn::Meta::List(meta) = &attr.meta else {
            return false;
        };

        let Some(ident) = meta.path.get_ident() else {
            return false;
        };

        if ident != expected_ident {
            return false;
        }

        let Ok(ident) = syn::parse::<syn::Ident>(meta.tokens.clone().into()) else {
            return false;
        };

        ident == inner_ident
    })
}

fn check_repr_packed(attrs: &[syn::Attribute]) -> bool {
    check_metalist_attr(attrs, "repr", "packed")
}

fn check_extrasize_skip(attrs: &[syn::Attribute]) -> SkipField {
    SkipField(check_metalist_attr(attrs, "typesize", "skip"))
}

struct GenerationRet {
    extra_size: TokenStream,
    #[cfg(feature = "details")]
    details: Option<TokenStream>,
}

/// Implements `TypeSize` automatically for a `struct` or `enum`.
///
/// Use `#[typesize(skip)]` on a field to assume it does not manage any external memory.
///
/// This will avoid requiring `TypeSize` to be implemented for this field, however may lead to undercounted results if the assumption does not hold.
///
/// # Struct Mode
///
/// `TypeSize::extra_size` will be calculated by adding up the `extra_size` of all fields.
///
/// # Enum Mode
///
/// `TypeSize::extra_size` will be calculated by adding up the `extra_size` of all of the fields of the active enum variant.
///
/// # Union Mode
///
/// Unions are unsupported as there is no safe way to calculate the `extra_size`, implement `typesize::TypeSize` manually.
#[proc_macro_derive(TypeSize, attributes(typesize))]
pub fn typesize_derive(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput {
        attrs,
        vis: _,
        ident,
        generics,
        data,
    } = parse_macro_input!(tokens as DeriveInput);

    let is_packed = check_repr_packed(&attrs);

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let bodies = match data {
        syn::Data::Struct(data) => gen_struct(&data.fields, is_packed),
        syn::Data::Enum(data) => gen_enum(data.variants.into_iter(), is_packed),
        syn::Data::Union(data) => {
            return syn::Error::new(
                data.union_token.span,
                "Unions are unsupported for typesize derive.",
            )
            .into_compile_error()
            .into()
        }
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
