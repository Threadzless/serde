//! This crate provides Serde's two derive macros.
//!
//! ```edition2021
//! # use serde_derive::{Deserialize, Serialize};
//! #
//! #[derive(Serialize, Deserialize)]
//! # struct S;
//! #
//! # fn main() {}
//! ```
//!
//! Please refer to [https://serde.rs/derive.html] for how to set this up.
//!
//! [https://serde.rs/derive.html]: https://serde.rs/derive.html

#![doc(html_root_url = "https://docs.rs/serde_derive/1.0.172-alpha.0")]
#![allow(unknown_lints, bare_trait_objects)]

extern crate proc_macro;

#[cfg(not(all(target_arch = "x86_64", target_os = "linux", target_env = "gnu")))]
include!("lib_from_source.rs");

#[cfg(all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"))]
include!("lib_precompiled.rs");
