//! Doctor Syn a computer algebra system for rust macros.

pub mod error;
pub mod polynomial;
pub mod transformation;
pub mod visitor;
// pub mod util;
pub mod expression;
pub mod name;
pub mod variablelist;

#[cfg(test)]
mod tests;

pub use expression::Expression;
pub use name::Name;
pub use variablelist::VariableList;

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
