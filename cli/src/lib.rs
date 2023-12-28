#![doc(
  html_logo_url = "https://raw.githubusercontent.com/rainlanguage/rainlang-vscode/master/docs/images/rain-logo-icon.svg",
  html_favicon_url = "https://raw.githubusercontent.com/rainlanguage/rainlang-vscode/master/docs/images/rain-logo-icon.svg"
)]
//! # Rain Metadata Tooling
//! 
//! A library that provides all the toolings and utilities in order to work with RainLanguage metadata. 
//! Dotrain LSP/compiler, Rain Orderbook, etc are a few examples to mention that use this library under the hood.
//!
//! Also provides CLI app (executable binary) to generate desireable Rain cbor encoded metadata based on [Metadata Specs](https://github.com/rainprotocol/specs/blob/main/metadata-v1.md) 
//! which for example is used in Rain deployment CI.

pub(crate) mod solc;
pub(crate) mod meta;
pub(crate) mod subgraph;

#[cfg(feature = "cli")]
pub mod cli;

// re-export main types and functionalities
pub use solc::*;
pub use meta::*;
pub use subgraph::*;
