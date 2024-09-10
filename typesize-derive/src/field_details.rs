use proc_macro2::{Ident, Punct, Spacing, TokenStream};
use quote::quote;

use crate::{for_each_field, gen_call_with_arg, PassMode};

pub(crate) fn generate<'a>(
    fields: &'a syn::Fields,
    transform_named: impl Fn(Option<&'a Ident>) -> TokenStream + 'a,
    transform_unnamed: impl Fn(usize) -> TokenStream + 'a,
    arg_pass_mode: PassMode,
) -> TokenStream {
    let exprs = for_each_field(
        fields,
        Punct::new(',', Spacing::Alone),
        transform_named,
        transform_unnamed,
        move |(ident, name)| {
            let size_expr = gen_call_with_arg(
                &quote!(::typesize::TypeSize::get_size),
                &ident,
                arg_pass_mode,
            );

            let collection_items_expr = gen_call_with_arg(
                &quote!(::typesize::TypeSize::get_collection_item_count),
                &ident,
                arg_pass_mode,
            );

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
