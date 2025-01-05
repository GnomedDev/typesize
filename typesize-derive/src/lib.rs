use proc_macro2::{Ident, Punct, Spacing, TokenStream};
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, Field};

mod r#enum;
mod r#struct;

use r#enum::gen_enum;
use r#struct::gen_struct;

#[derive(Clone)]
enum FieldConfig {
    Default,
    Skip,
    With(syn::Path),
}

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

fn try_join_tokens(
    exprs: impl ExactSizeIterator<Item = syn::Result<impl ToTokens>>,
    sep: impl ToTokens,
) -> syn::Result<TokenStream> {
    let expr_count = exprs.len();
    let mut out_tokens = TokenStream::new();
    for (i, expr) in exprs.enumerate() {
        expr?.to_tokens(&mut out_tokens);
        if expr_count != i + 1 {
            sep.to_tokens(&mut out_tokens);
        }
    }

    Ok(out_tokens)
}

fn gen_named_exprs<'a>(
    named_fields: syn::punctuated::Iter<'a, Field>,
    transform_named: impl Fn(&'a Ident) -> TokenStream + 'a,
    common_body: impl Fn(TokenStream, TokenStream, FieldConfig) -> TokenStream + 'a,
) -> Option<impl ExactSizeIterator<Item = syn::Result<TokenStream>> + 'a> {
    if named_fields.len() == 0 {
        return None;
    }

    Some(named_fields.map(move |field| {
        let ident = field.ident.as_ref().unwrap();
        let field_config = get_field_config(&field.attrs)?;
        Ok(common_body(
            transform_named(ident),
            quote!(#ident),
            field_config,
        ))
    }))
}

fn gen_unnamed_exprs<'a>(
    unnamed_fields: syn::punctuated::Iter<'a, Field>,
    transform_unnamed: impl Fn(usize) -> TokenStream + 'a,
    common_body: impl Fn(TokenStream, TokenStream, FieldConfig) -> TokenStream + 'a,
) -> Option<impl ExactSizeIterator<Item = syn::Result<TokenStream>> + 'a> {
    if unnamed_fields.len() == 0 {
        return None;
    };

    let enumerated_iter = unnamed_fields.enumerate();
    Some(enumerated_iter.map(move |(i, field)| {
        let field_config = get_field_config(&field.attrs)?;
        Ok(common_body(transform_unnamed(i), quote!(#i), field_config))
    }))
}

fn for_each_field<'a>(
    fields: &'a syn::Fields,
    join_with: Punct,
    transform_named: impl Fn(&'a Ident) -> TokenStream + 'a,
    transform_unnamed: impl Fn(usize) -> TokenStream + 'a,
    common_body: impl Fn(TokenStream, TokenStream, FieldConfig) -> TokenStream + 'a,
) -> Option<syn::Result<TokenStream>> {
    match fields {
        syn::Fields::Named(fields) => Some(try_join_tokens(
            gen_named_exprs(fields.named.iter(), transform_named, common_body)?,
            join_with,
        )),
        syn::Fields::Unnamed(fields) => Some(try_join_tokens(
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
) -> syn::Result<TokenStream> {
    for_each_field(
        fields,
        Punct::new('+', Spacing::Alone),
        transform_named,
        transform_unnamed,
        move |ident, _name, config| match config {
            FieldConfig::Skip => quote!(0),
            FieldConfig::Default => {
                gen_call_with_arg(&quote!(::typesize::TypeSize::extra_size), &ident, pass_mode)
            }
            FieldConfig::With(fn_path) => {
                gen_call_with_arg(&fn_path.into_token_stream(), &ident, pass_mode)
            }
        },
    )
    .unwrap_or_else(|| Ok(quote!(0_usize)))
}

fn check_repr_packed(attrs: &[syn::Attribute]) -> bool {
    fn is_valid_repr_for_packed(ident: &syn::Ident) -> bool {
        // "packed may only be applied to the Rust and C representations."
        // https://doc.rust-lang.org/reference/type-layout.html#the-alignment-modifiers
        ident == "C" || ident == "Rust"
    }

    struct CheckIsPacked(bool);
    impl syn::parse::Parse for CheckIsPacked {
        fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
            let first_token = input.parse::<syn::Ident>()?;
            if !input.peek(syn::Token![,]) {
                let is_packed = first_token == "packed";
                return Ok(Self(is_packed));
            }

            input.parse::<syn::Token![,]>()?;

            let second_token = input.parse::<syn::Ident>()?;
            if is_valid_repr_for_packed(&first_token) && second_token == "packed" {
                return Ok(Self(true));
            }

            if first_token == "packed" && is_valid_repr_for_packed(&second_token) {
                return Ok(Self(true));
            }

            Ok(Self(false))
        }
    }

    attrs.iter().any(|attr| {
        let syn::Meta::List(meta) = &attr.meta else {
            return false;
        };

        let Some(ident) = meta.path.get_ident() else {
            return false;
        };

        if ident != "repr" {
            return false;
        }

        syn::parse2::<CheckIsPacked>(meta.tokens.clone()).unwrap().0
    })
}

fn get_field_config(attrs: &[syn::Attribute]) -> syn::Result<FieldConfig> {
    // based on
    // https://docs.rs/syn/latest/syn/macro.custom_keyword.html#example

    mod kw {
        syn::custom_keyword!(skip);
        syn::custom_keyword!(with);
    }

    enum Input {
        Skip {
            _skip: kw::skip,
        },
        With {
            _with: kw::with,
            _eq: syn::Token![=],
            path: syn::Path,
        },
    }

    impl syn::parse::Parse for Input {
        fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
            let lookahead = input.lookahead1();
            if lookahead.peek(kw::skip) {
                Ok(Self::Skip {
                    _skip: input.parse()?,
                })
            } else if lookahead.peek(kw::with) {
                Ok(Self::With {
                    _with: input.parse()?,
                    _eq: input.parse()?,
                    path: input.parse()?,
                })
            } else {
                Err(lookahead.error())
            }
        }
    }

    for attr in attrs {
        let syn::Meta::List(meta) = &attr.meta else {
            continue;
        };

        let Some(path) = meta.path.get_ident() else {
            continue;
        };

        if path != "typesize" {
            continue;
        }

        let input = syn::parse::<Input>(meta.tokens.clone().into())?;
        return Ok(match input {
            Input::Skip { .. } => FieldConfig::Skip,
            Input::With { path, .. } => FieldConfig::With(path),
        });
    }

    Ok(FieldConfig::Default)
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
/// Use `#[typesize(with = path::to::fn)]` on a field to specify a custom `extra_size` for that field.
/// The function accepts a reference to the type of the field.
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
        syn::Data::Union(data) => Err(syn::Error::new(
            data.union_token.span,
            "Unions are unsupported for typesize derive.",
        )),
    };

    let bodies = match bodies {
        Ok(bodies) => bodies,
        Err(err) => {
            return err.into_compile_error().into();
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
        );
    }

    let output = quote! {
        #[automatically_derived]
        impl #impl_generics ::typesize::TypeSize for #ident #ty_generics #where_clause {
            #impl_body
        }
    };

    output.into()
}
