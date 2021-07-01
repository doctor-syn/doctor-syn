//! Doctor Syn a computer algebra system for rust macros.

pub mod error;
pub mod expression;
pub mod name;
pub mod polynomial;
pub mod transformation;
pub mod variablelist;
pub mod visitor;

pub mod bdmath;

// Re-export for use in macros.
pub use syn;
pub use quote;
pub use bigdecimal;
pub use proc_macro2;

#[cfg(test)]
mod tests;

pub use error::*;
pub use expression::{Expression, Parity};
pub use name::Name;
pub use std::convert::{TryFrom, TryInto};
pub use variablelist::VariableList;
pub use bdmath::num_digits_for;

