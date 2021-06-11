use doctor_syn::Parity;
use doctor_syn::{expr, name};
use std::io::Write;
use quote::quote;

fn gen_sin(num_terms: usize) -> proc_macro2::TokenStream {
    let xmin = -0.5;
    let xmax = 0.5;

    let approx = expr!((x * 3.1415926535897932384626433 * 2.0).sin())
        .approx(num_terms, xmin, xmax, name!(x), Parity::Odd)
        .unwrap()
        .use_suffix(Some("f32".to_string()))
        .unwrap()
        .into_inner();

    quote!(
        fn sin(x: f32) -> f32 {
            let x = x * (1.0 / (std::f32::consts::PI * 2.0));
            let x = x - x.round();
            #approx
        }
    )
}

fn gen_cos(num_terms: usize) -> proc_macro2::TokenStream {
    let xmin = -0.5;
    let xmax = 0.5;

    let approx = expr!((x * 3.1415926535897932384626433 * 2.0).cos())
        .approx(num_terms, xmin, xmax, name!(x), Parity::Even)
        .unwrap()
        .use_suffix(Some("f32".to_string()))
        .unwrap()
        .into_inner();

    quote!(
        fn cos(x: f32) -> f32 {
            let x = x * (1.0 / (std::f32::consts::PI * 2.0));
            let x = x - x.round();
            #approx
        }
    )
}

fn gen_sin_cos(_num_terms: usize) -> proc_macro2::TokenStream {
    // There is some synergy between sin and cos, but not as much as ULP-focused approximants.
    quote!(
        fn sin_cos(x: f32) -> (f32, f32) {
            (sin(x), cos(x))
        }
    )
}

fn gen_tan(num_terms: usize) -> proc_macro2::TokenStream {
    // Use a Padé approximation. The expression (x*x - pi*pi/4) goes to zero at the poles
    // cancelling the infinities, similar to sinc(x).
    let xmin = -0.499999;
    let xmax = 0.499999;

    let approx = expr!( (x * 3.1415926535897932384626433).tan() * (x*x - 0.25) )
        .approx(num_terms, xmin, xmax, name!(x), Parity::Odd)
        .unwrap()
        .use_suffix(Some("f32".to_string()))
        .unwrap()
        .into_inner();

    // TODO: calculate the recipocal without a divide.
    quote!(
        fn tan(x: f32) -> f32 {
            let x = x * (1.0 / (std::f32::consts::PI));
            let x = x - x.round();
            let recip = 1.0 / (x*x - 0.25);
            let y = #approx ;
            y * recip
        }
    )
}

fn gen_atan2(num_terms: usize) -> proc_macro2::TokenStream {
    let xmin = -1.0;
    let xmax = 1.0;

    let approx = expr!( x.atan() )
        .approx(num_terms, xmin, xmax, name!(x), Parity::Odd)
        .unwrap()
        .use_suffix(Some("f32".to_string()))
        .unwrap()
        .into_inner();

    // TODO: calculate the recipocal without a divide.
    quote!(
        fn atan2(y: f32, x: f32) -> f32 {
            use std::f32::consts::PI;
            let offset180 = if y < 0.0 { -PI } else { PI };
            let (x, y, offset) = if x < 0.0 { (-x, -y, offset180) } else { (x, y, 0.0) };
            let offset90 = if y < 0.0 { -PI/2.0 } else { PI/2.0 };
            let (x, y, offset) = if y.abs() > x { (y, -x, offset + offset90) } else { (x, y, offset) };
            let x = y / x;
            let y = #approx ;
            y + offset
        }
    )
}

fn gen_asin(num_terms: usize) -> proc_macro2::TokenStream {
    const LIM : f32 = 0.9;
    let approx = expr!( x.asin() )
        .approx(num_terms, -LIM, LIM, name!(x), Parity::Odd)
        .unwrap()
        .use_suffix(Some("f32".to_string()))
        .unwrap()
        .into_inner();

    quote!(
        fn asin(x: f32) -> f32 {
            use std::f32::consts::PI;
            let c = if x < 0.0 { -PI/2.0 } else { PI/2.0 };
            let s = if x < 0.0 { -1.0 } else { 1.0  };
            let x0 = x;
            let x = if x * x < #LIM * #LIM { x } else { (1.0-x*x).sqrt() };
            let y = #approx ;
            if x0*x0 < #LIM*#LIM { y } else { c - y * s }
        }
    )
}

