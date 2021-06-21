use std::io::Write;
use quote::{quote, format_ident};
use proc_macro2::TokenStream;

mod trig;
mod inv_trig;
mod aux;
mod log_exp;
mod hyperbolic;
mod recip_sqrt;

use trig::*;
use inv_trig::*;
use aux::*;
use log_exp::*;
use hyperbolic::*;
use recip_sqrt::*;

fn gen_test(test_name: &str, refexpr: TokenStream, expr: TokenStream, accuracy: f64, tmin: f64, tmax: f64) -> proc_macro2::TokenStream {
    // let mut res = Vec::new();

    // writeln!(res, "#[test]").unwrap();
    // writeln!(res, "fn test_{}() {{", test_name).unwrap();
    // writeln!(res, "    const N: i32 = 0x100000;").unwrap();
    // writeln!(res, "    let tmin = {:20.16};", tmin).unwrap();
    // writeln!(res, "    let tmax = {:20.16};", tmax).unwrap();
    // writeln!(res, "    let mut max_error = 0.0_f64;").unwrap();
    // writeln!(res, "    for i in 0..=N {{").unwrap();
    // writeln!(res, "        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;").unwrap();
    // writeln!(res, "        let y1 = {};", refexpr).unwrap();
    // writeln!(res, "        let y2 = {};", expr).unwrap();
    // writeln!(res, "        max_error = max_error.max((y1 - y2).abs());").unwrap();
    // // writeln!(res, "        if i % (N/16) == 0 {{ println!(\"y1={{:20.16}}\\ny2={{:20.16}} e={{:20.16}}\", y1, y2, y2-y1); }}").unwrap();
    // writeln!(res, "        if i % (N/16) == 0 {{ println!(\"y1={{:20.16}} y2={{:20.16}} e={{:20.16}}\", y1, y2, y2-y1); }}").unwrap();
    // writeln!(res, "    }}").unwrap();
    // writeln!(res, "    println!(\"{} me={{:20}}\", max_error);", expr).unwrap();
    // writeln!(res, "    assert!(!max_error.is_nan());").unwrap();
    // writeln!(res, "    assert!(max_error < {});", accuracy).unwrap();
    // writeln!(res, "}}").unwrap();

    let test_name = format_ident!("test_{}", test_name);

    quote!(
        #[test]
        fn #test_name() {
            const N: i32 = 0x100000;
            let tmin = #tmin;
            let tmax = #tmax;
            let mut max_error = 0.0_f64;
            for i in 0..=N {
                let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
                let y1 = #refexpr;
                let y2 = #expr;
                max_error = max_error.max((y1 - y2).abs());
                if i % (N/16) == 0 { println!("y1={:20.16} y2={:20.16} e={:20.16}", y1, y2, y2-y1); }
            }
            println!("me={:20}", max_error);
            assert!(!max_error.is_nan());
            assert!(max_error < #accuracy);
        }
    )
}

