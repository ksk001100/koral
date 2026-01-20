use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn impl_derive_from_args(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl koral::traits::FromArgs for #name {
            fn from_args(_args: &[String]) -> koral::KoralResult<Self> {
                Ok(Self::default())
            }

            fn get_subcommands() -> Vec<koral::internal::command::CommandDef> {
                <Self as koral::traits::App>::subcommands(&Self::default())
            }
        }
    };

    TokenStream::from(expanded)
}
