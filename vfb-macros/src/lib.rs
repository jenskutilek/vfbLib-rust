use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Meta};

/// Derive macro for VFB entry types.
///
/// Requires `#[vfb(key = <integer>, reader = "method_name")]` attribute on each variant.
/// The `reader` parameter specifies which VfbReader method to call for deserialization.
/// Optionally uses `#[serde(rename = "...")]` for JSON serialization names.
///
/// Generates:
/// - `new_from_data()` function to construct variants from binary data
/// - `key_to_variant()` function to map binary keys to variant names
/// - `variant_to_key()` function to map variant names back to keys
#[proc_macro_derive(VfbEntry, attributes(vfb, serde))]
pub fn derive_vfb_entry(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let variants = match &input.data {
        Data::Enum(data) => &data.variants,
        _ => {
            return syn::Error::new_spanned(&input, "VfbEntry can only be derived for enums")
                .to_compile_error()
                .into()
        }
    };

    let mut key_matches = Vec::new();
    let mut variant_matches = Vec::new();
    let mut new_from_data_matches = Vec::new();

    for variant in variants {
        // Find the #[vfb(key = ..., reader = "...")] attribute
        let mut vfb_key: Option<u16> = None;
        let mut reader_method: Option<String> = None;

        for attr in &variant.attrs {
            if attr.path().is_ident("vfb") {
                if let Meta::List(_meta_list) = &attr.meta {
                    attr.parse_nested_meta(|meta| {
                        if meta.path.is_ident("key") {
                            let value = meta.value()?;
                            let lit: syn::LitInt = value.parse()?;
                            vfb_key = Some(lit.base10_parse()?);
                            Ok(())
                        } else if meta.path.is_ident("reader") {
                            let value = meta.value()?;
                            let lit: syn::LitStr = value.parse()?;
                            reader_method = Some(lit.value());
                            Ok(())
                        } else {
                            Err(meta.error("expected `key` or `reader`"))
                        }
                    })
                    .ok();
                }
            }
        }

        let vfb_key = match vfb_key {
            Some(k) => k,
            None => {
                return syn::Error::new_spanned(
                    variant,
                    "VfbEntry variant requires #[vfb(key = <u16>, reader = \"...\")] attribute",
                )
                .to_compile_error()
                .into()
            }
        };

        // Find the variant name (use #[serde(rename = "...")] if present, otherwise use Rust name)
        let variant_name_str = variant
            .attrs
            .iter()
            .find_map(|attr| {
                if attr.path().is_ident("serde") {
                    if let Meta::List(meta_list) = &attr.meta {
                        if let Ok(Meta::NameValue(nv)) = meta_list.parse_args::<Meta>() {
                            if nv.path.is_ident("rename") {
                                if let syn::Expr::Lit(syn::ExprLit {
                                    lit: syn::Lit::Str(lit_str),
                                    ..
                                }) = &nv.value
                                {
                                    return Some(lit_str.value());
                                }
                            }
                        }
                    }
                }
                None
            })
            .unwrap_or_else(|| variant.ident.to_string());

        let variant_ident = &variant.ident;

        // Generate the decompile call based on the reader method or variant type
        let decompile_call = match &variant.fields {
            Fields::Unit => {
                // Unit variant - no data, return empty variant
                quote! {
                    #vfb_key => {
                        if bytes.is_empty() {
                            Ok(Some(#name::#variant_ident))
                        } else {
                            Ok(None)
                        }
                    }
                }
            }
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                if let Some(reader) = reader_method {
                    // Use the explicitly specified reader method
                    let reader_ident = syn::Ident::new(&reader, variant_ident.span());
                    quote! {
                        #vfb_key => {
                            if bytes.is_empty() {
                                Ok(None)
                            } else {
                                let mut r = crate::buffer::VfbReader::new(bytes);
                                Ok(Some(#name::#variant_ident(r.#reader_ident()?)))
                            }
                        }
                    }
                } else {
                    // Fallback to RawData for types without explicit reader
                    quote! {
                        #vfb_key => {
                            if bytes.is_empty() {
                                Ok(None)
                            } else {
                                Ok(Some(#name::#variant_ident(crate::entries::RawData(bytes.to_vec()))))
                            }
                        }
                    }
                }
            }
            _ => return syn::Error::new_spanned(
                variant,
                "VfbEntry variants must be either unit variants or have exactly one unnamed field",
            )
            .to_compile_error()
            .into(),
        };

        new_from_data_matches.push(decompile_call);

        // Generate match arms for key_to_variant and variant_to_key
        key_matches.push(quote! {
            #vfb_key => Some(stringify!(#variant_ident))
        });

        variant_matches.push(quote! {
            #variant_name_str => Some(#vfb_key)
        });
    }

    let expanded = quote! {
        impl #name {
            /// Construct a variant from binary data based on the key
            pub fn new_from_data(key: u16, bytes: &[u8]) -> Result<Option<Self>, crate::error::VfbError> {
                match key {
                    #(#new_from_data_matches),*,
                    _ => Ok(None),
                }
            }

            /// Get variant name from binary key
            pub fn key_to_variant(key: u16) -> Option<&'static str> {
                match key {
                    #(#key_matches),*,
                    _ => None,
                }
            }

            /// Get binary key from variant name
            pub fn variant_to_key(variant: &str) -> Option<u16> {
                match variant {
                    #(#variant_matches),*,
                    _ => None,
                }
            }
        }
    };

    TokenStream::from(expanded)
}