fn gen_acos(num_terms: usize) -> proc_macro2::TokenStream {
    const LIM : f32 = 0.9;
    let approx = expr!( x.asin() )
        .approx(num_terms, -LIM, LIM, name!(x), Parity::Odd)
        .unwrap()
        .use_suffix(Some("f32".to_string()))
        .unwrap()
        .into_inner();

    quote!(
        fn acos(x: f32) -> f32 {
            use std::f32::consts::PI;
            let c = if x < 0.0 { PI } else { 0.0 };
            let s = if x < 0.0 { 1.0 } else { -1.0  };
            let x0 = x;
            let x = if x * x < #LIM * #LIM { x } else { (1.0-x*x).sqrt() };
            let y = #approx ;
            if x0*x0 < #LIM*#LIM { PI/2.0 - y } else { c - y * s }
        }
    )
}

fn gen_atan(num_terms: usize) -> proc_macro2::TokenStream {
    const LIM : f32 = 1.0;
    let approx = expr!( x.atan() )
        .approx(num_terms, -LIM, LIM, name!(x), Parity::Odd)
        .unwrap()
        .use_suffix(Some("f32".to_string()))
        .unwrap()
        .into_inner();

    quote!(
        fn atan(x: f32) -> f32 {
            use std::f32::consts::PI;
            let c = if x < 0.0 { -PI/2.0 } else { PI/2.0 };
            let small = x.abs() < #LIM;
            let x = if small { x } else { x.recip() };
            let y = #approx ;
            if small { y } else { c - y }
        }
    )
}

fn gen_exp2_approx(_num_terms: usize) -> proc_macro2::TokenStream {
    // A very approximate 2.pow(x) used for estimates +/- 0.05
    quote!(
        fn exp2_approx(x: f32) -> f32 {
            f32::from_bits((x.mul_add(0x00800000 as f32, 0x3f800000 as f32 - 0x00800000 as f32 * 0.04)) as u32)
        }
    )
}

fn gen_recip_approx(_num_terms: usize) -> proc_macro2::TokenStream {
    // A very approximate x.recip() used for estimates +/- 0.1
    quote!(
        fn recip_approx(x: f32) -> f32 {
            let y = f32::from_bits((0x3f800000 as f32 * 2.0 - (x.abs().to_bits() as f32)) as u32) - 0.08;
            if x < 0.0 { -y } else { y }
        }
    )
}

fn gen_log2_approx(_num_terms: usize) -> proc_macro2::TokenStream {
    // A very approximate x.log2() used for estimates.
    quote!(
        fn log2_approx(x: f32) -> f32 {
            let exponent = (x.to_bits() >> 23) as i32 - 0x7f;
            let x = f32::from_bits((x.to_bits() & 0x7fffff) | 0x3f800000) - 0.96;
            let y: f32 = x;
            y + (exponent as f32)
        }
    )
}

// fn gen_asin(num_terms: usize) -> proc_macro2::TokenStream {
//     quote!(
//         fn asin(x: f32) -> f32 {
//             atan2(x, (1.0-x*x).sqrt())
//         }
//     )
// }

fn gen_exp2(num_terms: usize) -> proc_macro2::TokenStream {
    let xmin = -0.5;
    let xmax = 0.5;

    let approx = expr!(2.0.powf(x))
        .approx(num_terms, xmin, xmax, name!(x), Parity::Neither)
        .unwrap()
        .use_suffix(Some("f32".to_string()))
        .unwrap()
        .into_inner();

    quote!(
        fn exp2(x: f32) -> f32 {
            let r = x.round();
            let mul = f32::from_bits((r.mul_add(0x00800000 as f32, 0x3f800000 as f32)) as u32);
            let x = x - r;
            #approx * mul
        }
    )
}

fn gen_exp(_num_terms: usize) -> proc_macro2::TokenStream {
    quote!(
        fn exp(x: f32) -> f32 {
            exp2(x * std::f32::consts::LOG2_E)
        }
    )
}

