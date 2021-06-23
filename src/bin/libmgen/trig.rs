use crate::test::gen_test;
use doctor_syn::Parity;
use doctor_syn::{expr, name};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::f64::consts::PI;

pub fn gen_quadrant_sin(num_terms: usize, num_bits: usize) -> TokenStream {
    let suffix = format!("f{}", num_bits);
    let fty = format_ident!("f{}", num_bits);

    // Quadrant sin/cos over a smaller range.
    let xmin = -0.25;
    let xmax = 0.25;

    let sin_approx = expr!((s * 3.1415926535897932384626433).sin())
        .approx(num_terms, xmin, xmax, name!(s), Parity::Odd)
        .unwrap()
        .use_suffix(Some(suffix.clone()))
        .unwrap()
        .into_inner();

    let cos_approx = expr!(-(c * 3.1415926535897932384626433).cos())
        .approx(num_terms + 1, xmin, xmax, name!(c), Parity::Even)
        .unwrap()
        .use_suffix(Some(suffix))
        .unwrap()
        .into_inner();

    quote!(
        fn sin(x: #fty) -> #fty {
            let x = x * (1.0 / (std::#fty::consts::PI));
            let xh = x + 0.5;
            let xr = x.round();
            let xhr = xh.round();
            let s = x - xr;
            let c = xh - xhr;
            let sr = #sin_approx;
            let cr = #cos_approx;
            let ss = if (xr as i32) & 1 == 0 { sr } else { -sr };
            let cs = if (xhr as i32 & 1) == 0 { cr } else { -cr };
            if s.abs() <= 0.25 { ss } else { cs }
        }
    )
}

pub fn gen_quadrant_cos(num_terms: usize, num_bits: usize) -> TokenStream {
    let suffix = format!("f{}", num_bits);
    let fty = format_ident!("f{}", num_bits);

    // Quadrant sin/cos over a smaller range.
    let xmin = -0.25;
    let xmax = 0.25;

    let sin_approx = expr!((s * 3.1415926535897932384626433).sin())
        .approx(num_terms, xmin, xmax, name!(s), Parity::Odd)
        .unwrap()
        .use_suffix(Some(suffix.clone()))
        .unwrap()
        .into_inner();

    let cos_approx = expr!((c * 3.1415926535897932384626433).cos())
        .approx(num_terms + 1, xmin, xmax, name!(c), Parity::Even)
        .unwrap()
        .use_suffix(Some(suffix))
        .unwrap()
        .into_inner();

    quote!(
        fn cos(x: #fty) -> #fty {
            let x = x * (1.0 / (std::#fty::consts::PI));
            let xh = x + 0.5;
            let xr = x.round();
            let xhr = xh.round();
            let c = x - xr;
            let s = xh - xhr;
            let sr = #sin_approx;
            let cr = #cos_approx;
            let ss = if xhr as i32 & 1 == 0 { sr } else { -sr };
            let cs = if xr as i32 & 1 == 0 { cr } else { -cr };
            if s.abs() <= 0.25 { ss } else { cs }
        }
    )
}

// pub fn gen_single_pass_sin(num_terms: usize, num_bits: usize) -> TokenStream {
//     let suffix = format!("f{}", num_bits);
//     let fty = format_ident!("f{}", num_bits);

//     let xmin = -0.5;
//     let xmax = 0.5;

//     let approx = expr!((x * 3.1415926535897932384626433 * 2.0).sin())
//         .approx(num_terms, xmin, xmax, name!(x), Parity::Odd)
//         .unwrap()
//         .use_suffix(Some(suffix))
//         .unwrap()
//         .into_inner();

//     quote!(
//         fn sin(x: #fty) -> #fty {
//             let x = x * (1.0 / (std::#fty::consts::PI * 2.0));
//             let x = x - x.round();
//             #approx
//         }
//     )
// }

// pub fn gen_single_pass_cos(num_terms: usize, num_bits: usize) -> TokenStream {
//     let suffix = format!("f{}", num_bits);
//     let fty = format_ident!("f{}", num_bits);

//     let xmin = -0.5;
//     let xmax = 0.5;

//     let approx = expr!((x * 3.1415926535897932384626433 * 2.0).cos())
//         .approx(num_terms, xmin, xmax, name!(x), Parity::Even)
//         .unwrap()
//         .use_suffix(Some(suffix))
//         .unwrap()
//         .into_inner();

//     quote!(
//         fn cos(x: #fty) -> #fty {
//             let x = x * (1.0 / (std::#fty::consts::PI * 2.0));
//             let x = x - x.round();
//             #approx
//         }
//     )
// }

pub fn gen_sin_cos(_num_terms: usize, num_bits: usize) -> TokenStream {
    let fty = format_ident!("f{}", num_bits);

    // There is some synergy between sin and cos, but not as much as ULP-focused approximants.
    quote!(
        fn sin_cos(x: #fty) -> (#fty, #fty) {
            (sin(x), cos(x))
        }
    )
}

pub fn gen_tan(num_terms: usize, num_bits: usize) -> TokenStream {
    let suffix = format!("f{}", num_bits);
    let fty = format_ident!("f{}", num_bits);

    // Use a Padé approximation. The expression (x*x - pi*pi/4) goes to zero at the poles
    // cancelling the infinities, similar to sinc(x).
    let xmin = -0.499999;
    let xmax = 0.499999;

    let approx = expr!((x * 3.1415926535897932384626433).tan() * (x * x - 0.25))
        .approx(num_terms, xmin, xmax, name!(x), Parity::Odd)
        .unwrap()
        .use_suffix(Some(suffix))
        .unwrap()
        .into_inner();

    // TODO: calculate the recipocal without a divide.
    quote!(
        fn tan(x: #fty) -> #fty {
            let x = x * (1.0 / (std::#fty::consts::PI));
            let x = x - x.round();
            let recip = 1.0 / (x*x - 0.25);
            let y = #approx ;
            y * recip
        }
    )
}

// Generate accurate sin, cos, tan, sin_cos.
// Return functions and tests.
pub fn gen_quadrant_trig(num_bits: usize) -> (TokenStream, TokenStream) {
    let sin = gen_quadrant_sin(8, num_bits);
    let cos = gen_quadrant_cos(8, num_bits);
    let tan = gen_tan(16, num_bits);
    let sin_cos = gen_sin_cos(9, num_bits);

    let fty = format_ident!("f{}", num_bits);

    let bit = (2.0_f64).powi(if num_bits == 32 { 23 } else { 52 });

    let test_sin = gen_test(
        quote!(test_sin),
        quote!(x.sin()),
        quote!(sin(x as #fty) as f64),
        bit * 2.0,
        -PI,
        PI,
    );
    let test_cos = gen_test(
        quote!(test_cos),
        quote!(x.cos()),
        quote!(cos(x as #fty) as f64),
        bit * 2.0,
        -PI,
        PI,
    );
    let test_tan = gen_test(
        quote!(test_tan),
        quote!(x.tan()),
        quote!(tan(x as #fty) as f64),
        bit * 2.0,
        -PI / 4.0,
        PI / 4.0,
    );
    let test_sin_cos_1 = gen_test(
        quote!(test_sin_cos_1),
        quote!(x.sin()),
        quote!(sin_cos(x as #fty).0 as f64),
        bit * 2.0,
        -PI,
        PI,
    );
    let test_sin_cos_2 = gen_test(
        quote!(test_sin_cos_2),
        quote!(x.cos()),
        quote!(sin_cos(x as #fty).1 as f64),
        bit * 2.0,
        -PI,
        PI,
    );

    (
        quote! {
            #sin #cos #tan #sin_cos
        },
        quote! {
            #test_sin #test_cos #test_tan #test_sin_cos_1 #test_sin_cos_2
        },
    )
}

// pub fn gen_single_pass_trig(file: &mut std::fs::File, num_bits: usize) -> std::io::Result<()> {
//     write!(file, "\n{}\n", gen_single_pass_sin(16, num_bits))?;
//     write!(file, "\n{}\n", gen_single_pass_cos(17, num_bits))?;
//     write!(file, "\n{}\n", gen_tan(16, num_bits))?;
//     write!(file, "\n{}\n", gen_sin_cos(9, num_bits))?;
//     Ok(())
// }
