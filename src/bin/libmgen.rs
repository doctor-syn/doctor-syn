use doctor_syn::Parity;
use doctor_syn::{expr, name};
use std::io::Write;
use quote::quote;

// TODO:
//
// acos
// acosh
// asin
// asinh
// atan
// atan2
// atanh
// cbrt
// hypot
// log
// log10
// powf
// powi
// recip
// sqrt
// tanh


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

fn gen_sinh(_num_terms: usize) -> proc_macro2::TokenStream {
    // This could be done better with and explicit pair of polynomials.
    quote!(
        fn sinh(x: f32) -> f32 {
            let a = x.mul_add(std::f32::consts::LOG2_E, -1.0);
            let b = x.mul_add(-std::f32::consts::LOG2_E, -1.0);
            exp2(a) - exp2(b)
        }
    )
}

fn gen_cosh(_num_terms: usize) -> proc_macro2::TokenStream {
    // This could be done better with and explicit pair of polynomials.
    quote!(
        fn cosh(x: f32) -> f32 {
            let a = x.mul_add(std::f32::consts::LOG2_E, -1.0);
            let b = x.mul_add(-std::f32::consts::LOG2_E, -1.0);
            exp2(a) + exp2(b)
        }
    )
}

fn gen_test(test_name: &str, name: &str, accuracy: f64, tmin: f64, tmax: f64) -> String {
    let mut res = Vec::new();

    writeln!(res, "#[test]").unwrap();
    writeln!(res, "fn test_{}() {{", test_name).unwrap();
    writeln!(res, "    const N: i32 = 0x100000;").unwrap();
    writeln!(res, "    let tmin = {:20.16};", tmin).unwrap();
    writeln!(res, "    let tmax = {:20.16};", tmax).unwrap();
    writeln!(res, "    let mut max_error = 0.0_f64;").unwrap();
    writeln!(res, "    for i in 0..=N {{").unwrap();
    writeln!(res, "        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;").unwrap();
    writeln!(res, "        let y1 = x.{}();", name).unwrap();
    writeln!(res, "        let y2 = {}(x as f32) as f64;", name).unwrap();
    writeln!(res, "        max_error = max_error.max((y1 - y2).abs());").unwrap();
    writeln!(res, "        if i % (N/16) == 0 {{ println!(\"y1={{:20.16}}\\ny2={{:20.16}} e={{:20.16}}\", y1, y2, y2-y1); }}").unwrap();
    writeln!(res, "    }}").unwrap();
    writeln!(res, "    println!(\"{} me={{:20}}\", max_error);", name).unwrap();
    writeln!(res, "    assert!(max_error < {});", accuracy).unwrap();
    writeln!(res, "}}").unwrap();

    String::from_utf8(res).unwrap()
}

fn generate_libm(path: &str) -> std::io::Result<()> {
    let mut file = std::fs::File::create(path)?;

    write!(file, "\n{}\n", gen_sin(16))?;
    write!(file, "\n{}\n", gen_cos(17))?;
    write!(file, "\n{}\n", gen_exp(7))?;
    write!(file, "\n{}\n", gen_exp2(7))?;
    write!(file, "\n{}\n", gen_exp_m1(7))?;
    write!(file, "\n{}\n", gen_ln(9))?;
    write!(file, "\n{}\n", gen_ln_1p(9))?;
    write!(file, "\n{}\n", gen_log2(9))?;
    write!(file, "\n{}\n", gen_log10(9))?;
    write!(file, "\n{}\n", gen_sin_cos(9))?;
    write!(file, "\n{}\n", gen_tan(16))?;
    write!(file, "\n{}\n", gen_sinh(7))?;
    write!(file, "\n{}\n", gen_cosh(7))?;

    let ulp = (2.0_f64).powi(-23);
    write!(file, "\n{}\n", gen_test("sin", "sin", ulp*6.0, -std::f64::consts::PI, std::f64::consts::PI))?;
    write!(file, "\n{}\n", gen_test("cos", "cos", ulp*3.0, -std::f64::consts::PI, std::f64::consts::PI))?;
    write!(file, "\n{}\n", gen_test("tan_a", "tan", ulp*2.0, -std::f64::consts::PI/4.0, std::f64::consts::PI/4.0))?;
    write!(file, "\n{}\n", gen_test("tan_b", "tan", ulp*7.0, -std::f64::consts::PI/3.0, std::f64::consts::PI/3.0))?;
    write!(file, "\n{}\n", gen_test("exp_a", "exp", ulp*3.0, 0.0, 1.0))?;
    write!(file, "\n{}\n", gen_test("exp_b", "exp", ulp*10.0, 1.0, 2.0))?;
    write!(file, "\n{}\n", gen_test("exp_m1", "exp_m1", ulp*3.0, 0.0, 1.0))?;
    write!(file, "\n{}\n", gen_test("exp2", "exp2", ulp*2.0, 0.0, 1.0))?;
    write!(file, "\n{}\n", gen_test("ln", "ln", ulp*2.0, 1.0, std::f64::consts::E))?;
    write!(file, "\n{}\n", gen_test("ln_1p_a", "ln_1p", ulp*2.0, 0.0, 1.0))?;
    write!(file, "\n{}\n", gen_test("ln_1p_b", "ln_1p", ulp*3.0, 1.0, std::f64::consts::E*3.0-1.0))?;
    write!(file, "\n{}\n", gen_test("log2", "log2", ulp*2.0, 0.25, 4.25))?;
    write!(file, "\n{}\n", gen_test("log10", "log10", ulp*2.0, 0.1, 10.1))?;
    write!(file, "\n{}\n", gen_test("cosh", "cosh", ulp*2.0, -1.0, 1.0))?;
    write!(file, "\n{}\n", gen_test("sinh", "sinh", ulp*2.0, -1.0, 1.0))?;

    Ok(())
}

fn main() {
    generate_libm("tests/libm.rs").unwrap();
}