fn gen_exp_m1(num_terms: usize) -> proc_macro2::TokenStream {
    let xmin = -0.5;
    let xmax = 0.5;

    let approx = expr!(2.0.powf(x)-1.0)
        .approx(num_terms, xmin, xmax, name!(x), Parity::Neither)
        .unwrap()
        .use_suffix(Some("f32".to_string()))
        .unwrap()
        .into_inner();

    quote!(
        fn exp_m1(x: f32) -> f32 {
            let x = x * std::f32::consts::LOG2_E;
            let r = x.round();
            let mul = f32::from_bits((r.mul_add(0x00800000 as f32, 0x3f800000 as f32)) as u32);
            let x = x - r;
            #approx * mul + (mul - 1.0)
        }
    )
}

fn gen_ln_1p(num_terms: usize) -> proc_macro2::TokenStream {
    let xmin = 0.0;
    let xmax = 1.0;

    let approx = expr!((x + 1.0).log2())
        .approx(num_terms, xmin, xmax, name!(x), Parity::Neither)
        .unwrap()
        .use_suffix(Some("f32".to_string()))
        .unwrap()
        .into_inner();

    quote!(
        fn ln_1p(x: f32) -> f32 {
            let exponent = ((x+1.0).to_bits() >> 23) as i32 - 0x7f;
            let x = if exponent == 0 {x} else { f32::from_bits(((x+1.0).to_bits() & 0x7fffff) | 0x3f800000) - 1.0 };
            let y: f32 = #approx;
            (y + (exponent as f32)) * (1.0 / std::f32::consts::LOG2_E)
        }
    )
}

fn gen_log2(num_terms: usize) -> proc_macro2::TokenStream {
    let xmin = -0.5;
    let xmax = 0.5;

    let approx = expr!((x + 1.5).log2())
        .approx(num_terms, xmin, xmax, name!(x), Parity::Neither)
        .unwrap()
        .use_suffix(Some("f32".to_string()))
        .unwrap()
        .into_inner();

    quote!(
        fn log2(x: f32) -> f32 {
            let exponent = (x.to_bits() >> 23) as i32 - 0x7f;
            let x = f32::from_bits((x.to_bits() & 0x7fffff) | 0x3f800000) - 1.5;
            let y: f32 = #approx;
            y + (exponent as f32)
        }
    )
}

fn gen_ln(_num_terms: usize) -> proc_macro2::TokenStream {
    quote!(
        fn ln(x: f32) -> f32 {
            log2(x) * (1.0 / std::f32::consts::LOG2_E)
        }
    )
}

fn gen_log10(_num_terms: usize) -> proc_macro2::TokenStream {
    quote!(
        fn log10(x: f32) -> f32 {
            log2(x) * (1.0 / std::f32::consts::LOG2_10)
        }
    )
}

fn gen_log(_num_terms: usize) -> proc_macro2::TokenStream {
    quote!(
        fn log(x: f32, base: f32) -> f32 {
            log2(x) / log2(base)
        }
    )
}

// https://en.wikipedia.org/wiki/Hyperbolic_functions
fn gen_sinh(_num_terms: usize) -> proc_macro2::TokenStream {
    quote!(
        fn sinh(x: f32) -> f32 {
            let a = x.mul_add(std::f32::consts::LOG2_E, -1.0);
            let b = x.mul_add(-std::f32::consts::LOG2_E, -1.0);
            exp2(a) - exp2(b)
        }
    )
}

// https://en.wikipedia.org/wiki/Hyperbolic_functions
fn gen_cosh(_num_terms: usize) -> proc_macro2::TokenStream {
    quote!(
        fn cosh(x: f32) -> f32 {
            let a = x.mul_add(std::f32::consts::LOG2_E, -1.0);
            let b = x.mul_add(-std::f32::consts::LOG2_E, -1.0);
            exp2(a) + exp2(b)
        }
    )
}

// https://en.wikipedia.org/wiki/Hyperbolic_functions
fn gen_tanh(_num_terms: usize) -> proc_macro2::TokenStream {
    quote!(
        fn tanh(x: f32) -> f32 {
            let exp2x = exp2(x*(std::f32::consts::LOG2_E*2.0));
            (exp2x - 1.0) / (exp2x + 1.0)
        }
    )
}

// https://en.wikipedia.org/wiki/Inverse_hyperbolic_functions
fn gen_asinh(_num_terms: usize) -> proc_macro2::TokenStream {
    quote!(
        fn asinh(x: f32) -> f32 {
            ln(x + (x*x+1.0).sqrt())
        }
    )
}

