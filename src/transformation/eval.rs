//! Evaluate expressions as integer, rational or floating point.
//!
//! At present, everything is converted to f64.
//! But `PI.sin()` should be zero.
//! But `(PI/2).sin()` should be one.

use crate::error::{Error, Result};
use crate::visitor::Visitor;
use crate::Expression;

use crate::Evaluateable;
use std::convert::{TryFrom, TryInto};
use syn::spanned::Spanned;
use syn::{BinOp, Expr, ExprBinary, ExprMethodCall, ExprUnary, UnOp};

#[derive(Debug, Clone)]
pub struct Eval<T: TryFrom<Expression> + TryInto<Expression>> {
    pub(crate) datatype: std::marker::PhantomData<T>,
}

impl<T: Evaluateable> Visitor for Eval<T> {
    fn visit_method_call(&self, expr: &ExprMethodCall) -> Result<Expr> {
        // println!("visit_method_call {:?}", expr);
        let receiver: Expression = self.visit_expr(&expr.receiver)?.into();
        let args: Vec<Expression> = expr
            .args
            .iter()
            .map(|a| -> Result<Expression> { Ok(self.visit_expr(a)?.into()) })
            .collect::<Result<Vec<_>>>()?;

        match (
            expr.method.to_string().as_str(),
            T::try_from(receiver),
            args.len(),
        ) {
            // nan() -> Self;
            // infinity() -> Self;
            // neg_infinity() -> Self;
            // neg_zero() -> Self;
            // min_value() -> Self;
            // min_positive_value() -> Self;
            // epsilon() -> Self {
            // max_value() -> Self;
            // ("is_nan", Ok(receiver), 0) => Ok(receiver.is_nan().try_into()?.into()),
            // ("is_infinite", Ok(receiver), 0) => Ok(receiver.is_infinite().try_into()?.into()),
            // ("is_finite", Ok(receiver), 0) => Ok(receiver.is_finite().try_into()?.into()),
            // ("is_normal", Ok(receiver), 0) => Ok(receiver.is_normal().try_into()?.into()),
            // ("classify", Ok(receiver), 0) => Ok(receiver.classify().try_into()?.into()),
            ("floor", Ok(receiver), 0) => Ok(receiver.floor().try_into()?.into()),
            ("ceil", Ok(receiver), 0) => Ok(receiver.ceil().try_into()?.into()),
            ("round", Ok(receiver), 0) => Ok(receiver.round().try_into()?.into()),
            ("trunc", Ok(receiver), 0) => Ok(receiver.trunc().try_into()?.into()),
            ("fract", Ok(receiver), 0) => Ok(receiver.fract().try_into()?.into()),
            ("abs", Ok(receiver), 0) => Ok(receiver.abs().try_into()?.into()),
            ("signum", Ok(receiver), 0) => Ok(receiver.signum().try_into()?.into()),
            // ("is_sign_positive", Ok(receiver), 0) => Ok(receiver.is_sign_positive().try_into()?.into()),
            // ("is_sign_negative", Ok(receiver), 0) => Ok(receiver.is_sign_negative().try_into()?.into()),
            ("mul_add", Ok(receiver), 2) => Ok(receiver
                .mul_add(args[0].clone().try_into()?, args[1].clone().try_into()?)
                .try_into()?
                .into()),
            ("recip", Ok(receiver), 0) => Ok(receiver.recip().try_into()?.into()),
            ("powi", Ok(receiver), 1) => Ok(receiver
                .powf(args[0].clone().try_into()?)
                .try_into()?
                .into()),
            ("powf", Ok(receiver), 1) => Ok(receiver
                .powf(args[0].clone().try_into()?)
                .try_into()?
                .into()),
            ("sqrt", Ok(receiver), 0) => Ok(receiver.sqrt().try_into()?.into()),
            ("exp", Ok(receiver), 0) => Ok(receiver.exp().try_into()?.into()),
            ("exp2", Ok(receiver), 0) => Ok(receiver.exp2().try_into()?.into()),
            ("ln", Ok(receiver), 0) => Ok(receiver.ln().try_into()?.into()),
            ("log", Ok(receiver), 1) => {
                Ok(receiver.log(args[0].clone().try_into()?).try_into()?.into())
            }
            ("log2", Ok(receiver), 0) => Ok(receiver.log2().try_into()?.into()),
            ("log10", Ok(receiver), 0) => Ok(receiver.log10().try_into()?.into()),
            ("to_degrees", Ok(receiver), 0) => Ok(receiver.to_degrees().try_into()?.into()),
            ("to_radians", Ok(receiver), 0) => Ok(receiver.to_radians().try_into()?.into()),
            ("max", Ok(receiver), 1) => {
                Ok(receiver.max(args[0].clone().try_into()?).try_into()?.into())
            }
            ("min", Ok(receiver), 1) => {
                Ok(receiver.min(args[0].clone().try_into()?).try_into()?.into())
            }
            ("abs_sub", Ok(receiver), 1) => Ok(receiver
                .abs_sub(args[0].clone().try_into()?)
                .try_into()?
                .into()),
            ("cbrt", Ok(receiver), 0) => Ok(receiver.cbrt().try_into()?.into()),
            ("hypot", Ok(receiver), 1) => Ok(receiver
                .hypot(args[0].clone().try_into()?)
                .try_into()?
                .into()),
            ("sin", Ok(receiver), 0) => Ok(receiver.sin().try_into()?.into()),
            ("cos", Ok(receiver), 0) => Ok(receiver.cos().try_into()?.into()),
            ("tan", Ok(receiver), 0) => Ok(receiver.tan().try_into()?.into()),
            ("asin", Ok(receiver), 0) => Ok(receiver.asin().try_into()?.into()),
            ("acos", Ok(receiver), 0) => Ok(receiver.acos().try_into()?.into()),
            ("atan", Ok(receiver), 0) => Ok(receiver.atan().try_into()?.into()),
            ("atan2", Ok(receiver), 1) => Ok(receiver
                .atan2(args[0].clone().try_into()?)
                .try_into()?
                .into()),
            //("sin_cos", Ok(receiver), 0) => Ok(receiver.sin_cos().try_into()?.into()),
            ("exp_m1", Ok(receiver), 0) => Ok(receiver.exp_m1().try_into()?.into()),
            ("ln_1p", Ok(receiver), 0) => Ok(receiver.ln_1p().try_into()?.into()),
            ("sinh", Ok(receiver), 0) => Ok(receiver.sinh().try_into()?.into()),
            ("cosh", Ok(receiver), 0) => Ok(receiver.cosh().try_into()?.into()),
            ("tanh", Ok(receiver), 0) => Ok(receiver.tanh().try_into()?.into()),
            ("asinh", Ok(receiver), 0) => Ok(receiver.asinh().try_into()?.into()),
            ("acosh", Ok(receiver), 0) => Ok(receiver.acosh().try_into()?.into()),
            ("atanh", Ok(receiver), 0) => Ok(receiver.atanh().try_into()?.into()),
            // ("integer_decode", Ok(receiver), 0) => Ok(receiver.integer_decode().try_into()?.into()),
            _ => Err(Error::CouldNotEvaulate(expr.span())),
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
            _ => Err(Error::CouldNotEvaulate(exprbinary.span())),
        }
    }

    fn visit_unary(&self, exprunary: &ExprUnary) -> Result<Expr> {
        let expr = T::try_from(self.visit_expr(&exprunary.expr)?.into())?;
        match exprunary.op {
            // UnOp::Deref(_) => (),
            // UnOp::Not(_) => (),
            UnOp::Neg(_) => Ok((-expr).try_into()?.into()),
            _ => Err(Error::CouldNotEvaulate(exprunary.span())),
        }
    }
}
