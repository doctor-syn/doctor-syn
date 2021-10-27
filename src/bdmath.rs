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
    if num_digits < 100 {
        let x : BigDecimal = "3.141592653589793238462643383279502884197169399375105820974944592307816406286208998628034825342117067982148086513282306647093844609550582231725359408128481".parse().unwrap();
        round(x, num_digits as i64)
    } else {
        // Machin series.
        (atan(one() / bigd(5), num_digits) * bigd(4) - atan(one() / bigd(239), num_digits))
            * bigd(4)
    }
}

pub fn sqrt_two(num_digits: i64) -> BigDecimal {
    sqrt(two(), num_digits).unwrap()
}

pub fn one_over_root_two_pi(num_digits: i64) -> BigDecimal {
    sqrt(one() / (two() * pi(num_digits)), num_digits).unwrap()
}

pub fn sqrt(x: BigDecimal, _num_digits: i64) -> Option<BigDecimal> {
    // TODO: make our own sqrt as this has a fixed num_digits of 100.
    x.sqrt()
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
    if num_bits == 32 {
        10
    } else {
        24
    }
}

// Evaluate MacLaurin series of exp-like functions (sin, cos, exp)
// Note that we would prefer to use round(), but there are bugs in BigDecimal.
fn maclaurin<F: FnMut(i32, &BigDecimal, &BigDecimal, &BigDecimal) -> Option<BigDecimal>>(
    x: BigDecimal,
    num_digits: i64,
    mut f: F,
) -> BigDecimal {
    let mut power = BigDecimal::one();
    let mut factorial = BigDecimal::one();
    let mut tot = BigDecimal::zero();
    let mut i = 0;
    // let imax = x.digits() as i32 * 10;
    let imax = 30000;
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
            if i > imax {
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
    maclaurin(x, num_digits, |i, tot, power, factorial| match i & 3 {
        1 => Some(tot + power / factorial),
        3 => Some(tot - power / factorial),
        _ => None,
    })
}

pub fn cos(x: BigDecimal, num_digits: i64) -> BigDecimal {
    maclaurin(x, num_digits, |i, tot, power, factorial| match i & 3 {
        0 => Some(tot + power / factorial),
        2 => Some(tot - power / factorial),
        _ => None,
    })
}

pub fn exp(x: BigDecimal, num_digits: i64) -> BigDecimal {
    maclaurin(x, num_digits, |_i, tot, power, factorial| {
        Some(tot + power / factorial)
    })
}

pub fn tan(x: BigDecimal, num_digits: i64) -> BigDecimal {
    sin(x.clone(), num_digits) / cos(x, num_digits)
}

pub fn asin(x: BigDecimal, num_digits: i64) -> BigDecimal {
    if x.abs() > half() {
        // https://en.wikipedia.org/wiki/Inverse_trigonometric_functions
        atan((one() + &x * &x).sqrt().unwrap() / (one() + &x), num_digits) * two() * x.signum()
    } else {
        let mut numer = BigDecimal::one();
        let mut denom = BigDecimal::one();
        maclaurin(x, num_digits, |i, tot, power, _factorial| {
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
        maclaurin(x, num_digits, |i, tot, power, _factorial| {
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
            maclaurin(x - one(), num_digits, |i, tot, power, _factorial| {
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
            maclaurin(one() - x, num_digits, |i, tot, power, _factorial| {
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

/// erf(x) = 2*pnorm(sqrt(2)*x) - 1
/// https://en.wikipedia.org/wiki/Error_function#Asymptotic_expansion
/// 1       -3        10      -42      216
/// 1 * 1!  -3 * 1!   5 * 2!  -7 * 3!  9 * 4!
pub fn erf(x: BigDecimal, num_digits: i64) -> BigDecimal {
    two() / sqrt(pi(num_digits), num_digits).unwrap()
        * &x
        * maclaurin(&x * &x, num_digits, |i, tot, power, factorial| {
            match i & 1 {
                0 => Some(tot + power / (bigd(2 * i + 1) * factorial)),
                1 => Some(tot - power / (bigd(2 * i + 1) * factorial)),
                _ => None,
            }
        })
}

/// erfc(x) = 2*pnorm(-sqrt(2)*x)
/// https://en.wikipedia.org/wiki/Error_function#Asymptotic_expansion
/// 1       -3        10      -42      216
/// 1 * 1!  -3 * 1!   5 * 2!  -7 * 3!  9 * 4!
pub fn erfc(x: BigDecimal, num_digits: i64) -> BigDecimal {
    one() - erf(x, num_digits)
}

/// Normal distribution.
/// https://stat.ethz.ch/R-manual/R-devel/library/stats/html/Normal.html
/// https://en.wikipedia.org/wiki/Normal_distribution
pub fn dnorm(x: BigDecimal, mean: BigDecimal, sd: BigDecimal, num_digits: i64) -> BigDecimal {
    let x = (x - mean) / &sd;
    let k1 = one_over_root_two_pi(num_digits);
    k1 * exp(-&x * &x * half(), num_digits) / sd
}

/// Cumulative normal distribution.
///   erfc(x) = 2*pnorm(-sqrt(2)*x)
///   erfc(x)/2 = pnorm(-sqrt(2)*x)
///   erfc(-x/sqrt(2))/2 = pnorm(x)
///
///   erf(x) = 2*pnorm(sqrt(2)*x) - 1
///   (erf(x) + 1)/2 = pnorm(sqrt(2)*x)
///   (erf(x/sqrt(2)) + 1)/2 = pnorm(x)
pub fn pnorm(x: BigDecimal, mean: BigDecimal, sd: BigDecimal, num_digits: i64) -> BigDecimal {
    let x = (x - mean) / sd;
    (erf(x / two().sqrt().unwrap(), num_digits) + one()) * half()
}

/// Inverse cumulative normal distribution (probit).
/// https://en.wikipedia.org/wiki/Probit
/// qnorm(x) = sqrt(2)*inverfc(2*x)
/// https://en.wikipedia.org/wiki/Normal_distribution
/// Newton raphson:
/// x <- x - (pnorm(x) - y) / dnorm(x)
pub fn qnorm(
    x: BigDecimal,
    mean: BigDecimal,
    sd: BigDecimal,
    num_digits: i64,
) -> Option<BigDecimal> {
    if let Some(logit) = ln(&x / (one() - &x), num_digits) {
        let mut guess = logit * bigdf(0.6);
        loop {
            let pnorm = pnorm(guess.clone(), mean.clone(), sd.clone(), num_digits);
            let err = pnorm - &x;
            if round(err.abs(), num_digits).is_zero() {
                break;
            }
            let dnorm = dnorm(guess.clone(), mean.clone(), sd.clone(), num_digits);
            guess -= err / dnorm;
        }
        Some(round(guess, num_digits))
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

#[test]
fn test_stats_functions() {
    use crate::expr;

    // assert_eq!(erf(one(), 20), bigdf(0.8427008));
    // assert_eq!(erf(half(), 20), bigdf(0.52));

    // assert_eq!(pnorm(one(), two(), bigd(3), 20), bigdf(0.3694413));

    // let pnorm1 = pnorm(bigd(1), bigd(2), bigd(3), 100);
    // assert_eq!(qnorm(pnorm1, bigd(2), bigd(3), 100), Some(bigdf(1.0)));

    // assert_eq!(
    //     expr!(1.erf())
    //         .eval(100)
    //         .unwrap(),
    //     expr!(0.1257944)
    // );

    // assert_eq!(
    //     expr!(1.dnorm(2, 3))
    //         .eval(100)
    //         .unwrap(),
    //     expr!(0.1257944)
    // );

    assert_eq!(
        expr!(((1).erf() - 0.8427007929497148693412206350826092592960669979663029).abs() < 1e-20)
            .eval(20)
            .unwrap(),
        expr!(true)
    );

    assert_eq!(
        expr!(((1).dnorm(2, 3) - 0.12579440923099772133941284170576695075747160303).abs() < 1e-20)
            .eval(20)
            .unwrap(),
        expr!(true)
    );

    // R gives 0.36944134018176366663 which is good to 16 digits.
    assert_eq!(
        expr!(((1).pnorm(2, 3) - 0.369441340181763638272).abs() < 1e-20)
            .eval(20)
            .unwrap(),
        expr!(true)
    );

    assert_eq!(
        expr!((((1).pnorm(2, 3)).qnorm(2, 3) - 1).abs() < 1e-20)
            .eval(20)
            .unwrap(),
        expr!(true)
    );
}
