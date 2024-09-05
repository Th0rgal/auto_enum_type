extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Ident};

// Generates a Type enum and conversion method for the input enum
#[proc_macro_derive(TypeEnum)]
pub fn derive_type_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = &input.ident;
    let attrs = &input.attrs;

    let data = match input.data {
        Data::Enum(data) => data,
        _ => panic!("TypeEnum can only be derived for enums"),
    };

    let enum_type_name = Ident::new(&format!("{}Type", enum_name), enum_name.span());
    let variants: Vec<_> = data.variants.iter().map(|v| &v.ident).collect();

    // Determine the appropriate integer type based on the number of variants
    let repr_type = match variants.len() {
        0..=256 => quote!(u8),
        257..=65536 => quote!(u16),
        65537..=4294967296 => quote!(u32),
        _ => quote!(u64),
    };

    let expanded = quote! {
        #(#attrs)*
        #[repr(C)]
        pub enum #enum_name {
            #(#variants),*
            // ... potentially with fields
        }

        #[repr(#repr_type)]
        pub enum #enum_type_name {
            #(#variants),*
        }

        impl #enum_name {
            pub fn event_type(&self) -> #enum_type_name {
                unsafe {
                    // Transmute only the discriminant bytes
                    std::mem::transmute(*(self as *const Self as *const #repr_type))
                }
            }
        }
    };

    TokenStream::from(expanded)
}
