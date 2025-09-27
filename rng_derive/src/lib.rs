mod random_variant;

use proc_macro::TokenStream;

#[proc_macro_derive(RandomVariant)]
pub fn derive_random_variant(input: TokenStream) -> TokenStream {
    random_variant::derive_random_variant(input)
}
