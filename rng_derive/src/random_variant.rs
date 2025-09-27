use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

pub(super) fn derive_random_variant(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    if let Data::Enum(enum_data) = input.data {
        let enum_name = input.ident;
        let variant_count = enum_data.variants.len();

        // Check if all variants are unit variants
        if !enum_data
            .variants
            .iter()
            .all(|v| matches!(v.fields, Fields::Unit))
        {
            panic!("RandomVariant does not work on non-unit variants");
        }

        // Generate match arms
        let mut match_arms = enum_data
            .variants
            .iter()
            .enumerate()
            .map(|(idx, v)| {
                let variant_name = &v.ident;

                quote! {
                    #idx => Self::#variant_name,
                }
            })
            .collect::<Vec<_>>();

        match_arms.push(quote! {_ => panic!(),});

        let match_arms = match_arms.into_iter();

        let expanded = quote! {
            impl RandomVariant for #enum_name {
                fn random_variant(rng: &mut Rng) -> Self {
                    match rng.gen_range(0..#variant_count) {
                        #(#match_arms)*
                    }
                }
            }
        };

        // panic!("{}", expanded);

        return TokenStream::from(expanded);
    } else {
        panic!("RandomVariant must only be derived on enums");
    }
}
