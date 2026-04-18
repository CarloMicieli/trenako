//! # Catalog
//! The Catalog module includes all types and functions to handle model railway catalogs.
//!
//! The list of supported types includes:
//!
//! - manufacturers
//! - catalog items and rolling stocks
//! - railway companies
//! - modeling scales

pub mod manufacturers;
pub mod catalog_items;
pub mod common;
pub mod railways;
pub mod scales;

#[cfg(test)]
mod test_helpers;

#[macro_use]
extern crate serde_derive;
