#![allow(clippy::collapsible_if)]
#![allow(clippy::inline_always)]
#![allow(clippy::needless_return)]
#![allow(clippy::redundant_field_names)]
#![allow(clippy::type_complexity)]
#![no_std]

extern crate smallvec;
extern crate sp_std;

mod crlf;
mod rope;
mod rope_builder;
mod slice;
mod tree;

pub mod iter;
pub mod str_utils;

pub use crate::{
  rope::Rope,
  rope_builder::RopeBuilder,
  slice::RopeSlice,
};
