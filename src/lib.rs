//! Doctor Syn a computer algebra system for rust macros.

mod polynomial;
mod approx;
mod interpreter;

#[cfg(test)]
mod test;

use proc_macro::TokenStream;
use syn::{parse_macro_input, ExprClosure};

/// Example:
/// ```
/// use doctor_syn::approx;
/// fn sin4(x: f64) {
///     approx!(|#[domain(min=0, max=2., terms=4)] x| x.sin());
/// }
/// ```
#[proc_macro]
pub fn approx(item: TokenStream) -> TokenStream {
    let clos : ExprClosure = parse_macro_input!(item as ExprClosure);
    match approx::do_approx(clos) {
        Ok(s) => s,
        Err(s) => s,
    }
}
