//! Evaluate expressions as integer, rational or floating point.
//!
//! At present, everything is converted to f64.
//! But `PI.sin()` should be zero.
//! But `(PI/2).sin()` should be one.

use crate::error::{Error, Result};
use crate::visitor::Visitor;
use crate::Expression;

use std::convert::TryInto;
use syn::{BinOp, Expr, ExprBinary, ExprMethodCall, ExprParen, ExprPath, ExprUnary, UnOp};

use crate::bdmath::*;

#[derive(Debug, Clone)]
pub struct Eval {
    pub(crate) num_digits: i32,
}

fn eval_err(e: Expr) -> Error {
    Error::CouldNotEvaulate(Expression::from(e))
}

impl Visitor for Eval {
    // eg. "(x)"
    fn visit_paren(&self, exprparen: &ExprParen) -> Result<Expr> {
        self.visit_expr(&exprparen.expr)
    }

    fn visit_method_call(&self, expr: &ExprMethodCall) -> Result<Expr> {
        // println!("visit_method_call {:?}", expr);
        let receiver: Expression = self.visit_expr(&expr.receiver)?.into();
        let args: Vec<Expression> = expr
            .args
            .iter()
            .map(|a| -> Result<Expression> { Ok(self.visit_expr(a)?.into()) })
            .collect::<Result<Vec<_>>>()?;

        let receiver: BigDecimal = receiver.try_into()?;
        let args: Vec<BigDecimal> = args
            .iter()
            .map(|a| a.try_into())
            .collect::<Result<Vec<_>>>()?;

        let errfn = || Error::CouldNotEvaulate(Expression::from(Expr::from(expr.clone())));
        let arg0 = || args[0].clone();
        // let mkexpr = |e : BigDecimal| Result::Ok(Expr::from(Expression::from(e)));

        match (expr.method.to_string().as_str(), receiver, args.len()) {
            // ("is_nan", receiver, 0) => Ok(Expression::from(is_nan(x, self.num_digits)).into()),
            // ("is_infinite", x, 0) => Ok(Expression::from(is_infinite(x, self.num_digits)).into()),
            // ("is_finite", x, 0) => Ok(Expression::from(is_finite(x, self.num_digits)).into()),
            // ("is_normal", x, 0) => Ok(Expression::from(is_normal(x, self.num_digits)).into()),
            // ("classify", x, 0) => Ok(Expression::from(classify(x, self.num_digits)).into()),
            // ("floor", x, 0) => Ok(Expression::from(floor(x, self.num_digits)).into()),
            // ("ceil", x, 0) => Ok(Expression::from(ceil(x, self.num_digits)).into()),
            ("round", x, 0) => Ok(Expression::from(round(x, 0)).into()),

            // ("trunc", x, 0) => Ok(Expression::from(trunc(x, self.num_digits)).into()),

            // ("fract", x, 0) => Ok(Expression::from(fract(x, self.num_digits)).into()),
            ("abs", x, 0) => Ok(Expression::from(x.abs()).into()),

            ("signum", x, 0) => Ok(Expression::from(x.signum()).into()),

            // ("mul_add", x, 2) => Ok(x
            //     .mul_add(arg0().try_into()?, args[1].clone().try_into()?)
            //     .try_into()?

            //     .into()),
            ("recip", x, 0) => Ok(Expression::from(x.inverse()).into()),

            ("powi", x, 1) => {
                Ok(Expression::from(pow(x, arg0(), self.num_digits).ok_or_else(errfn)?).into())
            }

            ("powf", x, 1) => {
                Ok(Expression::from(pow(x, arg0(), self.num_digits).ok_or_else(errfn)?).into())
            }

            ("sqrt", x, 0) => Ok(Expression::from(x.sqrt().ok_or_else(errfn)?).into()),

            ("exp", x, 0) => Ok(Expression::from(exp(x, self.num_digits)).into()),

            ("exp2", x, 0) => Ok(Expression::from(pow(two(), x, self.num_digits).ok_or_else(errfn)?).into()),

            ("ln", x, 0) => Ok(Expression::from(ln(x, self.num_digits).ok_or_else(errfn)?).into()),

            ("log", x, 1) => {
                Ok(Expression::from(log(x, arg0(), self.num_digits).ok_or_else(errfn)?).into())
            }

            ("log2", x, 0) => {
                Ok(Expression::from(log(x, two(), self.num_digits).ok_or_else(errfn)?).into())
            }

            ("log10", x, 0) => {
                Ok(Expression::from(log(x, bigd(10), self.num_digits).ok_or_else(errfn)?).into())
            }

            // ("to_degrees", x, 0) => Ok(Expression::from(to_degrees(x, self.num_digits)).into()),
            // ("to_radians", x, 0) => Ok(Expression::from(to_radians(x, self.num_digits)).into()),
            // ("max", x, 1) => {
            //     Ok(x.max(arg0().Expression::from(()?x, self.num_digits)).into())
            // }
            // ("min", x, 1) => {
            //     Ok(x.min(arg0().Expression::from(()?x, self.num_digits)).into())
            // }
            // ("abs_sub", x, 1) => Ok(x
            //     .abs_sub(arg0().try_into()?)
            //     .try_into()?
            //     .into()),
            // ("cbrt", x, 0) => Ok(Expression::from(cbrt(x, self.num_digits)).into()),
            // ("hypot", x, 1) => Ok(x
            //     .hypot(arg0().try_into()?)
            //     .try_into()?
            //     .into()),
            ("sin", x, 0) => Ok(Expression::from(sin(x, self.num_digits)).into()),

            ("cos", x, 0) => Ok(Expression::from(cos(x, self.num_digits)).into()),

            ("tan", x, 0) => Ok(Expression::from(tan(x, self.num_digits)).into()),

            ("asin", x, 0) => Ok(Expression::from(asin(x, self.num_digits)).into()),

            ("acos", x, 0) => Ok(Expression::from(acos(x, self.num_digits)).into()),

            ("atan", x, 0) => Ok(Expression::from(atan(x, self.num_digits)).into()),

            // ("atan2", x, 1) => Ok(x
            //     .atan2(arg0().try_into()?)
            //     .try_into()?
            //     .into()),

            // //("sin_cos", x, 0) => Ok(Expression::from(sin_cos(x, self.num_digits)).into()),
            // ("exp_m1", x, 0) => Ok(Expression::from(exp_m1(x, self.num_digits)).into()),
            // ("ln_1p", x, 0) => Ok(Expression::from(ln_1p(x, self.num_digits)).into()),
            // ("sinh", x, 0) => Ok(Expression::from(sinh(x, self.num_digits)).into()),
            // ("cosh", x, 0) => Ok(Expression::from(cosh(x, self.num_digits)).into()),
            // ("tanh", x, 0) => Ok(Expression::from(tanh(x, self.num_digits)).into()),
            // ("asinh", x, 0) => Ok(Expression::from(asinh(x, self.num_digits)).into()),
            // ("acosh", x, 0) => Ok(Expression::from(acosh(x, self.num_digits)).into()),
            // ("atanh", x, 0) => Ok(Expression::from(atanh(x, self.num_digits)).into()),
            // ("integer_decode", x, 0) => Ok(Expression::from(integer_decode(x, self.num_digits)).into()),
            _ => Err(eval_err(expr.clone().into())),
        }
    }

