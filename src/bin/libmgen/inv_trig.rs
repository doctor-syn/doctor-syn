use doctor_syn::Parity;
use doctor_syn::{expr, name};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::test::gen_test;

pub fn gen_atan2(num_terms: usize, num_bits: usize) -> TokenStream {
    let suffix = format!("f{}", num_bits);
    let fty = format_ident!("f{}", num_bits);

    let xmin = -1.0;
    let xmax = 1.0;

    let approx = expr!(x.atan())
        .approx(num_terms, xmin, xmax, name!(x), Parity::Odd)
        .unwrap()
        .use_suffix(Some(suffix))
        .unwrap()
        .into_inner();

    // TODO: calculate the recipocal without a divide.
    quote!(
        fn atan2(y: #fty, x: #fty) -> #fty {
            use std::#fty::consts::PI;
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

pub fn gen_asin(num_terms: usize, num_bits: usize) -> TokenStream {
    let suffix = format!("f{}", num_bits);
    let fty = format_ident!("f{}", num_bits);
    let lim = quote!(0.9);

    let approx = expr!(x.asin())
        .approx(num_terms, -0.9, 0.9, name!(x), Parity::Odd)
        .unwrap()
        .use_suffix(Some(suffix))
        .unwrap()
        .into_inner();

    quote!(
        fn asin(x: #fty) -> #fty {
            use std::#fty::consts::PI;
            const LIM : #fty = #lim;
            let c = if x < 0.0 { -PI/2.0 } else { PI/2.0 };
            let s = if x < 0.0 { -1.0 } else { 1.0  };
            let x0 = x;
            let x = if x * x < LIM * LIM { x } else { (1.0-x*x).sqrt() };
            let y = #approx ;
            if x0*x0 < LIM * LIM { y } else { c - y * s }
        }
    )
}

pub fn gen_acos(num_terms: usize, num_bits: usize) -> TokenStream {
    let suffix = format!("f{}", num_bits);
    let fty = format_ident!("f{}", num_bits);
    let lim = quote!(0.9);

    let approx = expr!(x.asin())
        .approx(num_terms, -0.9, 0.9, name!(x), Parity::Odd)
        .unwrap()
        .use_suffix(Some(suffix))
        .unwrap()
        .into_inner();

    quote!(
        fn acos(x: #fty) -> #fty {
            use std::#fty::consts::PI;
            const LIM : #fty = #lim;
            let c = if x < 0.0 { PI } else { 0.0 };
            let s = if x < 0.0 { 1.0 } else { -1.0  };
            let x0 = x;
            let x = if x * x < LIM * LIM { x } else { (1.0-x*x).sqrt() };
            let y = #approx ;
            if x0*x0 < LIM * LIM { PI/2.0 - y } else { c - y * s }
        }
    )
}

pub fn gen_atan(num_terms: usize, num_bits: usize) -> TokenStream {
    let suffix = format!("f{}", num_bits);
    let fty = format_ident!("f{}", num_bits);
    let lim = quote!(1.0);

    let approx = expr!(x.atan())
        .approx(num_terms, -1.0, 1.0, name!(x), Parity::Odd)
        .unwrap()
        .use_suffix(Some(suffix))
        .unwrap()
        .into_inner();

    quote!(
        fn atan(x: #fty) -> #fty {
            use std::#fty::consts::PI;
            const LIM : #fty = #lim;

            let c = if x < 0.0 { -PI/2.0 } else { PI/2.0 };
            let small = x.abs() < LIM;
            let x = if small { x } else { x.recip() };
            let y = #approx ;
            if small { y } else { c - y }
        }
    )
}

// Generate accurate sin, cos, tan, sin_cos.
// Return functions and tests.
pub fn gen_inv_trig(num_bits: usize) -> (TokenStream, TokenStream) {
    let atan2 = gen_atan2(16, num_bits);
    let asin = gen_asin(22, num_bits);
    let acos = gen_acos(22, num_bits);
    let atan = gen_atan(22, num_bits);

    let fty = format_ident!("f{}", num_bits);

    let bit = (2.0_f64).powi(if num_bits == 32 { 23 } else { 52 });

    let test_asin = gen_test(
        quote!(test_asin),
        quote!(x.asin()),
        quote!(asin(x as #fty) as f64),
        bit * 9.0,
        -0.999,
        0.999,
    );
    let test_acos = gen_test(
        quote!(test_acos),
        quote!(x.acos()),
        quote!(acos(x as #fty) as f64),
        bit * 9.0,
        -0.999,
        0.999,
    );
    let test_atan = gen_test(
        quote!(test_atan),
        quote!(x.atan()),
        quote!(atan(x as #fty) as f64),
        bit * 2.0,
        -2.0,
        2.0,
    );

    let test_atan2_a = gen_test(
        quote!(test_atan2_a),
        quote!(x.atan2(1.0)),
        quote!(atan2(x as #fty, 1.0) as f64),
        bit * 3.0,
        -1.0,
        1.0,
    );
    let test_atan2_b = gen_test(
        quote!(test_atan2_b),
        quote!(x.atan2(-1.0)),
        quote!(atan2(x as #fty, -1.0) as f64),
        bit * 3.0,
        -1.0,
        1.0,
    );
    let test_atan2_c = gen_test(
        quote!(test_atan2_c),
        quote!((1.0_f64).atan2(x)),
        quote!(atan2(1.0, x as #fty) as f64),
        bit * 3.0,
        -1.0,
        1.0,
    );
    let test_atan2_d = gen_test(
        quote!(test_atan2_d),
        quote!((-1.0_f64).atan2(x)),
        quote!(atan2(-1.0, x as #fty) as f64),
        bit * 3.0,
        -1.0,
        1.0,
    );

    (
        quote! {
            #asin #acos #atan #atan2
        },
        quote! {
            #test_asin #test_acos #test_atan #test_atan2_a #test_atan2_b #test_atan2_c #test_atan2_d
        },
    )
}
