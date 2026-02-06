//! Procedural macro for `spider-lib` to simplify `ScrapedItem` implementation.
//!
//! This module provides the `#[scraped_item]` attribute macro, which automatically
//! derives the necessary traits and implementations for a struct to be used as
//! a `ScrapedItem` within the `spider-lib` framework.
//!
//! By applying this macro to a struct, users can easily define their data
//! structures for scraped items, reducing boilerplate code for serialization,
//! deserialization, cloning, and type conversions required by the library's
//! item processing pipeline.

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

/// A procedural macro to derive the `ScrapedItem` trait.
#[proc_macro_attribute]
pub fn scraped_item(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as ItemStruct);
    let name = &ast.ident;

    let expanded = quote! {
        #[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
        #ast

        impl ScrapedItem for #name {
            fn as_any(&self) -> &dyn ::std::any::Any {
                self
            }

            fn box_clone(&self) -> Box<dyn ScrapedItem + Send + Sync> {
                Box::new(self.clone())
            }

            fn to_json_value(&self) -> ::serde_json::Value {
                ::serde_json::to_value(self).unwrap()
            }
        }
    };

    TokenStream::from(expanded)
}

