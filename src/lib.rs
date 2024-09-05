extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Ident};

#[proc_macro_attribute]
pub fn type_enum(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let enum_name = &input.ident;
    let attrs = &input.attrs;

    let data = match input.data {
        Data::Enum(data) => data,
        _ => panic!("TypeEnum can only be used on enums"),
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
        #[repr(#repr_type)]
        pub enum #enum_name {
            #(#variants),*
        }

        #[repr(#repr_type)]
        #[derive(Eq, Hash, PartialEq)]
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
