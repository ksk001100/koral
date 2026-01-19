use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Expr, Fields, Lit, Meta};

pub fn impl_derive_app(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let mut app_name = name.to_string().to_lowercase();
    let mut version = "0.0.0".to_string();
    let mut description = "".to_string(); // Added description
    let mut strict = false;
    let mut action_fn = None;

    let mut flag_registrations = Vec::new();
    let mut subcommand_registrations = Vec::new();
    let mut middleware_registrations = Vec::new();

    // Automatic dispatch support
    let mut dispatch_field: Option<(syn::Ident, syn::Type)> = None;

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
                            } else if nv.path.is_ident("description") {
                                // Added description parsing
                                if let Expr::Lit(expr_lit) = nv.value {
                                    if let Lit::Str(lit) = expr_lit.lit {
                                        description = lit.value();
                                    }
                                }
                            } else if nv.path.is_ident("action") {
                                // action = path::to::fn
                                if let Expr::Path(expr_path) = nv.value {
                                    action_fn = Some(expr_path.path);
                                }
                            } else if nv.path.is_ident("strict") {
                                if let Expr::Lit(expr_lit) = nv.value {
                                    if let Lit::Bool(lit) = expr_lit.lit {
                                        strict = lit.value;
                                    }
                                }
                            }
                        }
                        Meta::List(list) => {
                            if list.path.is_ident("flags") {
                                // flags(Flag1, Flag2)
                                let types = list
                                    .parse_args_with(
                                        syn::punctuated::Punctuated::<syn::Type, syn::Token![,]>::parse_terminated,
                                    )
                                    .ok();
                                if let Some(types) = types {
                                    for ty in types {
                                        flag_registrations.push(quote! {
                                            flags.push(koral::internal::flag::FlagDef::from_trait::<#ty>());
                                        });
                                    }
                                }
                            } else if list.path.is_ident("middleware") {
                                // middleware(MW1, MW2)
                                let types = list
                                    .parse_args_with(
                                        syn::punctuated::Punctuated::<syn::Type, syn::Token![,]>::parse_terminated,
                                    )
                                    .ok();
                                if let Some(types) = types {
                                    for ty in types {
                                        middleware_registrations.push(quote! {
                                            mws.push(Box::new(#ty::default()));
                                        });
                                    }
                                }
                            }
                        }

                        Meta::Path(path) => {
                            if path.is_ident("strict") {
                                strict = true;
                            }
                        }
                    }
                }
            }
        }
    }

    if let Data::Struct(data_struct) = input.data {
        if let Fields::Named(fields) = data_struct.fields {
            for field in fields.named {
                let ident = field.ident.clone().unwrap();
                let ty = field.ty;
                let mut is_subcommand = false;
                let mut is_middleware = false;
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
                                    } else if path.is_ident("middleware") {
                                        is_middleware = true;
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

                if is_middleware {
                    middleware_registrations.push(quote! {
                        mws.push(Box::new(self.#ident.clone()));
                    });
                }

                if is_subcommand {
                    // It's a subcommand provider
                    subcommand_registrations.push(quote! {
                         let _ = &self.#ident;
                         subs.extend(<#ty as koral::traits::FromArgs>::get_subcommands());
                    });

                    if dispatch_field.is_none() {
                        dispatch_field = Some((ident.clone(), ty.clone()));
                    }
                } else {
                    // Treat as state (ignore) by default.
                    // Flags must be registered via #[app(flags(...))] at the struct level.
                    flag_registrations.push(quote! {
                         let _ = &self.#ident;
                    });
                }
            }
        }
    }

    // Inject automatic dispatch logic if a subcommand field exists
    let action_impl = if let Some((sub_ident, sub_ty)) = dispatch_field {
        let user_action = if let Some(action) = action_fn {
            quote! {
                 koral::internal::handler::call_handler(#action, self, ctx)
            }
        } else {
            quote! { Ok(()) }
        };

        quote! {
            fn execute(&mut self, ctx: koral::Context) -> koral::KoralResult<()> {
                if !ctx.args.is_empty() {
                    // Try to parse subcommand
                    // Note: accessing ctx.args is safe here
                    let maybe_sub = <#sub_ty as koral::traits::FromArgs>::from_args(&ctx.args);
                    if let Ok(cmd) = maybe_sub {
                        self.#sub_ident = cmd;

                        // NOTE: FromArgs consumed the subcommand name, but likely NOT the remaining args.
                        // However, FromArgs implementation usually only looks at the first arg if it's a subcmd.
                        // We need to pass the REST.
                        // But wait, our `App::run` implementation skips the first argument (program name).
                        // So we should pass the subcommand name as the first argument to `run`.
                        // ctx.args[0] IS the subcommand name.

                        let sub_args = ctx.args.clone();

                         // We need to access state if available.
                         // ctx.state is Option<&mut dyn Any>.
                         // But we cannot move out of ctx.state if we need ctx later (which we don't if we return).
                         // However, ctx is not Copy.
                         // Because `run_with_state` takes `&mut dyn Any`, we can use `ctx.state`.
                         // But `ctx` consumes itself in `execute`.
                         // We can match on `ctx.state`.

                         match ctx.state {
                             Some(state) => {
                                 return self.#sub_ident.run_with_state(state, sub_args);
                             }
                             None => {
                                 return self.#sub_ident.run(sub_args);
                             }
                         }
                     }
                 }

                 // Fallback to user action
                 koral::internal::parser::validate_required_flags(&koral::traits::App::flags(self), &ctx.flags)?;
                 #user_action
            }
        }
    } else if let Some(action) = action_fn {
        quote! {
            fn execute(&mut self, ctx: koral::Context) -> koral::KoralResult<()> {
                koral::internal::parser::validate_required_flags(&koral::traits::App::flags(self), &ctx.flags)?;
                koral::internal::handler::call_handler(#action, self, ctx)
            }
        }
    } else {
        quote! {
            fn execute(&mut self, ctx: koral::Context) -> koral::KoralResult<()> {
                koral::internal::parser::validate_required_flags(&koral::traits::App::flags(self), &ctx.flags)?;
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

            fn description(&self) -> &str { // Added implementation
                #description
            }

            fn flags(&self) -> Vec<koral::internal::flag::FlagDef> {
                let mut flags = Vec::new();
                #(#flag_registrations)*
                flags
            }

            fn subcommands(&self) -> Vec<koral::internal::command::CommandDef> {
                let mut subs = Vec::new();
                #(#subcommand_registrations)*
                subs
            }

            fn is_strict(&self) -> bool {
                #strict
            }

            fn middlewares(&self) -> Vec<Box<dyn koral::Middleware>> {
                let mut mws: Vec<Box<dyn koral::Middleware>> = Vec::new();
                #(#middleware_registrations)*
                mws
            }

            #action_impl
        }
    };

    TokenStream::from(expanded)
}
