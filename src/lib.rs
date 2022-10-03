#![cfg_attr(not(feature = "std"), no_std)]

#![doc = include_str!("../README.md")]

mod string;
mod splitter;

pub mod info;
pub mod separator;

pub use { string::info as str_info, string::separator as str_separator };

#[cfg(feature = "derive")]
pub use splitter_derive::*;

pub use crate::info::Info;
pub use crate::splitter::Splitter;
pub use { crate::string::StrInfo, crate::string::StrSplitter };
