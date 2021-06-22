use doctor_syn::Parity;
use doctor_syn::{expr, name};
use quote::quote;

pub fn gen_exp2(num_terms: usize) -> proc_macro2::TokenStream {
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

pub fn gen_exp(_num_terms: usize) -> proc_macro2::TokenStream {
    quote!(
        fn exp(x: f32) -> f32 {
            exp2(x * std::f32::consts::LOG2_E)
        }
    )
}

pub fn gen_exp_m1(num_terms: usize) -> proc_macro2::TokenStream {
    let xmin = -0.5;
    let xmax = 0.5;

    let approx = expr!(2.0.powf(x) - 1.0)
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

pub fn gen_ln_1p(num_terms: usize) -> proc_macro2::TokenStream {
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

pub fn gen_log2(num_terms: usize) -> proc_macro2::TokenStream {
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

pub fn gen_ln(_num_terms: usize) -> proc_macro2::TokenStream {
    quote!(
        fn ln(x: f32) -> f32 {
            log2(x) * (1.0 / std::f32::consts::LOG2_E)
        }
    )
}

pub fn gen_log10(_num_terms: usize) -> proc_macro2::TokenStream {
    quote!(
        fn log10(x: f32) -> f32 {
            log2(x) * (1.0 / std::f32::consts::LOG2_10)
        }
    )
}

pub fn gen_log(_num_terms: usize) -> proc_macro2::TokenStream {
    quote!(
        fn log(x: f32, base: f32) -> f32 {
            log2(x) / log2(base)
        }
    )
}

pub fn gen_powf(_num_terms: usize) -> proc_macro2::TokenStream {
    quote!(
        fn powf(x: f32, y: f32) -> f32 {
            exp2(log2(x) * y)
        }
    )
}

pub fn gen_powi(_num_terms: usize) -> proc_macro2::TokenStream {
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
            if y < 0 {
                recip(b)
            } else {
                b
            }
        }
    )
}
