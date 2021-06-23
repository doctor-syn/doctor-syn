use doctor_syn::Parity;
use doctor_syn::{expr, name};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::test::*;

pub fn gen_exp2(num_terms: usize, num_bits: usize) -> TokenStream {
    let suffix = format!("f{}", num_bits);
    let fty = format_ident!("f{}", num_bits);

    let xmin = -0.5;
    let xmax = 0.5;

    let approx = expr!(2.0.powf(x))
        .approx(num_terms, xmin, xmax, name!(x), Parity::Neither)
        .unwrap()
        .use_suffix(Some(suffix))
        .unwrap()
        .into_inner();

    quote!(
        fn exp2(x: #fty) -> #fty {
            let r = x.round();
            let mul = #fty::from_bits((r.mul_add(0x00800000 as #fty, 0x3f800000 as #fty)) as u32);
            let x = x - r;
            #approx * mul
        }
    )
}

pub fn gen_exp(_num_terms: usize, num_bits: usize) -> TokenStream {
    let fty = format_ident!("f{}", num_bits);
    quote!(
        fn exp(x: #fty) -> #fty {
            exp2(x * std::#fty::consts::LOG2_E)
        }
    )
}

pub fn gen_exp_m1(num_terms: usize, num_bits: usize) -> TokenStream {
    let fty = format_ident!("f{}", num_bits);
    let suffix = format!("f{}", num_bits);

    let xmin = -0.5;
    let xmax = 0.5;

    let approx = expr!(2.0.powf(x) - 1.0)
        .approx(num_terms, xmin, xmax, name!(x), Parity::Neither)
        .unwrap()
        .use_suffix(Some(suffix))
        .unwrap()
        .into_inner();

    quote!(
        fn exp_m1(x: #fty) -> #fty {
            let x = x * std::#fty::consts::LOG2_E;
            let r = x.round();
            let mul = #fty::from_bits((r.mul_add(0x00800000 as #fty, 0x3f800000 as #fty)) as u32);
            let x = x - r;
            #approx * mul + (mul - 1.0)
        }
    )
}

pub fn gen_ln_1p(num_terms: usize, num_bits: usize) -> TokenStream {
    let fty = format_ident!("f{}", num_bits);
    let suffix = format!("f{}", num_bits);

    let xmin = 0.0;
    let xmax = 1.0;

    let approx = expr!((x + 1.0).log2())
        .approx(num_terms, xmin, xmax, name!(x), Parity::Neither)
        .unwrap()
        .use_suffix(Some(suffix))
        .unwrap()
        .into_inner();

    quote!(
        fn ln_1p(x: #fty) -> #fty {
            let exponent = ((x+1.0).to_bits() >> 23) as i32 - 0x7f;
            let x = if exponent == 0 {x} else { #fty::from_bits(((x+1.0).to_bits() & 0x7fffff) | 0x3f800000) - 1.0 };
            let y: #fty = #approx;
            (y + (exponent as #fty)) * (1.0 / std::#fty::consts::LOG2_E)
        }
    )
}

pub fn gen_log2(num_terms: usize, num_bits: usize) -> TokenStream {
    let fty = format_ident!("f{}", num_bits);
    let suffix = format!("f{}", num_bits);

    let xmin = -0.5;
    let xmax = 0.5;

    let approx = expr!((x + 1.5).log2())
        .approx(num_terms, xmin, xmax, name!(x), Parity::Neither)
        .unwrap()
        .use_suffix(Some(suffix))
        .unwrap()
        .into_inner();

    quote!(
        fn log2(x: #fty) -> #fty {
            let exponent = (x.to_bits() >> 23) as i32 - 0x7f;
            let x = #fty::from_bits((x.to_bits() & 0x7fffff) | 0x3f800000) - 1.5;
            let y: #fty = #approx;
            y + (exponent as #fty)
        }
    )
}

pub fn gen_ln(_num_terms: usize, num_bits: usize) -> TokenStream {
    let fty = format_ident!("f{}", num_bits);

    quote!(
        fn ln(x: #fty) -> #fty {
            log2(x) * (1.0 / std::#fty::consts::LOG2_E)
        }
    )
}

pub fn gen_log10(_num_terms: usize, num_bits: usize) -> TokenStream {
    let fty = format_ident!("f{}", num_bits);

    quote!(
        fn log10(x: #fty) -> #fty {
            log2(x) * (1.0 / std::#fty::consts::LOG2_10)
        }
    )
}

pub fn gen_log(_num_terms: usize, num_bits: usize) -> TokenStream {
    let fty = format_ident!("f{}", num_bits);

    quote!(
        fn log(x: #fty, base: #fty) -> #fty {
            log2(x) / log2(base)
        }
    )
}

pub fn gen_powf(_num_terms: usize, num_bits: usize) -> TokenStream {
    let fty = format_ident!("f{}", num_bits);

    quote!(
        fn powf(x: #fty, y: #fty) -> #fty {
            exp2(log2(x) * y)
        }
    )
}

pub fn gen_powi(_num_terms: usize, num_bits: usize) -> TokenStream {
    let fty = format_ident!("f{}", num_bits);

    // Note, for constant values under 16, the code path is very short.
    quote!(
        fn powi(x: #fty, y: i32) -> #fty {
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
            let b = if p < 16 { b } else { powf(x, p as #fty) };

            // negative powers are reciprocals.
            if y < 0 {
                recip(b)
            } else {
                b
            }
        }
    )
}

pub fn gen_log_exp(num_bits: usize) -> (TokenStream, TokenStream) {
    let exp = gen_exp(7, num_bits);
    let exp2 = gen_exp2(7, num_bits);
    let exp_m1 = gen_exp_m1(7, num_bits);

    let ln = gen_ln(9, num_bits);
    let ln_1p = gen_ln_1p(9, num_bits);
    let log2 = gen_log2(9, num_bits);
    let log10 = gen_log10(9, num_bits);
    let log = gen_log(9, num_bits);
    let powf = gen_powf(16, num_bits);
    let powi = gen_powi(16, num_bits);

    let bit = (2.0_f64).powi(if num_bits == 32 { 23 } else { 52 });

    let test_exp_a = gen_test(
        quote!(test_exp_a),
        quote!(x.exp()),
        quote!(exp(x as f32) as f64),
        bit * 3.0,
        0.0,
        1.0,
    );
    let test_exp_b = gen_test(
        quote!(test_exp_b),
        quote!(x.exp()),
        quote!(exp(x as f32) as f64),
        bit * 10.0,
        1.0,
        2.0,
    );
    let test_exp_m1 = gen_test(
        quote!(test_exp_m1),
        quote!(x.exp_m1()),
        quote!(exp_m1(x as f32) as f64),
        bit * 3.0,
        0.0,
        1.0,
    );
    let test_exp2 = gen_test(
        quote!(test_exp2),
        quote!(x.exp2()),
        quote!(exp2(x as f32) as f64),
        bit * 2.0,
        0.0,
        1.0,
    );
    // let test_exp2_x = gen_test(
    //     quote!(test_exp2_x),
    //     quote!(x.exp2()),
    //     quote!(exp2_approx(x as f32) as f64),
    //     0.05,
    //     0.0,
    //     1.0,
    // );

    let test_ln = gen_test(
        quote!(test_ln),
        quote!(x.ln()),
        quote!(ln(x as f32) as f64),
        bit * 2.0,
        1.0,
        std::f64::consts::E,
    );
    let test_ln_1p_a = gen_test(
        quote!(test_ln_1p_a),
        quote!(x.ln_1p()),
        quote!(ln_1p(x as f32) as f64),
        bit * 2.0,
        0.0,
        1.0,
    );
    let test_ln_1p_b = gen_test(
        quote!(test_ln_1p_b),
        quote!(x.ln_1p()),
        quote!(ln_1p(x as f32) as f64),
        bit * 3.0,
        1.0,
        std::f64::consts::E * 3.0 - 1.0,
    );
    let test_log2 = gen_test(
        quote!(test_log2),
        quote!(x.log2()),
        quote!(log2(x as f32) as f64),
        bit * 2.0,
        0.25,
        4.25,
    );
    let test_log10 = gen_test(
        quote!(test_log10),
        quote!(x.log10()),
        quote!(log10(x as f32) as f64),
        bit * 2.0,
        0.1,
        10.1,
    );
    let test_log_2 = gen_test(
        quote!(test_log_2),
        quote!(x.log(2.0)),
        quote!(log(x as f32, 2.0) as f64),
        bit * 2.0,
        0.5,
        1.5,
    );
    let test_log_e = gen_test(
        quote!(test_log_e),
        quote!(x.log(std::f64::consts::E)),
        quote!(log(x as f32, std::f32::consts::E) as f64),
        bit * 2.0,
        0.5,
        1.5,
    );

    let test_powf_2 = gen_test(
        quote!(test_powf_2),
        quote!(x.powf(2.0)),
        quote!(powf(x as f32, 2.0) as f64),
        bit * 4.0,
        0.5,
        1.5,
    );
    let test_powf_m1 = gen_test(
        quote!(test_powf_m1),
        quote!(x.powf(-1.0)),
        quote!(powf(x as f32, -1.0) as f64),
        bit * 4.0,
        0.5,
        1.5,
    );

    let test_powi_2 = gen_test(
        quote!(test_powi_2),
        quote!(x.powi(2)),
        quote!(powi(x as f32, 2) as f64),
        bit * 2.0,
        0.5,
        1.5,
    );
    let test_powi_3 = gen_test(
        quote!(test_powi_3),
        quote!(x.powi(3)),
        quote!(powi(x as f32, 3) as f64),
        bit * 4.0,
        0.12,
        1.2,
    );
    let test_powi_m1 = gen_test(
        quote!(test_powi_m1),
        quote!(x.powi(-1)),
        quote!(powi(x as f32, -1) as f64),
        bit * 2.0,
        0.5,
        1.5,
    );
    let test_powi_m2 = gen_test(
        quote!(test_powi_m2),
        quote!(x.powi(-2)),
        quote!(powi(x as f32, -2) as f64),
        bit * 6.0,
        0.5,
        1.5,
    );
    let test_powi_16 = gen_test(
        quote!(test_powi_16),
        quote!(x.powi(16)),
        quote!(powi(x as f32, 16) as f64),
        bit * 7.0,
        0.25,
        1.0,
    );

    (
        quote!(
            #exp
            #exp2
            #exp_m1
            #ln
            #ln_1p
            #log2
            #log10
            #log
            #powi
            #powf
        ),
        quote!(
            #test_exp_a
            #test_exp_b
            #test_exp_m1
            #test_exp2
            #test_ln
            #test_ln_1p_a
            #test_ln_1p_b
            #test_log2
            #test_log10
            #test_log_2
            #test_log_e

            #test_powf_2
            #test_powf_m1

            #test_powi_2
            #test_powi_3
            #test_powi_m1
            #test_powi_m2
            #test_powi_16
        ),
    )
}
