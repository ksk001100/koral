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
                cmd_defs.push(quote! {
                    koral::internal::command::CommandDef::new(#cmd_name, "").with_aliases(vec![#(#aliases.to_string()),*]),
                });
            }
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                let inner_ty = &fields.unnamed.first().unwrap().ty;
                match_arms.push(quote! {
                     s if s == #cmd_name || [#(#aliases),*].contains(&s) => {
                        let remaining_args = if args.len() > 1 { &args[1..] } else { &[] };
                        let inner = <#inner_ty as koral::traits::FromArgs>::from_args(remaining_args).ok().unwrap_or_default();
                        Ok(Self::#variant_name(inner))
                    },
                });

                // App delegators
                run_arms.push(quote! {
                    Self::#variant_name(cmd) => {
                        let mut passed_args = vec![#cmd_name.to_string()];
                        passed_args.extend(next_args);
                        cmd.run(passed_args)
                    },
                });
                run_state_arms.push(quote! {
                    Self::#variant_name(cmd) => {
                        let mut passed_args = vec![#cmd_name.to_string()];
                        passed_args.extend(next_args);
                        cmd.run_with_state(state, passed_args)
                    },
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
                flag_arms.push(quote! {
                    Self::#variant_name(cmd) => cmd.flags(),
                });

                cmd_defs.push(quote! {
                   koral::internal::command::CommandDef::new(#cmd_name, "")
                       .with_aliases(vec![#(#aliases.to_string()),*])
                       .with_subcommands(<#inner_ty as koral::traits::FromArgs>::get_subcommands())
                       .with_flags(<#inner_ty as koral::traits::App>::flags(&<#inner_ty as Default>::default())),
               });
            }
            _ => panic!("Subcommand variants must be Unit or Tuple with 1 element"),
        }
    }

    // Parse Enum attributes for name and about
    let mut app_name = name.to_string().to_lowercase();
    let mut app_about = "".to_string();

    for attr in input.attrs {
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
                                    app_name = lit.value();
                                }
                            }
                        } else if nv.path.is_ident("about") || nv.path.is_ident("description") {
                            if let Expr::Lit(expr_lit) = nv.value {
                                if let Lit::Str(lit) = expr_lit.lit {
                                    app_about = lit.value();
                                }
                            }
                        }
                    }
                }
            }
        }
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
                #app_name
            }

            fn description(&self) -> &str {
                #app_about
            }

            fn execute(&mut self, ctx: koral::Context) -> koral::KoralResult<()> {
                match self {
                    #(#execute_arms)*
                }
            }

            fn run(&mut self, args: Vec<String>) -> koral::KoralResult<()> {
                // Check if help is requested for THIS command (the Group)
                // args logic similar to App trait default run
                let help_invoked = args.iter().position(|a| a == "--help" || a == "-h");
                if let Some(h_idx) = help_invoked {
                    // If help is invoked, check if it targets a subcommand
                    // subcommands() returns the list of variants
                    let subcommands = self.subcommands();
                    // Don't skip args[0], as it might be the variant name itself (e.g. "k8s")
                    let sub_idx = args.iter().enumerate().find_map(|(i, arg)| {
                        if subcommands.iter().any(|s| s.name == *arg || s.aliases.contains(arg)) {
                            Some(i)
                        } else {
                            None
                        }
                    });

                    let should_print_help = match sub_idx {
                        Some(s_idx) => h_idx < s_idx,
                        None => true,
                    };

                    if should_print_help {
                        self.print_help();
                        return Ok(());
                    }
                }

                let next_args = if !args.is_empty() {
                    args[1..].to_vec()
                } else {
                    vec![]
                };
                match self {
                    #(#run_arms)*
                }
            }

            fn run_with_state(&mut self, state: &mut dyn std::any::Any, args: Vec<String>) -> koral::KoralResult<()> {
                // Same help logic as run
                let help_invoked = args.iter().position(|a| a == "--help" || a == "-h");
                if let Some(h_idx) = help_invoked {
                    let subcommands = self.subcommands();
                     let sub_idx = args.iter().enumerate().find_map(|(i, arg)| {
                        if subcommands.iter().any(|s| s.name == *arg || s.aliases.contains(arg)) {
                            Some(i)
                        } else {
                            None
                        }
                    });

                    let should_print_help = match sub_idx {
                        Some(s_idx) => h_idx < s_idx,
                        None => true,
                    };

                    if should_print_help {
                        self.print_help();
                        return Ok(());
                    }
                }

                let next_args = if !args.is_empty() {
                    args[1..].to_vec()
                } else {
                    vec![]
                };
                match self {
                    #(#run_state_arms)*
                }
            }

            fn subcommands(&self) -> Vec<koral::internal::command::CommandDef> {
               <Self as koral::traits::FromArgs>::get_subcommands()
            }

            fn flags(&self) -> Vec<koral::internal::flag::FlagDef> {
                // Return empty flags for Enum itself (it's a container)
                vec![]
            }
        }
    };

    TokenStream::from(expanded)
}