    fn visit_binary(&self, exprbinary: &ExprBinary) -> Result<Expr> {
        let left: Expression = self.visit_expr(&exprbinary.left)?.into();
        let right: Expression = self.visit_expr(&exprbinary.right)?.into();

        if left.is_numeric() && right.is_numeric() {
            let left: BigDecimal = left.try_into().unwrap();
            let right: BigDecimal = right.try_into().unwrap();
            match exprbinary.op {
                BinOp::Add(_) => Ok(Expression::from(left + right).into()),
                BinOp::Sub(_) => Ok(Expression::from(left - right).into()),
                BinOp::Mul(_) => {
                    Ok(Expression::from(round(left * right, self.num_digits as i64)).into())
                }
                BinOp::Div(_) => {
                    Ok(Expression::from(round(left / right, self.num_digits as i64)).into())
                }
                BinOp::Lt(_) => Ok(Expression::from(left < right).into()),
                BinOp::Gt(_) => Ok(Expression::from(left > right).into()),
                BinOp::Le(_) => Ok(Expression::from(left <= right).into()),
                BinOp::Ge(_) => Ok(Expression::from(left >= right).into()),
                BinOp::Eq(_) => Ok(Expression::from(left == right).into()),
                BinOp::Ne(_) => Ok(Expression::from(left != right).into()),
                _ => Err(eval_err(exprbinary.clone().into())),
            }
        } else {
            Err(eval_err(exprbinary.clone().into()))
        }
    }

    fn visit_unary(&self, exprunary: &ExprUnary) -> Result<Expr> {
        let expr: Expression = self.visit_expr(&exprunary.expr)?.into();
        if expr.is_numeric() {
            let expr: BigDecimal = expr.try_into().unwrap();
            match exprunary.op {
                // UnOp::Deref(_) => (),
                // UnOp::Not(_) => (),
                UnOp::Neg(_) => Ok(Expression::from(-expr).into()),
                _ => Err(eval_err(exprunary.clone().into())),
            }
        } else {
            Err(eval_err(exprunary.clone().into()))
        }
    }

    // eg. "x" or "f64::const::PI"
    fn visit_path(&self, exprpath: &ExprPath) -> Result<Expr> {
        if let Some(name) = exprpath.path.get_ident() {
            if name == "PI" {
                Ok(Expression::from(pi(self.num_digits)).into())
            } else {
                Err(eval_err(exprpath.clone().into()))
            }
        } else {
            Err(eval_err(exprpath.clone().into()))
        }
    }
}
