use proc_macro2::{Ident, Punct, Spacing, TokenStream};
use quote::quote;

use crate::{for_each_field, gen_call_with_arg, PassMode};

pub(crate) fn generate<'a>(
    fields: &'a syn::Fields,
    transform_named: impl Fn(Option<&'a Ident>) -> TokenStream + 'a,
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
