//! # spider-macro
//!
//! Provides procedural macros for the `spider-lib` framework to reduce boilerplate code.
//!
//! ## Overview
//!
//! The `spider-macro` crate contains procedural macros that automate the
//! implementation of common traits and patterns used in the spider framework.
//! These macros significantly reduce the amount of boilerplate code required
//! when defining custom data structures for scraped items.
//!
//! ## Key Macros
//!
//! - **`#[scraped_item]`**: Derives the `ScrapedItem` trait along with necessary
//!   implementations for serialization, deserialization, cloning, and type
//!   conversions. This macro automatically implements all required traits for
//!   a struct to be used as a scraped item in the framework.
//!
//! ## Features
//!
//! - **Automatic Trait Derivation**: Implements `Serialize`, `Deserialize`,
//!   `Clone`, and `Debug` traits automatically
//! - **ScrapedItem Implementation**: Provides the complete implementation of
//!   the `ScrapedItem` trait required by the framework
//! - **Type Safety**: Maintains type safety while reducing boilerplate
//! - **Performance**: Generates efficient code without runtime overhead
//!
//! ## Example
//!
//! ```rust,ignore
//! use spider_macro::scraped_item;
//!
//! #[scraped_item]
//! struct Article {
//!     title: String,
//!     content: String,
//!     author: String,
//!     published_date: String,
//! }
//!
//! // The macro generates all necessary implementations automatically
//! // including serialization, deserialization, and the ScrapedItem trait
//! ```

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
        #[derive(::spider_util::serde::Serialize, ::spider_util::serde::Deserialize, Clone, Debug)]
        #ast

        impl ::spider_util::item::ScrapedItem for #name {
            fn as_any(&self) -> &dyn ::std::any::Any {
                self
            }

            fn box_clone(&self) -> Box<dyn ::spider_util::item::ScrapedItem + Send + Sync> {
                Box::new(self.clone())
            }

            fn to_json_value(&self) -> ::spider_util::serde_json::Value {
                ::spider_util::serde_json::to_value(self).unwrap()
            }
        }
    };

    TokenStream::from(expanded)
}

