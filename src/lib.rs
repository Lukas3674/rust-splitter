//! # Splitter
//! 
//! ### A string and slice splitter library
//! 
//! ## String Example
//! ```rust
//! use splitter::StrSplitter;
//! 
//! let sp = StrSplitter::new("bytes example", " ");
//! assert_eq!(
//!     sp.collect::<Vec<_>>(),
//!     vec!["bytes", " ", "example"],
//! );
//! ```
//! 
//! ## Slice Example
//! ```rust
//! use splitter::Splitter;
//! 
//! let sp = Splitter::new(&[1, 2, 3, 3, 4], [[2], [4]]);
//! let re: Vec<&[usize]> = vec![&[1], &[2], &[3, 3], &[4]];
//! assert_eq!(sp.collect::<Vec<_>>(), re);
//! ```
//! 
//! ## Look at the examples for more info
//! 
//! ## Features
//! - `std` - enables the standard library (currently only used with `impls` - feature)
//! - `impls` - automatically implements `Info` and `StrInfo` for usefull types from `core` and `std`
//! - `infos` - adds pre-defined usefull `Info` and `StrInfo` types
//! - `derive` - enables the [`Info`] and [`StrInfo`] derive macro
//! - `full` - enables all features
//! 

#![cfg_attr(not(feature = "std"), no_std)]

mod string;
mod splitter;

pub mod info;
pub mod separator;

pub use { string::info as str_info, string::separator as str_separator };

#[cfg(feature = "derive")]
pub use splitter_derive::*;

pub use info::Info;
pub use crate::splitter::Splitter;
pub use { string::StrInfo, string::StrSplitter };
