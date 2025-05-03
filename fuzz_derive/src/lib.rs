use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(RandomInstance)]
pub fn derive_random_instance(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = match input.data {
        syn::Data::Struct(data) => impl_random_instance_struct(&name, &data),
        syn::Data::Enum(data) => impl_random_instance_enum(&name, &data),
        syn::Data::Union(_) => panic!("RandomInstance derive not supported for unions"),
    };

    // panic!("{}", TokenStream::from(expanded));

    TokenStream::from(expanded)
}

fn impl_random_instance_struct(
    name: &syn::Ident,
    data: &syn::DataStruct,
) -> proc_macro2::TokenStream {
    let fields = match &data.fields {
        syn::Fields::Named(fields) => &fields.named,
        syn::Fields::Unnamed(fields) => &fields.unnamed,
        syn::Fields::Unit => {
            // For unit structs
            return quote! {
                impl RandomInstance for #name {
                    fn random_instance(rng: &mut Rng) -> Self {
                        #name
                    }
                }
            };
        }
    };

    // Generate field initializers
    let initializers = fields.iter().map(|field| {
        let field_name = field.ident.as_ref();
        let field_type = field.ty.clone();

        quote! {
            #field_name: #field_type::random_instance(Rng::new())
        }
    });

    quote! {
        impl RandomInstance for #name {
            fn random_instance(rng: &mut Rng) -> Self {
                #name {
                    #(#initializers,)*
                }
            }
        }
    }
}

fn impl_random_instance_enum(name: &syn::Ident, data: &syn::DataEnum) -> proc_macro2::TokenStream {
    let variants = &data.variants;

    // Generate match arms for each variant
    let arms = variants.iter().enumerate().map(|(idx, variant)| {
        let variant_name = &variant.ident;

        // Handle both named and unnamed fields
        match &variant.fields {
            syn::Fields::Named(fields) => {
                let field_names = fields.named.iter().map(|f| &f.ident);
                let field_types = fields.named.iter().map(|f| &f.ty);

                quote! {
                    #idx => #name::#variant_name {
                        #(#field_names: #field_types::random_instance(Rng::new()),)*
                    }
                }
            }
            syn::Fields::Unnamed(fields) => {
                let field_types = fields.unnamed.iter().map(|f| &f.ty);

                quote! {
                    #idx => #name::#variant_name (#(#field_types::random_instance(Rng::new()),)*)
                }
            }
            syn::Fields::Unit => {
                quote! {
                    #idx => #name::#variant_name
                }
            }
        }
    });

    let variant_count = variants.len();

    quote! {
        impl RandomInstance for #name {
            fn random_instance(rng: &mut Rng) -> Self {
                match rng.gen_range(0..#variant_count) {
                    #(#arms,)*
                    _ => unreachable!(),
                }
            }
        }
    }
}