// https://en.wikipedia.org/wiki/Inverse_hyperbolic_functions
fn gen_acosh(_num_terms: usize) -> proc_macro2::TokenStream {
    quote!(
        fn acosh(x: f32) -> f32 {
            ln(x + (x*x-1.0).sqrt())
        }
    )
}

// https://en.wikipedia.org/wiki/Inverse_hyperbolic_functions
fn gen_atanh(_num_terms: usize) -> proc_macro2::TokenStream {
    quote!(
        fn atanh(x: f32) -> f32 {
            (ln(1.0 + x) - ln(1.0 - x)) * 0.5
        }
    )
}

fn gen_sqrt(_num_terms: usize) -> proc_macro2::TokenStream {
    // Probably better done with a reciprocal estimate or bitcast log divide.
    //
    // Given an estimate r of a square root:
    //
    // if (r + e).pow(2) = x
    //
    // r.pow(2) + 2*r*e + e.pow(2) = x
    //
    // e = (x - r.pow(2)) / 2*r.pow(2) + O(e.pow(2))
    //
    // ie. the Babylonian!

    quote!(
        fn sqrt(x: f32) -> f32 {
            let r = exp2(log2(x) * (1.0/2.0));
            let y = r + (x - r*r) / (2.0*r);
            y
        }
    )
}

fn gen_cbrt(_num_terms: usize) -> proc_macro2::TokenStream {
    // Probably better done with a bitcast log divide.
    //
    // Given an estimate r of a cube root:
    //
    // if (r + e).pow(3) = x
    //
    // r.pow(3) + 3*r.pow(2)*e + 3*r*e.pow(2) + e.pow(3) = x
    //
    // e = (x - r.pow(3)) / 3*r.pow(2) + O(e.pow(2))

    quote!(
        fn cbrt(x: f32) -> f32 {
            let r = exp2(log2(x.abs()) * (1.0/3.0));
            let y = r + (x.abs() - r*r*r) / (3.0*r*r);
            if x < 0.0 { -y } else { y }
        }
    )
}

fn gen_recip(_num_terms: usize) -> proc_macro2::TokenStream {
    // Probably better done with a reciprocal estimate and refinement.
    //
    // Given an estimate r of a reciprocal 1/x
    //
    // r' = x * ( 2.0 - x * r )
    //
    // is a better estimate.

    quote!(
        fn recip(x: f32) -> f32 {
            //let r = exp2_approx(-log2_approx(x));
            let r = recip_approx(x);
            let r = r * ( 2.0 - x * r );
            let r = r * ( 2.0 - x * r );
            let r = r * ( 2.0 - x * r );
            r
        }
    )
}

fn gen_hypot(_num_terms: usize) -> proc_macro2::TokenStream {
    // see https://en.wikipedia.org/wiki/Hypot
    //
    quote!(
        fn hypot(x: f32, y: f32) -> f32 {
            let (x, y) = if x.abs() > y.abs() { (x, y) } else { (y, x) };
            if x.abs() <= f32::MIN_POSITIVE { x } else { x.abs()*(1.0 + (y/x)*(y/x)).sqrt() }
        }
    )
}

fn gen_powf(_num_terms: usize) -> proc_macro2::TokenStream {
    quote!(
        fn powf(x: f32, y: f32) -> f32 {
            exp2(log2(x) * y)
        }
    )
}

fn gen_powi(_num_terms: usize) -> proc_macro2::TokenStream {
    // Note, for constant values under 16, the code path is very short.
    quote!(
        fn powi(x: f32, y: i32) -> f32 {
            // do 0..15 as multiplies.
            let a = x;
            let p = y.abs();
            let b = if (p & (1 << 0)) != 0 { a } else { 1.0 };
            let a = a * a;
            let b = if (p & (1 << 1)) != 0 { b * a } else { b };
            let a = a * a;
            let b = if (p & (1 << 2)) != 0 { b * a } else { b };
            let a = a * a;
            let b = if (p & (1 << 3)) != 0 { b * a } else { b };

            // do 16.. as logs.
            let b = if p < 16 { b } else { powf(x, p as f32) };

            // negative powers are reciprocals.
            if y < 0 { recip(b) } else { b }
        }
    )
}

