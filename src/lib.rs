//! Doctor Syn a computer algebra system for rust macros.

pub mod error;
pub mod expression;
pub mod name;
pub mod polynomial;
pub mod transformation;
pub mod variablelist;
pub mod visitor;

mod bdmath;

// Re-export for use in macros.
#[doc(hidden)]
pub use syn;

#[cfg(test)]
mod tests;

pub use error::*;
pub use expression::{Expression, Parity};
pub use name::Name;
pub use std::convert::{TryFrom, TryInto};
pub use variablelist::VariableList;
pub use bdmath::num_digits_for;

pub trait Evaluateable:
    TryFrom<Expression, Error = error::Error>
    + TryInto<Expression, Error = error::Error>
{
}

impl Evaluateable for f64 {}

impl Evaluateable for f32 {}

// #[cfg(test)]
// mod test;

// use syn::{parse_macro_input, ExprClosure};
// use quote::quote;

// /// Example:
// /// ```
// /// use doctor_syn::approx;
// /// fn sin4(x: f64) {
// ///     approx!(|#[min="0"] #[max="2*PI"] #[terms="4"] x| x.sin());
// /// }
// /// ```
// #[proc_macro]
// pub fn approx(item: TokenStream) -> TokenStream {
//     let clos : ExprClosure = parse_macro_input!(item as ExprClosure);
//     match approx::do_approx(clos) {
//         Ok(res) => quote!(#res).into(),
//         Err(e) => { let e = format!("{:?}", e); quote!(#e).into() }
//     }
// }
