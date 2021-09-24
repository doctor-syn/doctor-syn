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
pub use bigdecimal;
pub use proc_macro2;
pub use quote;
pub use syn;

#[cfg(test)]
mod tests;

pub use bdmath::num_digits_for;
pub use error::*;
pub use expression::{Expression, Parity};
pub use name::Name;
pub use std::convert::{TryFrom, TryInto};
pub use variablelist::VariableList;