fn gen_test(test_name: &str, refexpr: &str, expr: &str, accuracy: f64, tmin: f64, tmax: f64) -> String {
    let mut res = Vec::new();

    writeln!(res, "#[test]").unwrap();
    writeln!(res, "fn test_{}() {{", test_name).unwrap();
    writeln!(res, "    const N: i32 = 0x100000;").unwrap();
    writeln!(res, "    let tmin = {:20.16};", tmin).unwrap();
    writeln!(res, "    let tmax = {:20.16};", tmax).unwrap();
    writeln!(res, "    let mut max_error = 0.0_f64;").unwrap();
    writeln!(res, "    for i in 0..=N {{").unwrap();
    writeln!(res, "        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;").unwrap();
    writeln!(res, "        let y1 = {};", refexpr).unwrap();
    writeln!(res, "        let y2 = {};", expr).unwrap();
    writeln!(res, "        max_error = max_error.max((y1 - y2).abs());").unwrap();
    writeln!(res, "        if i % (N/16) == 0 {{ println!(\"y1={{:20.16}}\\ny2={{:20.16}} e={{:20.16}}\", y1, y2, y2-y1); }}").unwrap();
    writeln!(res, "    }}").unwrap();
    writeln!(res, "    println!(\"{} me={{:20}}\", max_error);", expr).unwrap();
    writeln!(res, "    assert!(!max_error.is_nan());").unwrap();
    writeln!(res, "    assert!(max_error < {});", accuracy).unwrap();
    writeln!(res, "}}").unwrap();

    String::from_utf8(res).unwrap()
}

