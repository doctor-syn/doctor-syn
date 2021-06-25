//! BigDecimal math support
//!

pub use bigdecimal::{BigDecimal, FromPrimitive, One, Signed, Zero};
pub use num_bigint::BigInt;

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

pub fn pi(num_digits: i64) -> BigDecimal {
    // let x : BigDecimal = "3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679".parse().unwrap();
    //round(x, num_digits as i64)

    // Machin series.
    (atan(one() / bigd(5), num_digits) * bigd(4) - atan(one() / bigd(239), num_digits)) * bigd(4)
}

pub fn bigd(i: i32) -> BigDecimal {
    BigDecimal::from_i32(i).unwrap()
}

pub fn bigdf(i: f64) -> BigDecimal {
    BigDecimal::from_f64(i).unwrap()
}

pub fn zero() -> BigDecimal {
    bigd(0)
}

pub fn one() -> BigDecimal {
    bigd(1)
}

pub fn half() -> BigDecimal {
    BigDecimal::from_f32(0.5).unwrap()
}

pub fn two() -> BigDecimal {
    bigd(2)
}

/// Return the number of decimal digits to calculate for a floating point size.
pub fn num_digits_for(num_bits: usize) -> i64 {
    if num_bits == 32 { 10 } else { 24 }
}

// Evaluate McLaurin series of exp-like functions (sin, cos, exp)
// Note that we would prefer to use round(), but there are bugs in BigDecimal.
fn mclaurin<F: FnMut(i32, &BigDecimal, &BigDecimal, &BigDecimal) -> Option<BigDecimal>>(
    x: BigDecimal,
    num_digits: i64,
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

pub fn sin(x: BigDecimal, num_digits: i64) -> BigDecimal {
    mclaurin(x, num_digits, |i, tot, power, factorial| match i & 3 {
        1 => Some(tot + power / factorial),
        3 => Some(tot - power / factorial),
        _ => None,
    })
}

pub fn cos(x: BigDecimal, num_digits: i64) -> BigDecimal {
    mclaurin(x, num_digits, |i, tot, power, factorial| match i & 3 {
        0 => Some(tot + power / factorial),
        2 => Some(tot - power / factorial),
        _ => None,
    })
}

pub fn exp(x: BigDecimal, num_digits: i64) -> BigDecimal {
    mclaurin(x, num_digits, |_i, tot, power, factorial| {
        Some(tot + power / factorial)
    })
}

pub fn tan(x: BigDecimal, num_digits: i64) -> BigDecimal {
    sin(x.clone(), num_digits) / cos(x, num_digits)
}

pub fn asin(x: BigDecimal, num_digits: i64) -> BigDecimal {
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

pub fn acos(x: BigDecimal, num_digits: i64) -> BigDecimal {
    pi(num_digits) / two() - asin(x, num_digits)
}

pub fn atan(x: BigDecimal, num_digits: i64) -> BigDecimal {
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

pub fn ln(mut x: BigDecimal, num_digits: i64) -> Option<BigDecimal> {
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

pub fn log(x: BigDecimal, base: BigDecimal, num_digits: i64) -> Option<BigDecimal> {
    if let (Some(lnx), Some(lnbase)) = (ln(x, num_digits), ln(base, num_digits)) {
        Some(lnx / lnbase)
    } else {
        None
    }
}

pub fn pow(x: BigDecimal, y: BigDecimal, num_digits: i64) -> Option<BigDecimal> {
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

