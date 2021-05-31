use crate::error::{Error, Result};
use crate::polynomial::Polynomial;
use crate::{Evaluateable, Expression, Name, VariableList, Parity};
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
    parity: Parity,
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
        // println!("x={:16} y={:16} s={:16}", x, y, x.sin());
        xvalues.push(x);
        yvalues.push(y);
    }

    let poly = Polynomial::from_points(xvalues.as_slice(), yvalues.as_slice());

    let k = num_terms;
    let terms = poly.terms();
    // println!("terms={:?}", terms);

    let expr: Expr = match parity {
        Parity::Odd => {
            let highest_coeff = terms[k - 1];
            if k % 2 != 0 {
                return Err(Error::WrongNumberOfTerms(expr.span()))
            }
            let mul_adds: Vec<TokenStream> = (1..k - 1).step_by(2)
                .rev()
                .map(|i| {
                    let ti = terms[i];
                    quote!(mul_add(x*x, #ti))
                })
                .collect();
            parse_quote!( (#highest_coeff) #( .#mul_adds )* * x )
        }
        Parity::Even => {
            let highest_coeff = terms[k - 1];
            if k % 2 == 0 {
                return Err(Error::WrongNumberOfTerms(expr.span()))
            }
            let mul_adds: Vec<TokenStream> = (0..k - 1).step_by(2)
                .rev()
                .map(|i| {
                    let ti = terms[i];
                    quote!(mul_add(x*x, #ti))
                })
                .collect();
            parse_quote!( (#highest_coeff) #( .#mul_adds )* )
        }
        Parity::Neither => {
            let highest_coeff = terms[k - 1];
            let mul_adds: Vec<TokenStream> = (0..k - 1)
                .rev()
                .map(|i| {
                    let ti = terms[i];
                    quote!(mul_add(x, #ti))
                })
                .collect();
            parse_quote!( (#highest_coeff) #( .#mul_adds )* )
        }
    };

    // println!("zero error points: -----------------");
    // let e : Expression = expr.clone().into();
    // for (x, y) in xvalues.iter().cloned().zip(yvalues) {
    //     let mut vars = VariableList::new();
    //     vars.add_var(variable.clone(), x.try_into()?);
    //     let subst = e.subst(vars)?;
    //     let yp: f64 = subst.eval()?;

    //     println!("x={:16} y={:16} s={:16} pe={:16} yp={:16}", x, y, x.sin(), poly.eval(x), yp);
    // }

    Ok(expr.into())
}
