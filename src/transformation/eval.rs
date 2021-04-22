//! Evaluate expressions as integer, rational or floating point.
//!
//! At present, everything is converted to f64.
//! But `PI.sin()` should be zero.
//! But `(PI/2).sin()` should be one.

use crate::visitor::Visitor;
use crate::error::{Error, Result};
use crate::Expression;
// use crate::util::{as_number, from_any};

use syn::{Expr, ExprBinary, ExprMethodCall, BinOp};
use syn::spanned::Spanned;
use std::convert::{TryFrom, TryInto};
use num_traits::Float;

#[derive(Debug, Clone)]
pub struct Eval<T: TryFrom<Expression> + TryInto<Expression>> {
    pub (crate) datatype: std::marker::PhantomData<T>,
}

impl<T: TryFrom<Expression, Error=Error> + TryInto<Expression, Error=Error> + Float> Visitor for Eval<T> {
    fn visit_method_call(&self, expr: &ExprMethodCall) -> Result<Expr> {
        println!("visit_method_call {:?}", expr);
        let receiver : Expression = self.visit_expr(&expr.receiver)?.into();
        let args : Vec<Expression> = expr.args.iter().map(|a| -> Result<Expression> {Ok(self.visit_expr(a)?.into())}).collect::<Result<Vec<_>>>()?;

        match (expr.method.to_string().as_str(), T::try_from(receiver), args.len()) {
            ("sin", Ok(receiver), 0) => Ok(receiver.sin().try_into()?.into()),
            ("cos", Ok(receiver), 0) => Ok(receiver.cos().try_into()?.into()),
            ("exp", Ok(receiver), 0) => Ok(receiver.exp().try_into()?.into()),
            ("sqrt", Ok(receiver), 0) => Ok(receiver.sqrt().try_into()?.into()),
            ("ln", Ok(receiver), 0) => Ok(receiver.ln().try_into()?.into()),
            ("powf", Ok(receiver), 1) => Ok(receiver.powf(args[0].clone().try_into()?).try_into()?.into()),
            _ => Err(Error::CouldNotEvaulate(expr.span()))
        }
    }

    fn visit_binary(&self, exprbinary: &ExprBinary) -> Result<Expr> {
        let left = T::try_from(self.visit_expr(&exprbinary.left)?.into())?;
        let right = T::try_from(self.visit_expr(&exprbinary.right)?.into())?;

        match exprbinary.op {
            BinOp::Add(_) => Ok((left + right).try_into()?.into()),
            BinOp::Sub(_) => Ok((left - right).try_into()?.into()),
            BinOp::Mul(_) => Ok((left * right).try_into()?.into()),
            BinOp::Div(_) => Ok((left / right).try_into()?.into()),
            _ => Err(Error::CouldNotEvaulate(exprbinary.span()))
        }
    }
}
