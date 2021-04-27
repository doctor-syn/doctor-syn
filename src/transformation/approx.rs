use crate::error::{Error, Result};
use crate::polynomial::Polynomial;
use crate::{Evaluateable, Expression, Name, VariableList};
use proc_macro2::TokenStream;
use quote::quote;
use std::convert::TryInto;
use syn::{parse_quote, Expr};

pub fn approx<T: Evaluateable>(
    expr: &Expression,
    num_terms: usize,
    xmin: T,
    xmax: T,
    variable: Name,
) -> Result<Expression> {
    let err_fn = || Error::CouldNotEvaulate(expr.span());
    use std::f64::consts::PI;
    let a = (xmax + xmin).to_f64().ok_or_else(err_fn)? * 0.5;
    let b = PI / (num_terms - 1) as f64;
    let c = (xmax - xmin).to_f64().ok_or_else(err_fn)? * 0.5;
    let mut xvalues = Vec::new();
    let mut yvalues = Vec::new();
    for i in 0..num_terms {
        // *almost* Chebyshev nodes.
        let x = a - c * (i as f64 * b).cos();
        let mut vars = VariableList::new();
        vars.add_var(variable.clone(), x.try_into()?);
        let subst = expr.subst(vars)?;
        let y: f64 = subst.eval()?;
        xvalues.push(x);
        yvalues.push(y);
    }

    let poly = Polynomial::from_points(xvalues.as_slice(), yvalues.as_slice());
    let k = num_terms;
    let terms = poly.terms();
    let highest_coeff = terms[k - 1];
    let mul_adds: Vec<TokenStream> = (0..k - 1)
        .rev()
        .map(|i| {
            let ti = terms[i];
            quote!(mul_add(x, #ti))
        })
        .collect();

    let expr: Expr = parse_quote!( #highest_coeff #( .#mul_adds )* );

    Ok(expr.into())
}
