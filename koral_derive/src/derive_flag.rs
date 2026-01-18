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
                (quote! { #ty }, true, false)
            }
            _ => panic!("Flag derive only supports unit structs or tuple structs with 1 element"),
        },
        _ => panic!("Flag derive only supports structs"),
    };

    let short_quote = match short {
        Some(c) => quote! { Some(#c) },
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
        }
    };

    TokenStream::from(expanded)
}
