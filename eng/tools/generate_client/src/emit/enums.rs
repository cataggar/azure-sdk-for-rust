// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{emit::ident, model::Package};
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::BTreeSet;

pub(super) fn render(package: &Package) -> Result<TokenStream, String> {
    let mut output = if package.enums.is_empty() {
        quote!()
    } else {
        quote! {
            use azure_core::fmt::SafeDebug;
            use serde::{Deserialize, Deserializer, Serialize, Serializer};
        }
    };
    let mut sorted: Vec<_> = package.enums.iter().collect();
    sorted.sort_by(|a, b| a.name.cmp(&b.name));
    for enumeration in sorted {
        let scope = format!("enum {}", enumeration.name);
        let name = ident(&enumeration.name, &scope)?;
        if enumeration.values.is_empty() {
            return Err(format!("{scope}: enum has no values"));
        }
        let mut sorted_values: Vec<_> = enumeration.values.iter().collect();
        sorted_values.sort_by(|a, b| a.name.cmp(&b.name));
        let mut variants = quote!();
        let mut to_wire = quote!();
        let mut from_wire = quote!();
        let mut wire_values = Vec::new();
        let mut rust_names = BTreeSet::new();
        for value in sorted_values {
            let variant = ident(&value.name, &format!("{}.{}", enumeration.name, value.name))?;
            if !rust_names.insert(variant.to_string().trim_start_matches("r#").to_owned()) {
                return Err(format!(
                    "{scope}: duplicate Rust enum variant '{}'",
                    value.name
                ));
            }
            if enumeration.extensible && value.name == "UnknownValue" {
                return Err(format!(
                    "{scope}: UnknownValue conflicts with the extensible fallback"
                ));
            }
            let wire = &value.wire_value;
            wire_values.push(wire.as_str());
            let variant_doc = format!("The {wire} value.");
            variants.extend(quote!(#[doc = #variant_doc] #variant,));
            to_wire.extend(quote!(Self::#variant => #wire,));
            from_wire.extend(quote!(#wire => Self::#variant,));
        }
        let unknown_variant = if enumeration.extensible {
            quote!(
                /// Any other wire value not defined by this enum.
                UnknownValue(String),
            )
        } else {
            quote!()
        };
        let unknown_to_wire = if enumeration.extensible {
            quote!(Self::UnknownValue(value) => value.as_str(),)
        } else {
            quote!()
        };
        let unknown_from_wire = if enumeration.extensible {
            quote!(Self::UnknownValue(value.to_owned()))
        } else {
            quote!(return Err(serde::de::Error::unknown_variant(&value, &[#(#wire_values),*])))
        };
        let documentation = format!("The {} values.", enumeration.name);
        output.extend(quote! {
            #[doc = #documentation]
            #[derive(Clone, SafeDebug, Eq, PartialEq)]
            pub enum #name {
                #variants
                #unknown_variant
            }

            impl AsRef<str> for #name {
                fn as_ref(&self) -> &str {
                    match self {
                        #to_wire
                        #unknown_to_wire
                    }
                }
            }

            impl std::fmt::Display for #name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_str(self.as_ref())
                }
            }

            impl Serialize for #name {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    serializer.serialize_str(self.as_ref())
                }
            }

            impl<'de> Deserialize<'de> for #name {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: Deserializer<'de>,
                {
                    let value = String::deserialize(deserializer)?;
                    Ok(match value.as_str() {
                        #from_wire
                        _ => #unknown_from_wire,
                    })
                }
            }
        });
    }
    Ok(output)
}
