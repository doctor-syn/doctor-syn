use super::tools::*;
use crate::error::{Error, Result};
use crate::visitor::Visitor;
use syn::spanned::Spanned;
use syn::{BinOp, Expr, ExprBinary, ExprUnary, UnOp};

#[derive(Debug)]
pub struct Expand {}

// Get a series of additions/subtractions.
fn match_sum(expr: &Expr, sum: &mut Vec<Expr>, is_negated: bool) -> Result<()> {
    let expr = deparen(expr);
    match expr {
        Expr::Binary(ExprBinary {
            left, op, right, ..
        }) => match op {
            BinOp::Add(_) => {
                match_sum(left, sum, is_negated)?;
                match_sum(right, sum, is_negated)?;
                return Ok(());
            }
            BinOp::Sub(_) => {
                match_sum(left, sum, is_negated)?;
                match_sum(right, sum, !is_negated)?;
                return Ok(());
            }
            BinOp::Mul(_) => {
                let mut left_sum = Vec::new();
                match_sum(left, &mut left_sum, is_negated)?;
                let mut right_sum = Vec::new();
                match_sum(right, &mut right_sum, false)?;
                for lhs in left_sum.iter() {
                    for rhs in right_sum.iter() {
                        sum.push(make_binary(lhs.clone(), op.clone(), rhs.clone()));
                    }
                }
                return Ok(());
            }
            _ => {}
        },
        _ => {}
    }
    if is_negated {
        sum.push(negate(&expr));
    } else {
        sum.push(expr.clone());
    }
    Ok(())
}

impl Visitor for Expand {
    fn visit_unary(&self, exprunary: &ExprUnary) -> Result<Expr> {
        match exprunary.op {
            UnOp::Deref(_) => Err(Error::UnsupportedExpr(exprunary.span())),
            UnOp::Not(_) => Err(Error::UnsupportedExpr(exprunary.span())),
            UnOp::Neg(_) => {
                let mut sum = Vec::new();
                match_sum(deparen(&exprunary.expr), &mut sum, true)?;
                Ok(make_sum(&*sum))
            }
        }
    }

    fn visit_binary(&self, exprbinary: &ExprBinary) -> Result<Expr> {
        let mut sum = Vec::new();
        let expr: Expr = exprbinary.clone().into();
        match_sum(&expr, &mut sum, false)?;
        Ok(make_sum(&*sum))
    }
}

#[test]
fn expand() -> Result<()> {
    use crate::expr;
    // Unary
    assert_eq!(expr!(--x).expand()?, expr!(x));
    assert_eq!(expr!(-(x + 1)).expand()?, expr!(-x - 1));

    // Binary add/sub
    assert_eq!(expr!((x + 1) + (x + 1)).expand()?, expr!(x + 1 + x + 1));
    assert_eq!(
        expr!((x + 1) + ((x + 1) + (x + 1))).expand()?,
        expr!(x + 1 + x + 1 + x + 1)
    );
    assert_eq!(expr!((x + 1) - (x + 1)).expand()?, expr!(x + 1 - x - 1));
    assert_eq!(
        expr!((x + 1) - ((x + 1) - (x + 1))).expand()?,
        expr!(x + 1 - x - 1 + x + 1)
    );
    assert_eq!(
        expr!((x + 1) - ((x + 1) - (-x + 1))).expand()?,
        expr!(x + 1 - x - 1 - x + 1)
    );

    // Binary mul
    assert_eq!(expr!(x * x).expand()?, expr!(x * x));
    assert_eq!(expr!(x * (x + 1)).expand()?, expr!(x * x + x * 1));
    assert_eq!(expr!((x + 1) * x).expand()?, expr!(x * x + 1 * x));
    assert_eq!(
        expr!((x + 1) * (x + 1)).expand()?,
        expr!(x * x + x * 1 + 1 * x + 1 * 1)
    );
    assert_eq!(
        expr!((x + 1) * (x + 1) * (x + 1)).expand()?,
        expr!(
            x * x * x
                + x * x * 1
                + x * 1 * x
                + x * 1 * 1
                + 1 * x * x
                + 1 * x * 1
                + 1 * 1 * x
                + 1 * 1 * 1
        )
    );
    Ok(())
}
