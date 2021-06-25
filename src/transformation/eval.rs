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

use bigdecimal::{BigDecimal, FromPrimitive, One, Signed, Zero};
use num_bigint::BigInt;

#[derive(Debug, Clone)]
pub struct Eval {
    pub(crate) num_digits: i32,
}

// BigDecimal's round panics on large numbers.
// example:
//   round to 5 digits:
//
//   1.23456789 (1.23456789, 8)
//     -----
//  +0.000005   (5, 6)
pub fn round(x: BigDecimal, round_digits: i64) -> BigDecimal {
    let (bigint, decimal_part_digits) = x.into_bigint_and_exponent();

    //let need_to_round_digits = decimal_part_digits - round_digits;

    let five = if bigint.is_positive() { 5 } else { -5 };
    let x = BigDecimal::new(bigint, decimal_part_digits);

    // Already rounded or negative digits.
    if round_digits < 0 || decimal_part_digits <= round_digits {
        return x;
    }

    (x + BigDecimal::new(BigInt::from_i32(five).unwrap(), round_digits + 1))
        .with_scale(round_digits)
}

#[test]
fn test_round() {
    use std::str::FromStr;
    assert_eq!(
        round(BigDecimal::from_str("1.23456789").unwrap(), 5),
        BigDecimal::from_str("1.23457").unwrap()
    );
    assert_eq!(
        round(BigDecimal::from_str("1.234").unwrap(), 5),
        BigDecimal::from_str("1.234").unwrap()
    );
    assert_eq!(
        round(BigDecimal::from_str("1.234").unwrap(), 0),
        BigDecimal::from_str("1").unwrap()
    );
    assert_eq!(
        round(BigDecimal::from_str("1.23456000").unwrap(), 5),
        BigDecimal::from_str("1.23456").unwrap()
    );
}

fn pi(num_digits: i32) -> BigDecimal {
    // let x : BigDecimal = "3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679".parse().unwrap();
    //round(x, num_digits as i64)

    // Machin series.
    (atan(one() / bigd(5), num_digits) * bigd(4) - atan(one() / bigd(239), num_digits)) * bigd(4)
}

fn bigd(i: i32) -> BigDecimal {
    BigDecimal::from_i32(i).unwrap()
}

fn one() -> BigDecimal {
    bigd(1)
}

fn half() -> BigDecimal {
    BigDecimal::from_f32(0.5).unwrap()
}

fn two() -> BigDecimal {
    bigd(2)
}

// Evaluate McLaurin series of exp-like functions (sin, cos, exp)
// Note that we would prefer to use round(), but there are bugs in BigDecimal.
fn mclaurin<F: FnMut(i32, &BigDecimal, &BigDecimal, &BigDecimal) -> Option<BigDecimal>>(
    x: BigDecimal,
    num_digits: i32,
    mut f: F,
) -> BigDecimal {
    let mut power = BigDecimal::one();
    let mut factorial = BigDecimal::one();
    let mut tot = BigDecimal::zero();
    let mut i = 0;
    loop {
        let new_tot = f(i, &tot, &power, &factorial);

        if let Some(new_tot) = new_tot {
            let new_tot = round(new_tot, num_digits as i64 * 2);
            // println!(
            //     "new_tot={} power={} factorial={}",
            //     new_tot, power, factorial
            // );
            if new_tot == tot {
                break;
            }
            if i > 3000 {
                panic!("Sequence did not converge.");
            }
            tot = new_tot;
        }

        power *= &x;
        i += 1;
        factorial *= bigd(i);
        power = round(power, num_digits as i64 * 2);
    }

    round(tot, num_digits as i64 + 1).normalized()
}

fn sin(x: BigDecimal, num_digits: i32) -> BigDecimal {
    mclaurin(x, num_digits, |i, tot, power, factorial| match i & 3 {
        1 => Some(tot + power / factorial),
        3 => Some(tot - power / factorial),
        _ => None,
    })
}

fn cos(x: BigDecimal, num_digits: i32) -> BigDecimal {
    mclaurin(x, num_digits, |i, tot, power, factorial| match i & 3 {
        0 => Some(tot + power / factorial),
        2 => Some(tot - power / factorial),
        _ => None,
    })
}

fn exp(x: BigDecimal, num_digits: i32) -> BigDecimal {
    mclaurin(x, num_digits, |_i, tot, power, factorial| {
        Some(tot + power / factorial)
    })
}

