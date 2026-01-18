use proc_macro::TokenStream;

mod derive_app;
mod derive_flag;
mod derive_subcommand;

#[proc_macro_derive(Flag, attributes(flag))]
pub fn derive_flag(input: TokenStream) -> TokenStream {
    derive_flag::impl_derive_flag(input)
}

#[proc_macro_derive(Subcommand, attributes(subcommand))]
pub fn derive_subcommand(input: TokenStream) -> TokenStream {
    derive_subcommand::impl_derive_subcommand(input)
}

#[proc_macro_derive(App, attributes(app))]
pub fn derive_app(input: TokenStream) -> TokenStream {
    derive_app::impl_derive_app(input)
}
