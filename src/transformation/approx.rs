use crate::error::Result;
use crate::polynomial::Polynomial;
use crate::{Expression, Name, VariableList};
pub use std::convert::{TryFrom, TryInto};
use syn::{parse_quote, Expr, Stmt};

pub fn approx(
    expr: &Expression,
    num_terms: usize,
    xmin: f64,
    xmax: f64,
    variable: Name,
) -> Result<Expression> {
    use std::f64::consts::PI;
    let a = (xmax + xmin) * 0.5;
    let b = PI / (num_terms - 1) as f64;
    let c = (xmax - xmin) * 0.5;
    let mut xvalues = Vec::new();
    let mut yvalues = Vec::new();
    for i in 0..num_terms {
        // *almost* Chebyshev nodes.
        let x = a - c * (i as f64 * b).cos();
        let mut vars = VariableList::new();
        vars.add_var(variable.clone(), x.try_into()?);
        let subst = expr.subst(vars)?;
        println!("subst={}", subst);
        let y: f64 = subst.eval_float()?;
        xvalues.push(x);
        yvalues.push(y);
    }
    println!("{:?}", xvalues);
    println!("{:?}", yvalues);

    let poly = Polynomial::from_points(xvalues.as_slice(), yvalues.as_slice());
    let k = num_terms;
    let terms = poly.terms();
    let mut stmts: Vec<Stmt> = Vec::new();
    let tk = terms[k - 1];
    stmts.push(parse_quote!(let y = #tk;));
    for i in (0..k - 1).rev() {
        let ti = terms[i];
        stmts.push(parse_quote!(let y = y.mul_add(x, #ti);));
    }

    let res: Expr = parse_quote!(
        {
            #( #stmts )*
            y
        }
    );
    println!("{:?}", res);

    Ok(res.into())
}
