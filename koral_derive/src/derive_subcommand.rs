use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Expr, Fields, Lit, Meta};

pub fn impl_derive_subcommand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let variants = match input.data {
        Data::Enum(data_enum) => data_enum.variants,
        _ => panic!("Subcommand derive only supports enums"),
    };

    let mut match_arms = Vec::new();
    let mut cmd_defs = Vec::new();

    // For App implementation
    let mut run_arms = Vec::new();
    let mut run_state_arms = Vec::new();
    let mut execute_arms = Vec::new();
    let mut name_arms = Vec::new();
    let mut flag_arms = Vec::new(); // Usually empty or delegated?
    let mut sub_arms = Vec::new();

    for variant in variants {
        let variant_name = variant.ident;
        let mut cmd_name = variant_name.to_string().to_lowercase();
        let mut aliases: Vec<String> = Vec::new();

        // Parse attributes for name override & aliases
        for attr in variant.attrs {
            if attr.path().is_ident("subcommand") {
                let nested = attr
                    .parse_args_with(
                        syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
                    )
                    .ok();
                if let Some(nested_meta) = nested {
                    for meta in nested_meta {
                        if let Meta::NameValue(nv) = meta {
                            if nv.path.is_ident("name") {
                                if let Expr::Lit(expr_lit) = nv.value {
                                    if let Lit::Str(lit) = expr_lit.lit {
                                        cmd_name = lit.value();
                                    }
                                }
                            } else if nv.path.is_ident("aliases") {
                                if let Expr::Lit(expr_lit) = nv.value {
                                    if let Lit::Str(lit) = expr_lit.lit {
                                        aliases = lit
                                            .value()
                                            .split(',')
                                            .map(|s| s.trim().to_string())
                                            .collect();
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        match variant.fields {
            Fields::Unit => {
                match_arms.push(quote! {
                    s if s == #cmd_name || [#(#aliases),*].contains(&s) => Ok(Self::#variant_name),
                });
            }
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                // If variant holds a value, that value must be an App that we can "run" or construct?
                // For `FromArgs`, we simply construct it. The App trait usually doesn't strictly imply construction from args directly unless we add another trait.
                // But typically if we match a subcommand, we want to delegate execution.
                //
                // However, `FromArgs` expects to return `Self`.
                // The variant `Remote(RemoteApp)` implies `RemoteApp` is the state.
                // `RemoteApp` should probably implement `Default` or `new`?
                // And `RemoteApp` implements `App`.
                //
                // Wait, if we return `Self`, we are returning the Enum.
                // How do we populate `RemoteApp`?
                // Usually `RemoteApp` has its own flags?
                // Strict parsing: if we see `remote`, we pass the REST of the args to `RemoteApp`?
                // But `FromArgs` returns the Enum.
                // The `RemoteApp` inside the enum usually holds the *parsed state*?
                //
                // If `RemoteApp` is just the logic container (struct Remote {}), it's usually empty or default.
                // Let's assume `Default` for now as per plan.

                match_arms.push(quote! {
                     s if s == #cmd_name || [#(#aliases),*].contains(&s) => {
                        // We strictly don't parse the INNER app here because `FromArgs` is just determining WHICH subcommand it is?
                        // Or should `FromArgs` also populate the inner app?
                        // If `RemoteApp` implements `Flag` parsing logic...
                        //
                        // Koral's `App` is stateful (flags, etc).
                        // If we return `Remote(RemoteApp)`, we assume verification/parsing happens LATER or `RemoteApp` is just initialized.
                        // Let's assume initialization via `Default`.

                        Ok(Self::#variant_name(Default::default()))
                    },
                });

                // App delegators
                run_arms.push(quote! {
                    Self::#variant_name(cmd) => cmd.run(args),
                });
                run_state_arms.push(quote! {
                    Self::#variant_name(cmd) => cmd.run_with_state(state, args),
                });
                execute_arms.push(quote! {
                    Self::#variant_name(cmd) => cmd.execute(ctx),
                });
                name_arms.push(quote! {
                    Self::#variant_name(cmd) => cmd.name(),
                });
                sub_arms.push(quote! {
                    Self::#variant_name(cmd) => cmd.subcommands(),
                });
                // Flags are tricky. Usually we want the flags of the active variant?
                // But App::flags() is static-like (called on instance).
                flag_arms.push(quote! {
                    Self::#variant_name(cmd) => cmd.flags(),
                });
            }
            _ => panic!("Subcommand variants must be Unit or Tuple with 1 element"),
        }

        cmd_defs.push(quote! {
            koral::internal::command::CommandDef::new(#cmd_name, "").with_aliases(vec![#(#aliases.to_string()),*]),
        });
    }

    let expanded = quote! {
        impl koral::traits::FromArgs for #name {
            fn from_args(args: &[String]) -> koral::KoralResult<Self> {
                if args.is_empty() {
                    return Err(koral::internal::error::KoralError::MissingArgument("No subcommand provided".to_string()));
                }

                let sub_name = &args[0];
                match sub_name.as_str() {
                    #(#match_arms)*
                    _ => Err(koral::internal::error::KoralError::InvalidFlag(format!("Unknown subcommand: {}", sub_name))),
                }
            }

            fn get_subcommands() -> Vec<koral::internal::command::CommandDef> {
                vec![
                    #(#cmd_defs)*
                ]
            }
        }

        impl koral::traits::App for #name {
            fn name(&self) -> &str {
                match self {
                    #(#name_arms)*
                }
            }

            fn execute(&mut self, ctx: koral::Context) -> koral::KoralResult<()> {
                match self {
                    #(#execute_arms)*
                }
            }

            fn run(&mut self, args: Vec<String>) -> koral::KoralResult<()> {
                match self {
                    #(#run_arms)*
                }
            }

            fn run_with_state(&mut self, state: &mut dyn std::any::Any, args: Vec<String>) -> koral::KoralResult<()> {
                match self {
                    #(#run_state_arms)*
                }
            }

            fn subcommands(&self) -> Vec<koral::internal::command::CommandDef> {
               // Usually static list of all possible subcommands
               /*
                  Note: This method is used for help generation of the *Enum itself*?
                  Or is it simply delegating?
                  If this Enum is a field in ParentApp, ParentApp calls `get_subcommands()` from `FromArgs`.

                  If we treat this Enum as the App itself, `subcommands()` should probably return all variants?
                  Same as `FromArgs::get_subcommands()`.
               */
               <Self as koral::traits::FromArgs>::get_subcommands()
            }

            fn flags(&self) -> Vec<koral::internal::flag::FlagDef> {
                // Return flags of the active variant?
                match self {
                    #(#flag_arms)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
