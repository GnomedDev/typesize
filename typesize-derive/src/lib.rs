use proc_macro2::{Ident, Punct, Spacing, TokenStream};
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, Field};

mod r#enum;
mod r#struct;

use r#enum::gen_enum;
use r#struct::gen_struct;

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
    named_fields: impl ExactSizeIterator<Item = &'a Field> + 'a,
    transform_named: impl Fn(&'a Ident) -> TokenStream + 'a,
    common_body: impl Fn((TokenStream, TokenStream)) -> TokenStream + 'a,
) -> Option<impl ExactSizeIterator<Item = TokenStream> + 'a> {
    if named_fields.len() == 0 {
        return None;
    }

    Some(
        named_fields
            .map(|field| field.ident.as_ref().unwrap())
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
    transform_named: impl Fn(&'a Ident) -> TokenStream + 'a,
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
        move |(ident, _)| {
            gen_call_with_arg(&quote!(::typesize::TypeSize::extra_size), &ident, pass_mode)
        },
    )
    .unwrap_or_else(|| quote!(0_usize))
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

struct GenerationRet {
    extra_size: TokenStream,
    #[cfg(feature = "details")]
    details: Option<TokenStream>,
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
