# spider-macro

Procedural macros for the `spider-lib` web scraping framework.

[![crates.io](https://img.shields.io/crates/v/spider-macro.svg)](https://crates.io/crates/spider-macro)
[![docs.rs](https://docs.rs/spider-macro/badge.svg)](https://docs.rs/spider-macro)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

`spider-macro` provides procedural macros that simplify the implementation of key traits in the `spider-lib` framework. This crate contains attribute macros that reduce boilerplate code when defining scraped data structures.

## Getting Started

To use `spider-macro`, add it to your project's `Cargo.toml`:

```toml
[dependencies]
spider-lib = "0.2" # Main framework
spider-macro = "0.1" # Proc-macros for spider-lib
```

## Available Macros

### `#[scraped_item]`

The `#[scraped_item]` attribute macro automatically implements the necessary traits for a struct to be used as a `ScrapedItem` within the `spider-lib` framework. This macro eliminates the need for manual implementations of serialization, deserialization, cloning, and type conversion traits.

```rust
use spider_macro::scraped_item;

#[scraped_item]
pub struct QuoteItem {
    pub text: String,
    pub author: String,
}
```

When applied to a struct, this macro automatically adds:

- `#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]`
- An implementation of the `ScrapedItem` trait with methods for type erasure, cloning, and JSON conversion

## How It Works

The `#[scraped_item]` macro generates the following implementations:

- `serde::Serialize` and `serde::Deserialize` for serialization/deserialization
- `Clone` and `Debug` for common operations
- Implementation of the `ScrapedItem` trait with:
  - `as_any()` for type erasure
  - `box_clone()` for boxed cloning
  - `to_json_value()` for JSON conversion

This allows your structs to seamlessly integrate with spider-lib's item processing pipeline without requiring manual boilerplate code.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
