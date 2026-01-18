use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Error, Fields};

pub fn impl_derive_flag_value(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    match input.data {
        Data::Enum(ref data) => {
            let variants = &data.variants;
            let from_str_arms = variants.iter().map(|v| {
                let ident = &v.ident;
                let ident_str = ident.to_string().to_lowercase();
                quote! {
                    #ident_str => Ok(Self::#ident),
                }
            });

            let to_string_arms = variants.iter().map(|v| {
                let ident = &v.ident;
                let ident_str = ident.to_string().to_lowercase();
                quote! {
                    Self::#ident => #ident_str.to_string(),
                }
            });

            let gen = quote! {
                impl std::str::FromStr for #name {
                    type Err = String;

                    fn from_str(s: &str) -> Result<Self, Self::Err> {
                        match s.to_lowercase().as_str() {
                            #(#from_str_arms)*
                            _ => Err(format!("Invalid value for {}: '{}'", stringify!(#name), s)),
                        }
                    }
                }

                impl ToString for #name {
                    fn to_string(&self) -> String {
                        match self {
                            #(#to_string_arms)*
                        }
                    }
                }
            };
            gen.into()
        }
        Data::Struct(ref data) => {
            match &data.fields {
                Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                    // Newtype struct (e.g., struct Foo(i32));
                    let gen = quote! {
                        impl std::str::FromStr for #name {
                            type Err = String;

                            fn from_str(s: &str) -> Result<Self, Self::Err> {
                                let inner = s.parse().map_err(|e| format!("Failed to parse {}: {}", stringify!(#name), e))?;
                                Ok(Self(inner))
                            }
                        }

                        impl ToString for #name {
                            fn to_string(&self) -> String {
                                self.0.to_string()
                            }
                        }
                    };
                    gen.into()
                },
                 _ => {
                    Error::new_spanned(
                        name,
                        "FlagValue derive for structs only supports single-field tuple structs (newtypes)",
                    )
                    .to_compile_error()
                    .into()
                }
            }
        }
        _ => Error::new_spanned(
            name,
            "FlagValue can only be derived for enums or single-field tuple structs",
        )
        .to_compile_error()
        .into(),
    }
}
