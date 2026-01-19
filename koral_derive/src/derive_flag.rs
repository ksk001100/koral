use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Expr, Fields, Lit, Meta};

pub fn impl_derive_flag(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    // Default values
    let mut flag_name = name.to_string().to_lowercase().replace("flag", "");
    let mut short = None;
    let mut help = String::new();
    let mut default_val: Option<String> = None;
    let mut env_var: Option<String> = None;
    let mut validator: Option<syn::Path> = None;
    let mut aliases: Vec<String> = Vec::new();
    let mut required = false;

    // Parse attributes
    for attr in input.attrs {
        if attr.path().is_ident("flag") {
            let nested = attr
                .parse_args_with(
                    syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
                )
                .unwrap();
            for meta in nested {
                match meta {
                    Meta::NameValue(nv) => {
                        if nv.path.is_ident("name") {
                            if let Expr::Lit(expr_lit) = nv.value {
                                if let Lit::Str(lit) = expr_lit.lit {
                                    flag_name = lit.value();
                                }
                            }
                        } else if nv.path.is_ident("short") {
                            if let Expr::Lit(expr_lit) = nv.value {
                                if let Lit::Char(lit) = expr_lit.lit {
                                    short = Some(lit.value());
                                }
                            }
                        } else if nv.path.is_ident("help") {
                            if let Expr::Lit(expr_lit) = nv.value {
                                if let Lit::Str(lit) = expr_lit.lit {
                                    help = lit.value();
                                }
                            }
                        } else if nv.path.is_ident("default") {
                            if let Expr::Lit(expr_lit) = nv.value {
                                if let Lit::Str(lit) = expr_lit.lit {
                                    default_val = Some(lit.value());
                                }
                            }
                        } else if nv.path.is_ident("env") {
                            if let Expr::Lit(expr_lit) = nv.value {
                                if let Lit::Str(lit) = expr_lit.lit {
                                    env_var = Some(lit.value());
                                }
                            }
                        } else if nv.path.is_ident("validator") {
                            if let Expr::Path(expr_path) = nv.value {
                                validator = Some(expr_path.path);
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
                        } else if nv.path.is_ident("required") {
                            if let Expr::Lit(expr_lit) = nv.value {
                                if let Lit::Bool(lit) = expr_lit.lit {
                                    required = lit.value;
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    // Determine value type and takes_value
    let (value_type, takes_value, _is_bool) = match input.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Unit => (quote! { bool }, false, true),
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                let ty = &fields.unnamed.first().unwrap().ty;
                let is_bool = if let syn::Type::Path(p) = ty {
                    p.path.is_ident("bool")
                } else {
                    false
                };
                (quote! { #ty }, !is_bool, is_bool)
            }
            _ => panic!("Flag derive only supports unit structs or tuple structs with 1 element"),
        },
        _ => panic!("Flag derive only supports structs"),
    };

    if default_val.is_none() && _is_bool {
        default_val = Some("false".to_string());
    }

    let short_quote = match short {
        Some(c) => quote! { Some(#c) },
        None => quote! { None },
    };

    let env_quote = match env_var {
        Some(e) => quote! { Some(#e) },
        None => quote! { None },
    };

    let validator_quote = match validator {
        Some(v) => quote! { Some(#v) },
        None => quote! { None },
    };

    let default_impl = if let Some(d) = default_val {
        quote! {
            fn default_value() -> Option<Self::Value> {
                // We rely on FromStr for the value type
                 <#value_type as std::str::FromStr>::from_str(#d).ok()
            }
        }
    } else {
        quote! {} // Default is None
    };

    let expanded = quote! {
        impl koral::Flag for #name {
            type Value = #value_type;

            fn name() -> &'static str {
                #flag_name
            }

            fn short() -> Option<char> {
                #short_quote
            }

            fn help() -> &'static str {
                #help
            }

            fn takes_value() -> bool {
                #takes_value
            }

            #default_impl

            fn env() -> Option<&'static str> {
                #env_quote
            }

            fn validator() -> Option<fn(&str) -> Result<(), String>> {
                #validator_quote
            }

            fn aliases() -> Vec<&'static str> {
                vec![#(#aliases),*]
            }

            fn required() -> bool {
                #required
            }
        }
    };

    TokenStream::from(expanded)
}