fn generate_libm(path: &str) -> std::io::Result<()> {
    let mut file = std::fs::File::create(path)?;

    write!(file, "\n{}\n", gen_sin(16))?;
    write!(file, "\n{}\n", gen_cos(17))?;
    write!(file, "\n{}\n", gen_tan(16))?;

    write!(file, "\n{}\n", gen_asin(22))?;
    write!(file, "\n{}\n", gen_acos(22))?;
    write!(file, "\n{}\n", gen_atan(22))?;

    write!(file, "\n{}\n", gen_exp(7))?;
    write!(file, "\n{}\n", gen_exp2(7))?;
    write!(file, "\n{}\n", gen_exp_m1(7))?;
    write!(file, "\n{}\n", gen_exp2_approx(1))?;

    write!(file, "\n{}\n", gen_ln(9))?;
    write!(file, "\n{}\n", gen_ln_1p(9))?;
    write!(file, "\n{}\n", gen_log2(9))?;
    write!(file, "\n{}\n", gen_log10(9))?;
    write!(file, "\n{}\n", gen_log(9))?;
    write!(file, "\n{}\n", gen_log2_approx(1))?;

    write!(file, "\n{}\n", gen_sinh(7))?;
    write!(file, "\n{}\n", gen_cosh(7))?;
    write!(file, "\n{}\n", gen_tanh(7))?;

    write!(file, "\n{}\n", gen_asinh(7))?;
    write!(file, "\n{}\n", gen_acosh(7))?;
    write!(file, "\n{}\n", gen_atanh(7))?;

    write!(file, "\n{}\n", gen_sin_cos(9))?;
    write!(file, "\n{}\n", gen_atan2(16))?;
    write!(file, "\n{}\n", gen_sqrt(16))?;
    write!(file, "\n{}\n", gen_cbrt(16))?;
    write!(file, "\n{}\n", gen_hypot(16))?;
    write!(file, "\n{}\n", gen_recip(16))?;
    write!(file, "\n{}\n", gen_recip_approx(1))?;
    write!(file, "\n{}\n", gen_powf(16))?;
    write!(file, "\n{}\n", gen_powi(16))?;

    // One bit of a f32 mantissa.
    let bit = (2.0_f64).powi(-23);
    write!(file, "\n{}\n", gen_test("sin", "x.sin()", "sin(x as f32) as f64", bit*6.0, -std::f64::consts::PI, std::f64::consts::PI))?;
    write!(file, "\n{}\n", gen_test("cos", "x.cos()", "cos(x as f32) as f64", bit*3.0, -std::f64::consts::PI, std::f64::consts::PI))?;
    write!(file, "\n{}\n", gen_test("tan_a", "x.tan()", "tan(x as f32) as f64", bit*2.0, -std::f64::consts::PI/4.0, std::f64::consts::PI/4.0))?;
    write!(file, "\n{}\n", gen_test("tan_b", "x.tan()", "tan(x as f32) as f64", bit*7.0, -std::f64::consts::PI/3.0, std::f64::consts::PI/3.0))?;

    write!(file, "\n{}\n", gen_test("asin", "x.asin()", "asin(x as f32) as f64", bit*9.0, -0.999, 0.999))?;
    write!(file, "\n{}\n", gen_test("acos", "x.acos()", "acos(x as f32) as f64", bit*9.0, -0.999, 0.999))?;
    write!(file, "\n{}\n", gen_test("atan", "x.atan()", "atan(x as f32) as f64", bit*2.0, -2.0, 2.0))?;

    write!(file, "\n{}\n", gen_test("exp_a", "x.exp()", "exp(x as f32) as f64", bit*3.0, 0.0, 1.0))?;
    write!(file, "\n{}\n", gen_test("exp_b", "x.exp()", "exp(x as f32) as f64", bit*10.0, 1.0, 2.0))?;
    write!(file, "\n{}\n", gen_test("exp_m1", "x.exp_m1()", "exp_m1(x as f32) as f64", bit*3.0, 0.0, 1.0))?;
    write!(file, "\n{}\n", gen_test("exp2", "x.exp2()", "exp2(x as f32) as f64", bit*2.0, 0.0, 1.0))?;
    write!(file, "\n{}\n", gen_test("exp2_x", "x.exp2()", "exp2_approx(x as f32) as f64", 0.05, 0.0, 1.0))?;

    write!(file, "\n{}\n", gen_test("ln", "x.ln()", "ln(x as f32) as f64", bit*2.0, 1.0, std::f64::consts::E))?;
    write!(file, "\n{}\n", gen_test("ln_1p_a", "x.ln_1p()", "ln_1p(x as f32) as f64", bit*2.0, 0.0, 1.0))?;
    write!(file, "\n{}\n", gen_test("ln_1p_b", "x.ln_1p()", "ln_1p(x as f32) as f64", bit*3.0, 1.0, std::f64::consts::E*3.0-1.0))?;
    write!(file, "\n{}\n", gen_test("log2", "x.log2()", "log2(x as f32) as f64", bit*2.0, 0.25, 4.25))?;
    write!(file, "\n{}\n", gen_test("log10", "x.log10()", "log10(x as f32) as f64", bit*2.0, 0.1, 10.1))?;
    write!(file, "\n{}\n", gen_test("log_2", "x.log(2.0)", "log(x as f32, 2.0) as f64", bit*2.0, 0.5, 1.5))?;
    write!(file, "\n{}\n", gen_test("log_e", "x.log(std::f64::consts::E)", "log(x as f32, std::f32::consts::E) as f64", bit*2.0, 0.5, 1.5))?;
    write!(file, "\n{}\n", gen_test("log_10", "x.log(10.0)", "log(x as f32, 10.0) as f64", bit*2.0, 0.5, 1.5))?;
    write!(file, "\n{}\n", gen_test("log_2x", "x.log2()", "log2_approx(x as f32) as f64", 0.05, 0.5, 10.5))?;

    write!(file, "\n{}\n", gen_test("cosh", "x.cosh()", "cosh(x as f32) as f64", bit*2.0, -1.0, 1.0))?;
    write!(file, "\n{}\n", gen_test("sinh", "x.sinh()", "sinh(x as f32) as f64", bit*2.0, -1.0, 1.0))?;
    write!(file, "\n{}\n", gen_test("tanh", "x.tanh()", "tanh(x as f32) as f64", bit*2.0, -1.0, 1.0))?;

    write!(file, "\n{}\n", gen_test("acosh", "x.acosh()", "acosh(x as f32) as f64", bit*2.0, -1.0, 1.0))?;
    write!(file, "\n{}\n", gen_test("asinh", "x.asinh()", "asinh(x as f32) as f64", bit*3.0, -1.0, 1.0))?;
    write!(file, "\n{}\n", gen_test("atanh", "x.atanh()", "atanh(x as f32) as f64", bit*3.0, -0.9, 0.9))?;

    write!(file, "\n{}\n", gen_test("sin_cos_s", "x.sin_cos().0", "sin_cos(x as f32).0 as f64", bit*6.0, -std::f64::consts::PI, std::f64::consts::PI))?;
    write!(file, "\n{}\n", gen_test("sin_cos_c", "x.sin_cos().1", "sin_cos(x as f32).1 as f64", bit*3.0, -std::f64::consts::PI, std::f64::consts::PI))?;

    write!(file, "\n{}\n", gen_test("atan2_a", "x.atan2(1.0)", "atan2(x as f32, 1.0) as f64", bit*3.0, -1.0, 1.0))?;
    write!(file, "\n{}\n", gen_test("atan2_b", "x.atan2(-1.0)", "atan2(x as f32, -1.0) as f64", bit*3.0, -1.0, 1.0))?;
    write!(file, "\n{}\n", gen_test("atan2_c", "(1.0_f64).atan2(x)", "atan2(1.0, x as f32) as f64", bit*3.0, -1.0, 1.0))?;
    write!(file, "\n{}\n", gen_test("atan2_d", "(-1.0_f64).atan2(x)", "atan2(-1.0, x as f32) as f64", bit*3.0, -1.0, 1.0))?;

    write!(file, "\n{}\n", gen_test("hypot_a", "x.hypot(1.0)", "hypot(x as f32, 1.0) as f64", bit*3.0, -1.0, 1.0))?;
    write!(file, "\n{}\n", gen_test("hypot_b", "x.hypot(-1.0)", "hypot(x as f32, -1.0) as f64", bit*3.0, -1.0, 1.0))?;
    write!(file, "\n{}\n", gen_test("hypot_c", "(1.0_f64).hypot(x)", "hypot(1.0, x as f32) as f64", bit*3.0, -1.0, 1.0))?;
    write!(file, "\n{}\n", gen_test("hypot_d", "(-1.0_f64).hypot(x)", "hypot(-1.0, x as f32) as f64", bit*3.0, -1.0, 1.0))?;

    write!(file, "\n{}\n", gen_test("sqrt", "x.sqrt()", "sqrt(x as f32) as f64", bit*1.0, 0.5, 2.0))?;
    write!(file, "\n{}\n", gen_test("cbrt", "x.cbrt()", "cbrt(x as f32) as f64", bit*1.0, -2.0, 2.0))?;
    write!(file, "\n{}\n", gen_test("recip", "x.recip()", "recip(x as f32) as f64", bit*2.0, 0.5, 1.5))?;
    write!(file, "\n{}\n", gen_test("recip_n", "x.recip()", "recip(x as f32) as f64", bit*2.0, -1.5, -0.5))?;
    write!(file, "\n{}\n", gen_test("recip_x", "x.recip()", "recip_approx(x as f32) as f64", 0.1, 0.5, 1.5))?;
    write!(file, "\n{}\n", gen_test("recip_y", "x.recip()", "recip_approx(x as f32) as f64", 0.1, -1.5, -0.5))?;

    write!(file, "\n{}\n", gen_test("powf_2", "x.powf(2.0)", "powf(x as f32, 2.0) as f64", bit*4.0, 0.5, 1.5))?;
    write!(file, "\n{}\n", gen_test("powf_m1", "x.powf(-1.0)", "powf(x as f32, -1.0) as f64", bit*4.0, 0.5, 1.5))?;

    write!(file, "\n{}\n", gen_test("powi_2", "x.powi(2)", "powi(x as f32, 2) as f64", bit*2.0, 0.5, 1.5))?;
    write!(file, "\n{}\n", gen_test("powi_3", "x.powi(3)", "powi(x as f32, 3) as f64", bit*4.0, 0.12, 1.2))?;
    write!(file, "\n{}\n", gen_test("powi_m1", "x.powi(-1)", "powi(x as f32, -1) as f64", bit*2.0, 0.5, 1.5))?;
    write!(file, "\n{}\n", gen_test("powi_m2", "x.powi(-2)", "powi(x as f32, -2) as f64", bit*6.0, 0.5, 1.5))?;
    write!(file, "\n{}\n", gen_test("powi_16", "x.powi(16)", "powi(x as f32, 16) as f64", bit*7.0, 0.25, 1.0))?;
    //write!(file, "\n{}\n", gen_test("powi_16", "x.powi(16)", "powf(x as f32, 16.0) as f64", bit*6.0, 0.5, 1.5))?;

    Ok(())
}

fn main() {
    generate_libm("tests/libm.rs").unwrap();
}
