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
                    Self::#ident => write!(f, "{}", #ident_str),
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

                impl std::fmt::Display for #name {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

                        impl std::fmt::Display for #name {
                           fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                               write!(f, "{}", self.0)
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
