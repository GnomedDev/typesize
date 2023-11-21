use proc_macro2::{Ident, Punct, Spacing, TokenStream};
use quote::quote;

use crate::for_each_field;

pub fn generate<'a>(
    fields: &'a syn::Fields,
    transform_named: impl Fn(&'a Option<Ident>) -> TokenStream + 'a,
    transform_unnamed: impl Fn(usize) -> TokenStream + 'a,
) -> TokenStream {
    for_each_field(
        fields,
        Punct::new('+', Spacing::Alone),
        transform_named,
        transform_unnamed,
        |(ident, _)| quote!(::typesize::TypeSize::extra_size(#ident)),
    )
    .unwrap_or_else(|| quote!(0_usize))
}