fn tan(x: BigDecimal, num_digits: i32) -> BigDecimal {
    sin(x.clone(), num_digits) / cos(x, num_digits)
}

fn asin(x: BigDecimal, num_digits: i32) -> BigDecimal {
    let mut numer = BigDecimal::one();
    let mut denom = BigDecimal::one();
    mclaurin(x, num_digits, |i, tot, power, _factorial| {
        if (i & 1) != 0 {
            let z = bigd(i);
            //println!("numer={} denom={}", numer, denom);
            let res = tot + power * &numer / &denom / &z;
            numer *= &z;
            denom *= bigd(i + 1);
            Some(res)
        } else {
            None
        }
    })
}

fn acos(x: BigDecimal, num_digits: i32) -> BigDecimal {
    pi(num_digits) / two() - asin(x, num_digits)
}

fn atan(x: BigDecimal, num_digits: i32) -> BigDecimal {
    if x.abs() > half() {
        // https://en.wikipedia.org/wiki/Inverse_trigonometric_functions
        atan(
            one() / (one() + (one() + &x * &x).sqrt().unwrap()),
            num_digits,
        ) * two()
            * x.signum()
    } else {
        mclaurin(x, num_digits, |i, tot, power, _factorial| {
            if (i & 1) != 0 {
                let z = BigDecimal::from_i32(if (i & 2) != 0 { -i } else { i }).unwrap();
                Some(tot + power / &z)
            } else {
                None
            }
        })
    }
}

fn ln(mut x: BigDecimal, num_digits: i32) -> Option<BigDecimal> {
    if x.is_zero() || x.is_negative() {
        return None;
    }

    let mut extra = bigd(0);
    let h = half();
    if &x >= &one() {
        // Reduce to exp(0)..exp(0.5)
        let lim = exp(h.clone(), num_digits);
        let scale = exp(-h.clone(), num_digits);
        while x >= lim {
            extra += &h;
            x *= &scale;
        }

        // ln(1 + x): https://en.wikipedia.org/wiki/Taylor_series
        Some(
            mclaurin(x - one(), num_digits, |i, tot, power, _factorial| {
                if i == 0 {
                    None
                } else {
                    let z = bigd(if (i & 1) == 0 { -i } else { i });
                    Some(tot + power / &z)
                }
            }) + extra,
        )
    } else {
        // Reduce to exp(-0.5)..exp(0)
        let lim = exp(-h.clone(), num_digits);
        let scale = exp(h.clone(), num_digits);
        while x <= lim {
            extra -= &h;
            x *= &scale;
        }

        // ln(1 - x): https://en.wikipedia.org/wiki/Taylor_series
        Some(
            mclaurin(one() - x, num_digits, |i, tot, power, _factorial| {
                if i == 0 {
                    None
                } else {
                    let z = bigd(-i);
                    Some(tot + power / &z)
                }
            }) + extra,
        )
    }
}

fn log(x: BigDecimal, base: BigDecimal, num_digits: i32) -> Option<BigDecimal> {
    if let (Some(lnx), Some(lnbase)) = (ln(x, num_digits), ln(base, num_digits)) {
        Some(lnx / lnbase)
    } else {
        None
    }
}

fn pow(x: BigDecimal, y: BigDecimal, num_digits: i32) -> Option<BigDecimal> {
    if let Some(lnx) = ln(x, num_digits) {
        Some(exp(y * lnx, num_digits))
    } else {
        None
    }
}

