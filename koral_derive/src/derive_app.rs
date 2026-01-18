use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Expr, Fields, Lit, Meta};

pub fn impl_derive_app(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let mut app_name = name.to_string().to_lowercase();
    let mut version = "0.0.0".to_string();
    let mut action_fn = None;

    // Parse attributes
    for attr in input.attrs {
        if attr.path().is_ident("app") {
            let nested = attr
                .parse_args_with(
                    syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
                )
                .ok();
            if let Some(nested_meta) = nested {
                for meta in nested_meta {
                    match meta {
                        Meta::NameValue(nv) => {
                            if nv.path.is_ident("name") {
                                if let Expr::Lit(expr_lit) = nv.value {
                                    if let Lit::Str(lit) = expr_lit.lit {
                                        app_name = lit.value();
                                    }
                                }
                            } else if nv.path.is_ident("version") {
                                if let Expr::Lit(expr_lit) = nv.value {
                                    if let Lit::Str(lit) = expr_lit.lit {
                                        version = lit.value();
                                    }
                                }
                            } else if nv.path.is_ident("action") {
                                // action = path::to::fn
                                if let Expr::Path(expr_path) = nv.value {
                                    action_fn = Some(expr_path.path);
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    let mut flag_registrations = Vec::new();
    let mut subcommand_registrations = Vec::new();

    if let Data::Struct(data_struct) = input.data {
        if let Fields::Named(fields) = data_struct.fields {
            for field in fields.named {
                let ident = field.ident.clone().unwrap();
                let ty = field.ty;
                let mut is_subcommand = false;
                let mut ignore = false;

                for attr in field.attrs {
                    if attr.path().is_ident("app") {
                        let nested = attr.parse_args_with(syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated).ok();
                        if let Some(nested_meta) = nested {
                            for meta in nested_meta {
                                if let Meta::Path(path) = meta {
                                    if path.is_ident("subcommand") {
                                        is_subcommand = true;
                                    } else if path.is_ident("ignore") || path.is_ident("skip") {
                                        ignore = true;
                                    }
                                }
                            }
                        }
                    }
                }

                if ignore {
                    flag_registrations.push(quote! {
                        let _ = &self.#ident;
                    });
                    continue;
                }

                if is_subcommand {
                    // It's a subcommand provider (e.g. strict implementing FromArgs + get_subcommands)
                    // We assume it implements FromArgs which has get_subcommands()
                    subcommand_registrations.push(quote! {
                         let _ = &self.#ident;
                         subs.extend(<#ty as koral::traits::FromArgs>::get_subcommands());
                    });
                } else {
                    // Assume it's a flag
                    flag_registrations.push(quote! {
                        let _ = &self.#ident;
                        flags.push(koral::flag::FlagDef::from_trait::<#ty>());
                    });
                }
            }
        }
    }

    let action_impl = if let Some(action) = action_fn {
        quote! {
            fn execute(&mut self, ctx: koral::Context) -> koral::KoralResult<()> {
                koral::handler::call_handler(#action, self, ctx)
            }
        }
    } else {
        quote! {
            fn execute(&mut self, _ctx: koral::Context) -> koral::KoralResult<()> {
                Ok(())
            }
        }
    };

    let expanded = quote! {
        impl koral::traits::App for #name {
            fn name(&self) -> &str {
                #app_name
            }

            fn version(&self) -> &str {
                #version
            }

            fn flags(&self) -> Vec<koral::flag::FlagDef> {
                let mut flags = Vec::new();
                #(#flag_registrations)*
                flags
            }

            fn subcommands(&self) -> Vec<koral::command::CommandDef> {
                let mut subs = Vec::new();
                #(#subcommand_registrations)*
                subs
            }

            #action_impl
        }
    };

    TokenStream::from(expanded)
}
