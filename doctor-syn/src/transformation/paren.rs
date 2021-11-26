use super::tools::*;
use crate::error::Result;
use crate::visitor::Visitor;
use syn::{Expr, ExprBinary, ExprUnary};

#[derive(Debug)]
pub struct Paren {}

pub fn parenthesise_op(expr: Expr) -> Expr {
    match expr {
        Expr::Binary(_) => make_paren(expr),
        Expr::Unary(_) => make_paren(expr),
        _ => expr,
    }
}

impl Visitor for Paren {
    fn visit_expr(&self, expr: &Expr) -> Result<Expr> {
        let expr = self.visit_expr_core(expr)?;
        Ok(match expr {
            Expr::Binary(ExprBinary {
                left, right, op, ..
            }) => {
                let left = parenthesise_op(*left.clone());
                let right = parenthesise_op(*right.clone());
                make_binary(left, op, right)
            }
            Expr::Unary(ExprUnary { op, expr, .. }) => {
                println!("HERE");
                let expr = parenthesise_op(*expr.clone());
                make_unary(op, expr)
            }
            _ => expr,
        })
    }
}

#[test]
fn unary() -> Result<()> {
    use crate::expr;
    assert_eq!(expr!(-x).paren()?, expr!(-x));
    assert_eq!(expr!(x + 1).paren()?, expr!(x + 1));
    assert_eq!(expr!(x + 1 + 1).paren()?, expr!((x + 1) + 1));
    assert_eq!(expr!(x + 1 + 1 + 1).paren()?, expr!(((x + 1) + 1) + 1));
    assert_eq!(expr!(2 * x + y).paren()?, expr!((2 * x) + y));
    Ok(())
}
