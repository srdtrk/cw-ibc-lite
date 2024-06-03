#![doc = include_str!("../Readme.md")]
#![deny(missing_docs)]
#![deny(clippy::nursery, clippy::pedantic, warnings)]

use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, AttributeArgs, DataEnum, DeriveInput};

/// Adds the necessary fields to an enum such that it implements the
/// interface needed to receive callbacks from `cw-ica-controller`.
///
/// For example:
///
/// ```
/// use cw_ibc_lite_shared::types::apps::callbacks::ibc_lite_app_callback;
/// use cosmwasm_schema::cw_serde;
///
/// #[ibc_lite_app_callback]
/// #[cw_serde]
/// enum ExecuteMsg {}
/// ```
///
/// Will transform the enum to:
///
/// ```
/// enum ExecuteMsg {
///     ReceiveIcaCallback(IcaControllerCallbackMsg),
/// }
/// ```
///
/// Note that other derive macro invocations must occur after this
/// procedural macro as they may depend on the new fields. For
/// example, the following will fail because the `Clone` derivation
/// occurs before the addition of the field.
///
/// ```compile_fail
/// use cw_ibc_lite_shared::types::apps::callbacks::ibc_lite_app_callback;
/// use cosmwasm_schema::cw_serde;
///
/// #[derive(Clone)]
/// #[ibc_lite_app_callback]
/// #[allow(dead_code)]
/// #[cw_serde]
/// enum Test {
///     Foo,
///     Bar(u64),
///     Baz { foo: u64 },
/// }
/// ```
#[proc_macro_attribute]
pub fn ibc_lite_app_callback(metadata: TokenStream, input: TokenStream) -> TokenStream {
    merge_variants(
        metadata,
        input,
        quote! {
        enum Right {
            /// The application callback message from `cw-ibc-lite`.
            /// The handler for this variant should verify that this message comes from an
            /// expected legitimate source.
            ReceiveIbcAppCallback(::cw_ibc_lite_shared::types::apps::callbacks::IbcAppCallbackMsg),
        }
        }
        .into(),
    )
}

/// Merges the variants of two enums.
/// Adapted from [dao-dao-macros](https://github.com/DA0-DA0/dao-contracts/blob/bc3a44983c1bbad48d12436353a95180489143e8/packages/dao-dao-macros/src/lib.rs)
fn merge_variants(metadata: TokenStream, left: TokenStream, right: TokenStream) -> TokenStream {
    use syn::Data::Enum;

    let args = parse_macro_input!(metadata as AttributeArgs);
    if let Some(first_arg) = args.first() {
        return syn::Error::new_spanned(first_arg, "macro takes no arguments")
            .to_compile_error()
            .into();
    }

    let mut left: DeriveInput = parse_macro_input!(left);
    let right: DeriveInput = parse_macro_input!(right);

    if let (
        Enum(DataEnum { variants, .. }),
        Enum(DataEnum {
            variants: to_add, ..
        }),
    ) = (&mut left.data, right.data)
    {
        variants.extend(to_add);

        quote! { #left }.into()
    } else {
        syn::Error::new(left.ident.span(), "variants may only be added for enums")
            .to_compile_error()
            .into()
    }
}
