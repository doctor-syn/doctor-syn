use crate::error::{Error, Result};
use crate::polynomial::Polynomial;
use crate::{Evaluateable, Expression, Name, Parity, VariableList};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::convert::TryInto;
use syn::{parse_quote, Expr};

fn mul_add_polynomial(terms: &[f64], variable: Name, parity: Parity, span: Span) -> Result<Expr> {
    let k = terms.len();
    let highest_coeff = terms[k - 1];
    let x = variable.as_ref();
    match parity {
        Parity::Odd => {
            if k % 2 != 0 {
                return Err(Error::WrongNumberOfTerms(span));
            }
            let mul_adds: Vec<TokenStream> = (1..k - 1)
                .step_by(2)
                .rev()
                .map(|i| {
                    let ti = terms[i];
                    quote!(mul_add(#x*#x, #ti))
                })
                .collect();
            Ok(parse_quote!( (#highest_coeff) #( .#mul_adds )* * #x ))
        }
        Parity::Even => {
            if k % 2 == 0 {
                return Err(Error::WrongNumberOfTerms(span));
            }
            let mul_adds: Vec<TokenStream> = (0..k - 1)
                .step_by(2)
                .rev()
                .map(|i| {
                    let ti = terms[i];
                    quote!(mul_add(#x*#x, #ti))
                })
                .collect();
            // let ts = quote!( (#highest_coeff) #( .#mul_adds )* ).to_string();
            // println!("ts={}", ts);
            // let e : Expr = syn::parse_str(ts.as_str()).unwrap();
            // println!("e={}", e.to_token_stream());

            // Ok(syn::parse_str(ts.as_str()).unwrap())
            Ok(parse_quote!( (#highest_coeff) #( .#mul_adds )* ))
        }
        Parity::Neither => {
            let mul_adds: Vec<TokenStream> = (0..k - 1)
                .rev()
                .map(|i| {
                    let ti = terms[i];
                    quote!(mul_add(#x, #ti))
                })
                .collect();
            Ok(parse_quote!( (#highest_coeff) #( .#mul_adds )* ))
        }
    }
}

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
        // println!("{} {}", y, subst);
        // println!("x={:16} y={:16} s={:16}", x, y, x.sin());
        xvalues.push(x);
        yvalues.push(y);
    }

    let poly = Polynomial::from_points(xvalues.as_slice(), yvalues.as_slice());

    mul_add_polynomial(poly.terms(), variable, parity, expr.span()).map(|e| e.into())
}
