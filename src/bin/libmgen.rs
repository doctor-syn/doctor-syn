use doctor_syn::Parity;
use doctor_syn::{expr, name};
use std::io::Write;
use quote::quote;

// acos
// acosh
// asin
// asinh
// atan
// atan2
// atanh
// cbrt
// cosh
// hypot
// ln_1p
// log
// log10
// log2
// powf
// powi
// recip
// sin_cos
// sinh
// sqrt
// tan
// tanh


fn gen_sin(num_terms: usize) -> proc_macro2::TokenStream {
    let xmin = -0.5;
    let xmax = 0.5;

    let approx = expr!((x * 3.1415926535897932384626433 * 2.0).sin() * -1.0)
        .approx(num_terms, xmin, xmax, name!(x), Parity::Odd)
        .unwrap()
        .use_suffix(Some("f32".to_string()))
        .unwrap()
        .into_inner();

    quote!(
        fn sin(x: f32) -> f32 {
            let x = x * (1.0 / (std::f32::consts::PI * 2.0));
            let x = x - x.floor() - 0.5;
            #approx
        }
    )
}

fn gen_cos(num_terms: usize) -> proc_macro2::TokenStream {
    let xmin = -0.5;
    let xmax = 0.5;

    let approx = expr!((x * 3.1415926535897932384626433 * 2.0).cos() * -1.0)
        .approx(num_terms, xmin, xmax, name!(x), Parity::Even)
        .unwrap()
        .use_suffix(Some("f32".to_string()))
        .unwrap()
        .into_inner();

    quote!(
        fn cos(x: f32) -> f32 {
            let x = x * (1.0 / (std::f32::consts::PI * 2.0));
            let x = x - x.floor() - 0.5;
            #approx
        }
    )
}

fn gen_exp2(num_terms: usize) -> proc_macro2::TokenStream {
    let xmin = -0.5;
    let xmax = 0.5;

    let approx = expr!(2.0.powf(x + 0.5))
        .approx(num_terms, xmin, xmax, name!(x), Parity::Neither)
        .unwrap()
        .use_suffix(Some("f32".to_string()))
        .unwrap()
        .into_inner();

    quote!(
        fn exp2(x: f32) -> f32 {
            let mul = f32::from_bits((x.floor() * 0x00800000 as f32 + 0x3f800000 as f32) as u32);
            let x = x - x.floor() - 0.5;
            #approx * mul
        }
    )
}

fn gen_exp(num_terms: usize) -> proc_macro2::TokenStream {
    let xmin = -0.5;
    let xmax = 0.5;

    let approx = expr!(2.0.powf(x + 0.5))
        .approx(num_terms, xmin, xmax, name!(x), Parity::Neither)
        .unwrap()
        .use_suffix(Some("f32".to_string()))
        .unwrap()
        .into_inner();

    quote!(
        fn exp(x: f32) -> f32 {
            let x = x * std::f32::consts::LOG2_E;
            let mul = f32::from_bits((x.floor() * 0x00800000 as f32 + 0x3f800000 as f32) as u32);
            let x = x - x.floor() - 0.5;
            #approx * mul
        }
    )
}

fn gen_exp_m1(num_terms: usize) -> proc_macro2::TokenStream {
    let xmin = -0.5;
    let xmax = 0.5;

    let approx = expr!(2.0.powf(x + 0.5)-1.0)
        .approx(num_terms, xmin, xmax, name!(x), Parity::Neither)
        .unwrap()
        .use_suffix(Some("f32".to_string()))
        .unwrap()
        .into_inner();

    quote!(
        fn exp_m1(x: f32) -> f32 {
            let x = x * std::f32::consts::LOG2_E;
            let mul = f32::from_bits((x.floor() * 0x00800000 as f32 + 0x3f800000 as f32) as u32);
            let x = x - x.floor() - 0.5;
            #approx * mul + (mul - 1.0)
        }
    )
}

fn gen_ln(num_terms: usize) -> proc_macro2::TokenStream {
    let xmin = -0.5;
    let xmax = 0.5;

    let approx = expr!((x + 1.5).log2())
        .approx(num_terms, xmin, xmax, name!(x), Parity::Neither)
        .unwrap()
        .use_suffix(Some("f32".to_string()))
        .unwrap()
        .into_inner();

    quote!(
        fn ln(x: f32) -> f32 {
            let exponent = (x.to_bits() >> 23) as i32 - 0x7f;
            let x = f32::from_bits((x.to_bits() & 0x7fffff) | 0x3f800000) - 1.5;
            let y: f32 = #approx;
            (y + (exponent as f32)) * (1.0 / std::f32::consts::LOG2_E)
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

fn generate_libm() -> std::io::Result<()> {
    let mut file = std::fs::File::create("tests/libm.rs")?;

    write!(file, "\n{}\n", gen_sin(16))?;
    write!(file, "\n{}\n", gen_cos(17))?;
    write!(file, "\n{}\n", gen_exp(7))?;
    write!(file, "\n{}\n", gen_exp2(7))?;
    write!(file, "\n{}\n", gen_exp_m1(7))?;
    write!(file, "\n{}\n", gen_ln(9))?;

    write!(file, "\n{}\n", gen_test("sin", "sin", (2.0_f64).powi(-23)*4.2, -std::f64::consts::PI, std::f64::consts::PI))?;
    write!(file, "\n{}\n", gen_test("cos", "cos", (2.0_f64).powi(-23)*3.0, -std::f64::consts::PI, std::f64::consts::PI))?;
    write!(file, "\n{}\n", gen_test("exp_a", "exp", (2.0_f64).powi(-23)*3.0, 0.0, 1.0))?;
    write!(file, "\n{}\n", gen_test("exp_b", "exp", (2.0_f64).powi(-23)*10.0, 1.0, 2.0))?;
    write!(file, "\n{}\n", gen_test("exp_m1", "exp_m1", (2.0_f64).powi(-23)*2.0, 0.0, 1.0))?;
    write!(file, "\n{}\n", gen_test("exp2", "exp2", (2.0_f64).powi(-23)*2.0, 0.0, 1.0))?;
    write!(file, "\n{}\n", gen_test("ln", "ln", (2.0_f64).powi(-23)*2.0, 1.0, std::f64::consts::E))?;

    Ok(())
}

fn main() {
    generate_libm().unwrap();
}
