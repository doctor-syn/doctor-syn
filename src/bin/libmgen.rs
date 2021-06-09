use doctor_syn::Parity;
use doctor_syn::{expr, name};
use std::io::Write;
use quote::quote;

// TODO:
//
// acos
// atan
// cbrt
// hypot
// log
// powf
// powi
// recip
// sqrt


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
            let x0 = x;
            let x = if x * x < #LIM * #LIM { x } else { (1.0-x*x).sqrt() };
            let y = #approx ;
            let c = if x0 < 0.0 { -std::f32::consts::PI/2.0 } else { std::f32::consts::PI/2.0 };
            let s = if x0 < 0.0 { -1.0 } else { 1.0  };
            if x0*x0 < #LIM*#LIM { y } else { c - y * s }
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
    // write!(file, "\n{}\n", gen_acos(16))?;
    // write!(file, "\n{}\n", gen_atan(16))?;

    write!(file, "\n{}\n", gen_exp(7))?;
    write!(file, "\n{}\n", gen_exp2(7))?;
    write!(file, "\n{}\n", gen_exp_m1(7))?;

    write!(file, "\n{}\n", gen_ln(9))?;
    write!(file, "\n{}\n", gen_ln_1p(9))?;
    write!(file, "\n{}\n", gen_log2(9))?;
    write!(file, "\n{}\n", gen_log10(9))?;

    write!(file, "\n{}\n", gen_sinh(7))?;
    write!(file, "\n{}\n", gen_cosh(7))?;
    write!(file, "\n{}\n", gen_tanh(7))?;

    write!(file, "\n{}\n", gen_asinh(7))?;
    write!(file, "\n{}\n", gen_acosh(7))?;
    write!(file, "\n{}\n", gen_atanh(7))?;

    write!(file, "\n{}\n", gen_sin_cos(9))?;
    write!(file, "\n{}\n", gen_atan2(16))?;

    let ulp = (2.0_f64).powi(-23);
    write!(file, "\n{}\n", gen_test("sin", "x.sin()", "sin(x as f32) as f64", ulp*6.0, -std::f64::consts::PI, std::f64::consts::PI))?;
    write!(file, "\n{}\n", gen_test("cos", "x.cos()", "cos(x as f32) as f64", ulp*3.0, -std::f64::consts::PI, std::f64::consts::PI))?;
    write!(file, "\n{}\n", gen_test("tan_a", "x.tan()", "tan(x as f32) as f64", ulp*2.0, -std::f64::consts::PI/4.0, std::f64::consts::PI/4.0))?;
    write!(file, "\n{}\n", gen_test("tan_b", "x.tan()", "tan(x as f32) as f64", ulp*7.0, -std::f64::consts::PI/3.0, std::f64::consts::PI/3.0))?;

    write!(file, "\n{}\n", gen_test("asin", "x.asin()", "asin(x as f32) as f64", ulp*9.0, -0.999, 0.999))?;

    write!(file, "\n{}\n", gen_test("exp_a", "x.exp()", "exp(x as f32) as f64", ulp*3.0, 0.0, 1.0))?;
    write!(file, "\n{}\n", gen_test("exp_b", "x.exp()", "exp(x as f32) as f64", ulp*10.0, 1.0, 2.0))?;
    write!(file, "\n{}\n", gen_test("exp_m1", "x.exp_m1()", "exp_m1(x as f32) as f64", ulp*3.0, 0.0, 1.0))?;
    write!(file, "\n{}\n", gen_test("exp2", "x.exp2()", "exp2(x as f32) as f64", ulp*2.0, 0.0, 1.0))?;

    write!(file, "\n{}\n", gen_test("ln", "x.ln()", "ln(x as f32) as f64", ulp*2.0, 1.0, std::f64::consts::E))?;
    write!(file, "\n{}\n", gen_test("ln_1p_a", "x.ln_1p()", "ln_1p(x as f32) as f64", ulp*2.0, 0.0, 1.0))?;
    write!(file, "\n{}\n", gen_test("ln_1p_b", "x.ln_1p()", "ln_1p(x as f32) as f64", ulp*3.0, 1.0, std::f64::consts::E*3.0-1.0))?;
    write!(file, "\n{}\n", gen_test("log2", "x.log2()", "log2(x as f32) as f64", ulp*2.0, 0.25, 4.25))?;
    write!(file, "\n{}\n", gen_test("log10", "x.log10()", "log10(x as f32) as f64", ulp*2.0, 0.1, 10.1))?;

    write!(file, "\n{}\n", gen_test("cosh", "x.cosh()", "cosh(x as f32) as f64", ulp*2.0, -1.0, 1.0))?;
    write!(file, "\n{}\n", gen_test("sinh", "x.sinh()", "sinh(x as f32) as f64", ulp*2.0, -1.0, 1.0))?;
    write!(file, "\n{}\n", gen_test("tanh", "x.tanh()", "tanh(x as f32) as f64", ulp*2.0, -1.0, 1.0))?;

    write!(file, "\n{}\n", gen_test("acosh", "x.acosh()", "acosh(x as f32) as f64", ulp*2.0, -1.0, 1.0))?;
    write!(file, "\n{}\n", gen_test("asinh", "x.asinh()", "asinh(x as f32) as f64", ulp*3.0, -1.0, 1.0))?;
    write!(file, "\n{}\n", gen_test("atanh", "x.atanh()", "atanh(x as f32) as f64", ulp*3.0, -0.9, 0.9))?;

    write!(file, "\n{}\n", gen_test("sin_cos_s", "x.sin_cos().0", "sin_cos(x as f32).0 as f64", ulp*6.0, -std::f64::consts::PI, std::f64::consts::PI))?;
    write!(file, "\n{}\n", gen_test("sin_cos_c", "x.sin_cos().1", "sin_cos(x as f32).1 as f64", ulp*3.0, -std::f64::consts::PI, std::f64::consts::PI))?;

    write!(file, "\n{}\n", gen_test("atan2_a", "x.atan2(1.0)", "atan2(x as f32, 1.0) as f64", ulp*3.0, -1.0, 1.0))?;
    write!(file, "\n{}\n", gen_test("atan2_b", "x.atan2(-1.0)", "atan2(x as f32, -1.0) as f64", ulp*3.0, -1.0, 1.0))?;
    write!(file, "\n{}\n", gen_test("atan2_c", "(1.0_f64).atan2(x)", "atan2(1.0, x as f32) as f64", ulp*3.0, -1.0, 1.0))?;
    write!(file, "\n{}\n", gen_test("atan2_d", "(-1.0_f64).atan2(x)", "atan2(-1.0, x as f32) as f64", ulp*3.0, -1.0, 1.0))?;

    Ok(())
}

fn main() {
    generate_libm("tests/libm.rs").unwrap();
}