#[test]
fn test_functions() {
    use crate::expr;

    // assert_eq!(expr!((0.5).log(2)).eval(20).unwrap(), expr!(true));

    assert_eq!(
        expr!(((0.25).log(2) + 2).abs() < 1e-20).eval(20).unwrap(),
        expr!(true)
    );
    assert_eq!(
        expr!(((0.5).log(2) + 1).abs() < 1e-20).eval(20).unwrap(),
        expr!(true)
    );
    assert_eq!(
        expr!(((1).log(2) + 0).abs() < 1e-20).eval(20).unwrap(),
        expr!(true)
    );
    assert_eq!(
        expr!(((2).log(2) - 1).abs() < 1e-20).eval(20).unwrap(),
        expr!(true)
    );
    assert_eq!(
        expr!(((4).log(2) - 2).abs() < 1e-20).eval(20).unwrap(),
        expr!(true)
    );

    assert_eq!(
        expr!(((0.25).exp().ln() - 0.25).abs() < 1e-20)
            .eval(20)
            .unwrap(),
        expr!(true)
    );
    assert_eq!(
        expr!(((0.5).exp().ln() - 0.5).abs() < 1e-20)
            .eval(20)
            .unwrap(),
        expr!(true)
    );
    assert_eq!(
        expr!(((1).exp().ln() - 1).abs() < 1e-20).eval(20).unwrap(),
        expr!(true)
    );
    assert_eq!(
        expr!(((2).exp().ln() - 2).abs() < 1e-20).eval(20).unwrap(),
        expr!(true)
    );

    assert_eq!(
        expr!(((-0.25).exp().ln() + 0.25).abs() < 1e-20)
            .eval(20)
            .unwrap(),
        expr!(true)
    );
    assert_eq!(
        expr!(((-0.5).exp().ln() + 0.5).abs() < 1e-20)
            .eval(20)
            .unwrap(),
        expr!(true)
    );
    assert_eq!(
        expr!(((-1).exp().ln() + 1).abs() < 1e-20).eval(20).unwrap(),
        expr!(true)
    );
    assert_eq!(
        expr!(((-2).exp().ln() + 2).abs() < 1e-20).eval(20).unwrap(),
        expr!(true)
    );

    assert_eq!(
        expr!(((0.5).asin() - PI / 6).abs() < 1e-20)
            .eval(20)
            .unwrap(),
        expr!(true)
    );
    assert_eq!(
        expr!(((0.5).asin() - PI / 6).abs() < 1e-40)
            .eval(40)
            .unwrap(),
        expr!(true)
    );
    assert_eq!(
        expr!(((0.5).asin() - PI / 6).abs() < 1e-100)
            .eval(100)
            .unwrap(),
        expr!(true)
    );
    assert_eq!(
        expr!(((-0.5).asin() + PI / 6).abs() < 1e-20)
            .eval(20)
            .unwrap(),
        expr!(true)
    );

    assert_eq!(
        expr!(((1).atan() - PI / 4).abs() < 1e-20).eval(20).unwrap(),
        expr!(true)
    );
    assert_eq!(
        expr!(((1 / 5).atan() * 4 - (1 / 239).atan() - PI / 4).abs() < 1e-20)
            .eval(20)
            .unwrap(),
        expr!(true)
    );

    assert_eq!(
        expr!(((0.5).acos() - PI / 3).abs() < 1e-20)
            .eval(20)
            .unwrap(),
        expr!(true)
    );
    assert_eq!(
        expr!(((0.5).acos() - PI / 3).abs() < 1e-40)
            .eval(40)
            .unwrap(),
        expr!(true)
    );
    assert_eq!(
        expr!(((0.5).acos() - PI / 3).abs() < 1e-100)
            .eval(100)
            .unwrap(),
        expr!(true)
    );

    assert_eq!(
        expr!(((1).exp() - 2.71828182845904523536).abs() < 1e-20)
            .eval(20)
            .unwrap(),
        expr!(true)
    );
    assert_eq!(
        expr!(((PI / 2).cos() - 0).abs() < 1e-20).eval(20).unwrap(),
        expr!(true)
    );
    assert_eq!(
        expr!(((PI / 2).sin() - 1).abs() < 1e-20).eval(20).unwrap(),
        expr!(true)
    );
    assert_eq!(
        expr!(((PI / 3).cos() - 0.5).abs() < 1e-20)
            .eval(20)
            .unwrap(),
        expr!(true)
    );
    assert_eq!(
        expr!(((PI / 3).sin() - 0.866025403784438646762).abs() < 1e-19)
            .eval(20)
            .unwrap(),
        expr!(true)
    );
    assert_eq!(
        expr!(((0).sin() - 0).abs() < 1e-20).eval(20).unwrap(),
        expr!(true)
    );
    assert_eq!(
        expr!(((1).sin() - 0.841470984807896506653).abs() < 1e-20)
            .eval(20)
            .unwrap(),
        expr!(true)
    );
    assert_eq!(
        expr!(((-1).sin() + 0.841470984807896506653).abs() < 1e-20)
            .eval(20)
            .unwrap(),
        expr!(true)
    );
    assert_eq!(
        expr!(((0).cos() - 1).abs() < 1e-20).eval(20).unwrap(),
        expr!(true)
    );
    assert_eq!(
        expr!(((1).cos() - 0.540302305868139717401).abs() < 1e-20)
            .eval(20)
            .unwrap(),
        expr!(true)
    );
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
