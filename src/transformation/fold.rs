//! Evaluate expressions as integer, rational or floating point.
//!
//! At present, everything is converted to f64.
//! But `PI.sin()` should be zero.
//! But `(PI/2).sin()` should be one.

use crate::visitor::Visitor;
use crate::error::Error;
use crate::util::{as_number, from_any};

use syn::{Expr, ExprBinary, Token, punctuated::Punctuated, ExprMethodCall, BinOp};

#[derive(Debug, Clone)]
pub struct Fold {
    pub (crate) max_digits: u32,
}

impl Visitor for Fold {
    fn visit_method_call(&self, expr: &ExprMethodCall) -> Result<Expr, Error> {
        let receiver = self.visit_expr(&expr.receiver)?;
        let args : Punctuated<Expr, Token![,]> = expr.args.iter().map(|a| self.visit_expr(a)).collect::<Result<Punctuated<Expr, Token![,]>, Error>>()?;

        match (expr.method.to_string().as_str(), as_number::<f64>(&receiver)) {
            ("sin", Ok(receiver)) => return from_any(receiver.sin()),
            ("cos", Ok(receiver)) => return from_any(receiver.cos()),
            ("exp", Ok(receiver)) => return from_any(receiver.exp()),
            ("sqrt", Ok(receiver)) => return from_any(receiver.sqrt()),
            ("ln", Ok(receiver)) => return from_any(receiver.ln()),
            ("powf", Ok(receiver)) => return from_any(receiver.powf(as_number(&args[0])?)),
            _ => ()
        };

        Ok(ExprMethodCall {
            attrs: expr.attrs.clone(),
            receiver: Box::new(receiver),
            dot_token: expr.dot_token.clone(),
            method: expr.method.clone(),
            turbofish: expr.turbofish.clone(),
            paren_token: expr.paren_token.clone(),
            args: args,
        }.into())
    }

    fn visit_binary(&self, exprbinary: &ExprBinary) -> Result<Expr, Error> {
        let left = self.visit_expr(&exprbinary.left)?;
        let right = self.visit_expr(&exprbinary.right)?;

        match (as_number::<f64>(&left), as_number::<f64>(&right)) {
            (Ok(left), Ok(right)) => {
                match exprbinary.op {
                    BinOp::Add(_) => return from_any(left + right),
                    BinOp::Sub(_) => return from_any(left - right),
                    BinOp::Mul(_) => return from_any(left * right),
                    BinOp::Div(_) => return from_any(left / right),
                    _ => ()
                }
            }
            _ => ()
        }

        Ok(ExprBinary {
            attrs: exprbinary.attrs.clone(),
            left: Box::new(left),
            op: exprbinary.op.clone(),
            right: Box::new(right),
        }.into())
    }
}
