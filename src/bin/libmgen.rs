use doctor_syn::Parity;
use doctor_syn::{expr, name};
// use std::io::prelude::*;
use quote::quote;

fn sin(x: f32) -> f32 {
    let x = x * (1.0 / (std::f32::consts::PI * 2.0));
    let x = x - x.floor() - 0.5;
    (12.268859941019306_f32)
        .mul_add(x * x, -41.216241051002875_f32)
        .mul_add(x * x, 76.58672703334098_f32)
        .mul_add(x * x, -81.59746095374902_f32)
        .mul_add(x * x, 41.34151143437585_f32)
        .mul_add(x * x, -6.283184525811273_f32)
        * x
}

fn cos(x: f32) -> f32 {
    let x = x * (1.0 / (std::f32::consts::PI * 2.0));
    let x = x - x.floor() - 0.5;
    (-6.528658161778678_f32)
        .mul_add(x * x, 25.97327546892989_f32)
        .mul_add(x * x, -60.17118230813523_f32)
        .mul_add(x * x, 85.45091743827739_f32)
        .mul_add(x * x, -64.93918704099474_f32)
        .mul_add(x * x, 19.739206679356567_f32)
        .mul_add(x * x, -1.0000000000000007_f32)
}

fn exp(x: f32) -> f32 {
    let x = x * std::f32::consts::LOG2_E;
    let mul = f32::from_bits((x.floor() * 0x00800000 as f32 + 0x3f800000 as f32) as u32);
    let x = x - x.floor() - 0.5;
    (0.00021877504780304022_f32)
        .mul_add(x, 0.0018964605237938004_f32)
        .mul_add(x, 0.01360194957589631_f32)
        .mul_add(x, 0.07849305736942819_f32)
        .mul_add(x, 0.3397315896731585_f32)
        .mul_add(x, 0.980258206874906_f32)
        .mul_add(x, 1.414213562373095_f32)
        * mul
}

fn ln(x: f32) -> f32 {
    let exponent = (x.to_bits() >> 23) as i32 - 0x7f;
    let x = f32::from_bits((x.to_bits() & 0x7fffff) | 0x3f800000) - 1.5;
    let y: f32 = (-0.008874696649735232_f32)
        .mul_add(x, 0.01511708903698527_f32)
        .mul_add(x, -0.020698917295047616_f32)
        .mul_add(x, 0.03731541074473215_f32)
        .mul_add(x, -0.07127813300215657_f32)
        .mul_add(x, 0.14254471630463159_f32)
        .mul_add(x, -0.32059812016799316_f32)
        .mul_add(x, 0.9617954032360934_f32)
        .mul_add(x, 0.5849625007211562_f32);
    (y + (exponent as f32)) * (1.0 / std::f32::consts::LOG2_E)
}

fn gen_sin() -> proc_macro2::TokenStream {
    let xmin = -0.5;
    let xmax = 0.5;

    let approx = expr!((x * 3.1415926535897932384626433 * 2.0).sin() * -1.0)
        .approx(12, xmin, xmax, name!(x), Parity::Odd)
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

fn gen_cos() -> proc_macro2::TokenStream {
    let xmin = -0.5;
    let xmax = 0.5;

    let approx = expr!((x * 3.1415926535897932384626433 * 2.0).cos() * -1.0)
        .approx(13, xmin, xmax, name!(x), Parity::Even)
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

fn gen_exp() -> proc_macro2::TokenStream {
    let xmin = -0.5;
    let xmax = 0.5;

    let approx = expr!(2.0.powf(x + 0.5))
        .approx(7, xmin, xmax, name!(x), Parity::Neither)
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

fn gen_ln() -> proc_macro2::TokenStream {
    let xmin = -0.5;
    let xmax = 0.5;

    let approx = expr!((x + 1.5).log2())
        .approx(9, xmin, xmax, name!(x), Parity::Neither)
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

fn test_sin() {
    const N: i32 = 0x100000;
    let tmin = 0.0;
    let tmax = std::f64::consts::PI / 2.0;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.sin();
        let y2 = sin(x as f32) as f64;
        max_error = max_error.max((y1 - y2).abs());
    }
    println!("sin me={:20}", max_error);
}

fn test_cos() {
    const N: i32 = 0x100000;
    let tmin = 0.0;
    let tmax = std::f64::consts::PI / 2.0;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.cos();
        let y2 = cos(x as f32) as f64;
        max_error = max_error.max((y1 - y2).abs());
    }
    println!("cos me={:20}", max_error);
}

fn test_exp() {
    const N: i32 = 0x100000;
    let tmin = -1.0;
    let tmax = 0.0;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.exp();
        let y2 = exp(x as f32) as f64;
        max_error = max_error.max((y1 - y2).abs());
    }
    println!("exp me={:20}", max_error);
}

fn test_ln() {
    const N: i32 = 0x100000;
    let tmin = 1.0;
    let tmax = 16.0;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.ln();
        let y2 = ln(x as f32) as f64;
        max_error = max_error.max((y1 - y2).abs());
    }
    println!("ln  me={:20}", max_error);
}

fn generate_libm() {
    println!("\n{}\n", gen_sin());
    println!("\n{}\n", gen_cos());
    println!("\n{}\n", gen_exp());
    println!("\n{}\n", gen_ln());

    /*eprintln!("1 ulp = {}", f32::from_bits(0x3f800001) - 1.0);
    test_sin();
    test_cos();
    test_exp();
    test_ln();

    const N: i32 = 16;
    let tmin = 1.0;
    let tmax = 4.0;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        // let y1 = sin(x as f32) as f64;
        let y1 = x.ln();
        let y2 = ln(x as f32) as f64;
        max_error = max_error.max((y1 - y2).abs());
        // println!(
        //     "{:20.13} {:20.13} {:20.13} {:5.2}",
        //     x,
        //     y1,
        //     y2,
        //     ulp_diff2(y1, y2)
        // );
        eprintln!(
            "test i={:4} x={:20.7} y1={:20.7} y2={:20.7} ulp={:5.2} {:8x} {:8x}",
            i,
            x,
            y1,
            y2,
            //ulp_diff2(y1, y2),
            (y1 - y2) * 2.0_f64.powi(23),
            (y1 as f32).to_bits(),
            (y2 as f32).to_bits()
        );
    }
    println!("me={}", max_error);*/
}

fn main() {
    generate_libm();
}
