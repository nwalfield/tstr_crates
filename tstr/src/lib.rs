//! This crate provides an encoding of type-level strings as types.
//!
//! # Examples
//!
//! ### Indexing
//!
//! This example demonstrates how you can use type-level strings,
//! and the [`Index`] trait, to access fields of generic types by name.
//!
//! ```
//! use std::ops::Index;
//!
//! use tstr::{TS, ts};
//!
//! fn main(){
//!     takes_person(&Person::new("Bob".into(), "Marley".into()));
//!
//!     takes_person(&OtherPerson::new("Bob", "Marley"));
//! }
//!
//! fn takes_person<P>(pers: &P)
//! where
//!     P: Index<TS!(name), Output = str> + Index<TS!(surname), Output = str>
//! {
//!     assert_eq!(&pers[ts!(name)], "Bob");
//!     assert_eq!(&pers[ts!(surname)], "Marley");
//! }
//!
//!
//! use person::Person;
//! mod person {
//!     use std::ops::Index;
//!
//!     use tstr::TS;
//!     
//!     pub struct Person {
//!         name: String,
//!         surname: String,
//!     }
//!     
//!     impl Person {
//!         pub fn new(name: String, surname: String) -> Self {
//!             Self{name, surname}
//!         }
//!     }
//!     
//!     impl Index<TS!(name)> for Person {
//!         type Output = str;
//!         
//!         fn index(&self, _: TS!(name)) -> &str {
//!             &self.name
//!         }
//!     }
//!    
//!     impl Index<TS!(surname)> for Person {
//!         type Output = str;
//!         
//!         fn index(&self, _: TS!(surname)) -> &str {
//!             &self.surname
//!         }
//!     }
//! }
//!
//! use other_person::OtherPerson;
//! mod other_person {
//!     use std::ops::Index;
//!
//!     use tstr::TS;
//!     
//!     pub struct OtherPerson {
//!         name: &'static str,
//!         surname: &'static str,
//!     }
//!     
//!     impl OtherPerson {
//!         pub fn new(name: &'static str, surname: &'static str) -> Self {
//!             Self{name, surname}
//!         }
//!     }
//!     
//!     impl Index<TS!(name)> for OtherPerson {
//!         type Output = str;
//!         
//!         fn index(&self, _: TS!(name)) -> &str {
//!             self.name
//!         }
//!     }
//!    
//!     impl Index<TS!(surname)> for OtherPerson {
//!         type Output = str;
//!         
//!         fn index(&self, _: TS!(surname)) -> &str {
//!             self.surname
//!         }
//!     }
//! }
//!
//! ```
//!
//! # Macro expansion
//!
//! This library reserves the right to change how it represent type-level strings internally
//! in every single release, and cargo feature combination.
//!
//! This only affects you if you expand the code generated by macros from this crate,
//! and then use that expanded code instead of going through the macros.
//!
//! # Cargo features
//!
//! - `"rust_1_46"`:
//! Enables const functions in [`tstr::utils`] for comparing `&str` and `&[u8]`.
//!
//! - `"cmp_traits"`: Enables the traits for comparing type-level strings.
//!
//! - `"use_syn"`:
//! Changes how literals passed to the macros of this crate are parsed to use the `syn` crate.
//! Use this if there is some literal that could not be
//! parsed but is a valid str/integer literal.
//!
//! - `"min_const_generics"`:
//! changes the representation of type-level strings to use many `char` const parameter,
//! making for better compiler errors for non-alphanumeric-ascii strings.
//! Requires Rust 1.51.0.
//!
//! - `"const_generics"`:
//! changes the representation of type-level strings to use a `&'static str` const parameter,
//! making for better compiler errors, and a few more features.
//! Requires `&'static str` to be stably usable as const parameters.
//!
//! - `"nightly_const_generics"`: Equivalent to the `"const_generics"` feature,
//! but enables the nightly compiler features to use `&'static str` const parameters.
//!
//! - `"for_examples"`: Enables the `for_examples` module,
//! with a few types used in documentation examples.
//!
//! # No-std support
//!
//! This crate is unconditionally `#![no_std]`, and can be used anywhere that Rust can be.
//!
//! # Minimum Supported Rust Version
//!
//! This crate supports Rust versions back to Rust 1.40.0.
//!
//! [`Index`]: https://doc.rust-lang.org/std/ops/trait.Index.html
//! [`tstr::utils`]: ./utils/index.html
#![no_std]
#![cfg_attr(feature = "nightly_const_generics", feature(adt_const_params))]
#![cfg_attr(feature = "docsrs", feature(doc_cfg))]
#![allow(non_camel_case_types)]
#![cfg_attr(feature = "nightly_const_generics", allow(incomplete_features))]

#[cfg(feature = "for_examples")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "for_examples")))]
pub mod for_examples;

#[cfg(not(feature = "const_generics"))]
#[cfg(feature = "cmp_traits")]
mod for_tupled_reprs;

pub mod asserts;

mod macros;
mod make_tstr;
mod to_uint;
mod tstr_type;

#[cfg(feature = "cmp_traits")]
mod tstr_cmp;

pub mod utils;

#[doc(hidden)]
extern crate self as tstr;

#[doc(hidden)]
pub use tstr_proc_macros::__ts_impl;

pub use crate::{asserts::Assert, make_tstr::MakeTStr, to_uint::ToUint, tstr_type::TStr};

#[cfg(feature = "cmp_traits")]
pub use tstr_cmp::TStrEq;

#[cfg(all(feature = "cmp_traits", feature = "const_generics"))]
pub use tstr_cmp::TStrOrd;

#[cfg_attr(feature = "docsrs", doc(cfg(feature = "const_generics")))]
#[cfg(feature = "const_generics")]
pub use crate::tstr_type::StrValue;

include! {"./p.rs"}
