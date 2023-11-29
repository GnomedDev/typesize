use proc_macro2::{Ident, Punct, Spacing, TokenStream};
use quote::quote;

use crate::{for_each_field, IdentMode};

pub(crate) fn generate<'a>(
    fields: &'a syn::Fields,
    transform_named: impl Fn(&'a Option<Ident>) -> TokenStream + 'a,
    transform_unnamed: impl Fn(usize) -> TokenStream + 'a,
    ident_mode: IdentMode,
) -> TokenStream {
    let exprs = for_each_field(
        fields,
        Punct::new(',', Spacing::Alone),
        transform_named,
        transform_unnamed,
        move |(ident, name)| {
            let size_expr = ident_mode.transform(&quote!(::typesize::TypeSize::get_size), &ident);
            let collection_items_expr = ident_mode.transform(
                &quote!(::typesize::TypeSize::get_collection_item_count),
                &ident,
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
