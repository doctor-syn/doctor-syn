use crate::bdmath::*;
use crate::error::{Error, Result};
use crate::polynomial::Polynomial;
use crate::{Expression, Name, Parity, VariableList};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::convert::TryInto;
use syn::{parse_quote, Expr};

fn mkexpr(x: &BigDecimal) -> Expr {
    Expression::from(x.clone()).into()
}

fn mul_add_polynomial(
    terms: &[BigDecimal],
    variable: Name,
    parity: Parity,
    span: Span,
) -> Result<Expr> {
    let k = terms.len();
    let highest_coeff = mkexpr(&terms[k - 1]);
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
                    let ti = mkexpr(&terms[i]);
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
                    let ti = mkexpr(&terms[i]);
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
                    let ti = mkexpr(&terms[i]);
                    quote!(mul_add(#x, #ti))
                })
                .collect();
            Ok(parse_quote!( (#highest_coeff) #( .#mul_adds )* ))
        }
    }
}

pub fn approx(
    expr: &Expression,
    num_terms: usize,
    xmin: f64,
    xmax: f64,
    variable: Name,
    parity: Parity,
    num_digits: i64,
) -> Result<Expression> {
    let xmin = bigdf(xmin);
    let xmax = bigdf(xmax);

    // let err_fn = || Error::CouldNotEvaulate(expr.span());
    let a = (&xmax + &xmin) * half();
    let b = pi(num_digits) / BigDecimal::from_usize(num_terms - 1).unwrap();
    let c = (&xmax - &xmin) * half();
    let mut xvalues = Vec::new();
    let mut yvalues = Vec::new();
    for i in 0..num_terms {
        // *almost* Chebyshev nodes.
        let x = &a - &c * cos(BigDecimal::from_usize(i).unwrap() * &b, num_digits);
        let mut vars = VariableList::new();
        vars.add_var(variable.clone(), mkexpr(&x).into());
        let subst = expr.subst(vars)?;
        let y: BigDecimal = subst.eval(num_digits)?.try_into()?;
        println!("x={:16} y={:16} {}", x, y, subst);
        xvalues.push(x);
        yvalues.push(y);
    }

    let poly = Polynomial::from_points(xvalues.as_slice(), yvalues.as_slice(), num_digits);

    mul_add_polynomial(poly.terms(), variable, parity, expr.span()).map(|e| e.into())
}
