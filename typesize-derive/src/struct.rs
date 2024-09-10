#[cfg(feature = "details")]
use proc_macro2::{Punct, Spacing, TokenStream};
use quote::quote;
#[cfg(feature = "details")]
use syn::Ident;

use crate::{extra_details_visit_fields, GenerationRet, PassMode};

#[cfg(feature = "details")]
fn field_details_visit_fields<'a>(
    fields: &'a syn::Fields,
    transform_named: impl Fn(&'a Ident) -> TokenStream + 'a,
    transform_unnamed: impl Fn(usize) -> TokenStream + 'a,
    arg_pass_mode: PassMode,
) -> TokenStream {
    use crate::{for_each_field, gen_call_with_arg};

    let exprs = for_each_field(
        fields,
        Punct::new(',', Spacing::Alone),
        transform_named,
        transform_unnamed,
        move |ident, name, skip_field| {
            let (size_expr, collection_items_expr);
            if skip_field.0 {
                size_expr = quote!(0);
                collection_items_expr = quote!(None);
            } else {
                size_expr = gen_call_with_arg(
                    &quote!(::typesize::TypeSize::get_size),
                    &ident,
                    arg_pass_mode,
                );

                collection_items_expr = gen_call_with_arg(
                    &quote!(::typesize::TypeSize::get_collection_item_count),
                    &ident,
                    arg_pass_mode,
                )
            };

            quote!(
                ::typesize::Field {
                    name: stringify!(#name),
                    size: #size_expr,
                    collection_items: #collection_items_expr
                }
            )
        },
    )
    .unwrap_or_default();

    quote!(vec![#exprs])
}

pub(crate) fn gen_struct(fields: &syn::Fields, is_packed: bool) -> GenerationRet {
    let transform_named = |ident| quote!(self.#ident);
    let transform_unnamed = |index| {
        let ident = syn::Index::from(index);
        quote!(self.#ident)
    };

    let pass_mode = if is_packed {
        PassMode::Packed
    } else {
        PassMode::InsertRef
    };

    GenerationRet {
        extra_size: extra_details_visit_fields(
            fields,
            transform_named,
            transform_unnamed,
            pass_mode,
        ),
        #[cfg(feature = "details")]
        details: Some(field_details_visit_fields(
            fields,
            transform_named,
            transform_unnamed,
            pass_mode,
        )),
    }
}