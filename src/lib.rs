#![recursion_limit = "512"]
#![allow(clippy::new_ret_no_self)]

#[macro_use]
extern crate async_trait;
#[macro_use]
extern crate log;

pub mod blocks;
pub mod runtime;
