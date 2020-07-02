//! The IOTA Wallet Library

#![warn(missing_docs, rust_2018_idioms)]
#![allow(unused_variables, dead_code)]

/// The account manager module.
pub mod account_manager;
/// The address module.
pub mod address;
/// The storage module.
pub mod storage;
/// The transaction module.
pub mod transaction;

pub use anyhow::Result;
pub use chrono::prelude::{DateTime, Utc};