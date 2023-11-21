use proc_macro2::{Ident, Punct, Spacing, TokenStream};
use quote::quote;

use crate::for_each_field;

pub fn generate<'a>(
    fields: &'a syn::Fields,
    transform_named: impl Fn(&'a Option<Ident>) -> TokenStream + 'a,
    transform_unnamed: impl Fn(usize) -> TokenStream + 'a,
) -> TokenStream {
    let exprs = for_each_field(
        fields,
        Punct::new(',', Spacing::Alone),
        transform_named,
        transform_unnamed,
        |(ident, name)| {
            quote!(
                ::typesize::Field {
                    name: stringify!(#name),
                    size: ::typesize::TypeSize::get_size(#ident),
                    collection_items: ::typesize::TypeSize::get_collection_item_count(#ident)
                }
            )
        },
    )
    .unwrap_or_default();

    quote!(vec![#exprs])
}