fn generate_libm(path: &str) -> std::io::Result<()> {
    let mut file = std::fs::File::create(path)?;

    // write!(file, "\n{}\n", gen_sin(16, false))?;
    gen_quadrant_trig(&mut file, 32)?;

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
    write!(file, "\n{}\n", gen_test("sin", quote!(x.sin()), quote!(sin(x as f32) as f64), bit*6.0, -std::f64::consts::PI, std::f64::consts::PI))?;
    write!(file, "\n{}\n", gen_test("cos", quote!(x.cos()), quote!(cos(x as f32) as f64), bit*3.0, -std::f64::consts::PI, std::f64::consts::PI))?;
    write!(file, "\n{}\n", gen_test("tan_a", quote!(x.tan()), quote!(tan(x as f32) as f64), bit*2.0, -std::f64::consts::PI/4.0, std::f64::consts::PI/4.0))?;
    write!(file, "\n{}\n", gen_test("tan_b", quote!(x.tan()), quote!(tan(x as f32) as f64), bit*7.0, -std::f64::consts::PI/3.0, std::f64::consts::PI/3.0))?;

    write!(file, "\n{}\n", gen_test("asin", quote!(x.asin()), quote!(asin(x as f32) as f64), bit*9.0, -0.999, 0.999))?;
    write!(file, "\n{}\n", gen_test("acos", quote!(x.acos()), quote!(acos(x as f32) as f64), bit*9.0, -0.999, 0.999))?;
    write!(file, "\n{}\n", gen_test("atan", quote!(x.atan()), quote!(atan(x as f32) as f64), bit*2.0, -2.0, 2.0))?;

    write!(file, "\n{}\n", gen_test("exp_a", quote!(x.exp()), quote!(exp(x as f32) as f64), bit*3.0, 0.0, 1.0))?;
    write!(file, "\n{}\n", gen_test("exp_b", quote!(x.exp()), quote!(exp(x as f32) as f64), bit*10.0, 1.0, 2.0))?;
    write!(file, "\n{}\n", gen_test("exp_m1", quote!(x.exp_m1()), quote!(exp_m1(x as f32) as f64), bit*3.0, 0.0, 1.0))?;
    write!(file, "\n{}\n", gen_test("exp2", quote!(x.exp2()), quote!(exp2(x as f32) as f64), bit*2.0, 0.0, 1.0))?;
    write!(file, "\n{}\n", gen_test("exp2_x", quote!(x.exp2()), quote!(exp2_approx(x as f32) as f64), 0.05, 0.0, 1.0))?;

    write!(file, "\n{}\n", gen_test("ln", quote!(x.ln()), quote!(ln(x as f32) as f64), bit*2.0, 1.0, std::f64::consts::E))?;
    write!(file, "\n{}\n", gen_test("ln_1p_a", quote!(x.ln_1p()), quote!(ln_1p(x as f32) as f64), bit*2.0, 0.0, 1.0))?;
    write!(file, "\n{}\n", gen_test("ln_1p_b", quote!(x.ln_1p()), quote!(ln_1p(x as f32) as f64), bit*3.0, 1.0, std::f64::consts::E*3.0-1.0))?;
    write!(file, "\n{}\n", gen_test("log2", quote!(x.log2()), quote!(log2(x as f32) as f64), bit*2.0, 0.25, 4.25))?;
    write!(file, "\n{}\n", gen_test("log10", quote!(x.log10()), quote!(log10(x as f32) as f64), bit*2.0, 0.1, 10.1))?;
    write!(file, "\n{}\n", gen_test("log_2", quote!(x.log(2.0)), quote!(log(x as f32, 2.0) as f64), bit*2.0, 0.5, 1.5))?;
    write!(file, "\n{}\n", gen_test("log_e", quote!(x.log(std::f64::consts::E)), quote!(log(x as f32, std::f32::consts::E) as f64), bit*2.0, 0.5, 1.5))?;
    write!(file, "\n{}\n", gen_test("log_2x", quote!(x.log2()), quote!(log2_approx(x as f32) as f64), 0.05, 0.5, 10.5))?;

    write!(file, "\n{}\n", gen_test("cosh", quote!(x.cosh()), quote!(cosh(x as f32) as f64), bit*2.0, -1.0, 1.0))?;
    write!(file, "\n{}\n", gen_test("sinh", quote!(x.sinh()), quote!(sinh(x as f32) as f64), bit*2.0, -1.0, 1.0))?;
    write!(file, "\n{}\n", gen_test("tanh", quote!(x.tanh()), quote!(tanh(x as f32) as f64), bit*2.0, -1.0, 1.0))?;

    write!(file, "\n{}\n", gen_test("acosh", quote!(x.acosh()), quote!(acosh(x as f32) as f64), bit*2.0, -1.0, 1.0))?;
    write!(file, "\n{}\n", gen_test("asinh", quote!(x.asinh()), quote!(asinh(x as f32) as f64), bit*3.0, -1.0, 1.0))?;
    write!(file, "\n{}\n", gen_test("atanh", quote!(x.atanh()), quote!(atanh(x as f32) as f64), bit*3.0, -0.9, 0.9))?;

    write!(file, "\n{}\n", gen_test("sin_cos_s", quote!(x.sin_cos().0), quote!(sin_cos(x as f32).0 as f64), bit*6.0, -std::f64::consts::PI, std::f64::consts::PI))?;
    write!(file, "\n{}\n", gen_test("sin_cos_c", quote!(x.sin_cos().1), quote!(sin_cos(x as f32).1 as f64), bit*3.0, -std::f64::consts::PI, std::f64::consts::PI))?;

    write!(file, "\n{}\n", gen_test("atan2_a", quote!(x.atan2(1.0)), quote!(atan2(x as f32, 1.0) as f64), bit*3.0, -1.0, 1.0))?;
    write!(file, "\n{}\n", gen_test("atan2_b", quote!(x.atan2(-1.0)), quote!(atan2(x as f32, -1.0) as f64), bit*3.0, -1.0, 1.0))?;
    write!(file, "\n{}\n", gen_test("atan2_c", quote!((1.0_f64).atan2(x)), quote!(atan2(1.0, x as f32) as f64), bit*3.0, -1.0, 1.0))?;
    write!(file, "\n{}\n", gen_test("atan2_d", quote!((-1.0_f64).atan2(x)), quote!(atan2(-1.0, x as f32) as f64), bit*3.0, -1.0, 1.0))?;

    write!(file, "\n{}\n", gen_test("hypot_a", quote!(x.hypot(1.0)), quote!(hypot(x as f32, 1.0) as f64), bit*3.0, -1.0, 1.0))?;
    write!(file, "\n{}\n", gen_test("hypot_b", quote!(x.hypot(-1.0)), quote!(hypot(x as f32, -1.0) as f64), bit*3.0, -1.0, 1.0))?;
    write!(file, "\n{}\n", gen_test("hypot_c", quote!((1.0_f64).hypot(x)), quote!(hypot(1.0, x as f32) as f64), bit*3.0, -1.0, 1.0))?;
    write!(file, "\n{}\n", gen_test("hypot_d", quote!((-1.0_f64).hypot(x)), quote!(hypot(-1.0, x as f32) as f64), bit*3.0, -1.0, 1.0))?;

    write!(file, "\n{}\n", gen_test("sqrt", quote!(x.sqrt()), quote!(sqrt(x as f32) as f64), bit*1.0, 0.5, 2.0))?;
    write!(file, "\n{}\n", gen_test("cbrt", quote!(x.cbrt()), quote!(cbrt(x as f32) as f64), bit*1.0, -2.0, 2.0))?;
    write!(file, "\n{}\n", gen_test("recip", quote!(x.recip()), quote!(recip(x as f32) as f64), bit*2.0, 0.5, 1.5))?;
    write!(file, "\n{}\n", gen_test("recip_n", quote!(x.recip()), quote!(recip(x as f32) as f64), bit*2.0, -1.5, -0.5))?;
    write!(file, "\n{}\n", gen_test("recip_x", quote!(x.recip()), quote!(recip_approx(x as f32) as f64), 0.1, 0.5, 1.5))?;
    write!(file, "\n{}\n", gen_test("recip_y", quote!(x.recip()), quote!(recip_approx(x as f32) as f64), 0.1, -1.5, -0.5))?;

    write!(file, "\n{}\n", gen_test("powf_2", quote!(x.powf(2.0)), quote!(powf(x as f32, 2.0) as f64), bit*4.0, 0.5, 1.5))?;
    write!(file, "\n{}\n", gen_test("powf_m1", quote!(x.powf(-1.0)), quote!(powf(x as f32, -1.0) as f64), bit*4.0, 0.5, 1.5))?;

    write!(file, "\n{}\n", gen_test("powi_2", quote!(x.powi(2)), quote!(powi(x as f32, 2) as f64), bit*2.0, 0.5, 1.5))?;
    write!(file, "\n{}\n", gen_test("powi_3", quote!(x.powi(3)), quote!(powi(x as f32, 3) as f64), bit*4.0, 0.12, 1.2))?;
    write!(file, "\n{}\n", gen_test("powi_m1", quote!(x.powi(-1)), quote!(powi(x as f32, -1) as f64), bit*2.0, 0.5, 1.5))?;
    write!(file, "\n{}\n", gen_test("powi_m2", quote!(x.powi(-2)), quote!(powi(x as f32, -2) as f64), bit*6.0, 0.5, 1.5))?;
    write!(file, "\n{}\n", gen_test("powi_16", quote!(x.powi(16)), quote!(powi(x as f32, 16) as f64), bit*7.0, 0.25, 1.0))?;
    //write!(file, "\n{}\n", gen_test("powi_16", quote!(x.powi(16)), quote!(powf(x as f32, 16.0) as f64), bit*6.0, 0.5, 1.5))?;

    Ok(())
}

fn main() {
    generate_libm("tests/libm.rs").unwrap();
}
