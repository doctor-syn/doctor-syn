//! Mathematical function generation.
#![allow(dead_code)]

use super::context::{from_any, as_number, parse_str, Context, Variable};
use super::polynomial::Polynomial;

use syn::{parse_quote, Lit, Meta, MetaNameValue, Stmt, ExprClosure};

use crate::error::Error;
use proc_macro2::Span;
use syn::spanned::Spanned;

// Only accept f64.
fn type_is_ok(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Path(typepath) => {
            if typepath.path.is_ident("f64") {
                return true;
            }
        }
        _ => (),
    }
    false
}

fn generate_function(
    args: Vec<Variable>,
    body: &syn::Expr,
) -> Result<ExprClosure, Error> {
    let domain = &args[0].domain;
    let xmin = domain
        .min
        .as_ref()
        .map(|e| as_number::<f64>(e))
        .unwrap_or(Ok(0.))?;
    let xmax = domain
        .max
        .as_ref()
        .map(|e| as_number::<f64>(e))
        .unwrap_or(Ok(1.))?;
    let terms = domain
        .terms
        .as_ref()
        .map(|e| as_number::<usize>(e))
        .unwrap_or(Ok(7))?;
    use std::f64::consts::PI;
    let a = (xmax + xmin) * 0.5;
    let b = PI / (terms - 1) as f64;
    let c = (xmax - xmin) * 0.5;
    let mut xvalues = Vec::new();
    let mut yvalues = Vec::new();
    let mut interp = Interpreter::from_variables(args);
    let xpath = parse_str("x")?;
    for i in 0..terms {
        // *almost* Chebyshev nodes.
        let x = a - c * (i as f64 * b).cos();
        interp.set_var(&xpath, from_any::<f64>(x)?)?;
        let y = as_number(&interp.expr(&body).unwrap())?;
        println!("{:?} {:?}", x, y);
        xvalues.push(x);
        yvalues.push(y);
    }

    let poly = Polynomial::from_points(xvalues.as_slice(), yvalues.as_slice());
    let k = terms;
    let terms = poly.terms();
    let mut stmts: Vec<Stmt> = Vec::new();
    let tk = terms[k - 1];
    stmts.push(parse_quote!(let y = #tk;));
    for i in (0..k - 1).rev() {
        let ti = terms[i];
        stmts.push(parse_quote!(let y = y.mul_add(x, #ti);));
    }
    let res: ExprClosure = parse_quote!(
        |x| {
            #( #stmts )*
            y
        }
    );
    println!("{:?}", res);
    Ok(res)
}

// Parse an attribute of the form #[name="expr"]
fn parse_expr_attr(attr: &syn::Attribute) -> Result<syn::Expr, Error> {
    let meta = attr
        .parse_meta()
        .map_err(|_| Error::BadAttribute(attr.span()))?;

    Ok(match meta {
        Meta::NameValue(MetaNameValue {
            lit: Lit::Str(lit_str),
            ..
        }) => lit_str.parse().map_err(|_| Error::BadAttribute(attr.pound_token.span))?,
        _ => return Err(Error::BadAttribute(attr.pound_token.span)),
    })
}

pub fn do_approx(clos: ExprClosure) -> Result<ExprClosure, Error> {
    println!("{:#?}", clos);

    // Collect the arguments of the closure.
    let mut variables: Vec<Variable> = Vec::new();
    for arg in &clos.inputs {
        match arg {
            syn::Pat::Ident(ref id) => {
                let mut var = Variable::new();
                var.path = syn::Path::from(id.ident.clone());
                for attr in &id.attrs {
                    if attr.path.is_ident("min") {
                        var.domain.min = Some(parse_expr_attr(attr)?);
                    } else if attr.path.is_ident("max") {
                        var.domain.max = Some(parse_expr_attr(attr)?);
                    } else if attr.path.is_ident("terms") {
                        var.domain.terms = Some(parse_expr_attr(attr)?);
                    } else {
                        return Err(Error::BadAttribute(id.ident.span()))
                    }
                }
                variables.push(var);
            }
            _ => {
                return Err(Error::UnsuportedClosureArgument(Span::call_site()))
            }
        }
    }

    generate_function(variables, &clos.body)
}
