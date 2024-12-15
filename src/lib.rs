// lib.rs

//! DAX-Rust is a library for working with Data Analysis Expressions (DAX) in Rust.
//!
//! This library provides functionality to:
//! - Create and manipulate tables with DAX data types
//! - Parse and evaluate DAX expressions
//! - Read and write data in various formats

pub mod error;
pub mod io;
// pub mod macros;
pub mod table;
pub mod types;

pub use error::DaxError;
pub use table::Table;
pub use types::Value;

