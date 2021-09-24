use crate::helpers;
use proc_macro2::TokenStream;
use quote::quote;

use crate::test::gen_test;

pub fn gen_sqrt(num_bits: usize, _number_type: &str) -> TokenStream {
    let fty = helpers::get_fty(num_bits);
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
        fn sqrt(x: #fty) -> #fty {
            let r = sqrt_approx(x);
            let y = r + (x - r * r) / (2.0 * r);
            y
        }
    )
}

pub fn gen_cbrt(num_bits: usize, _number_type: &str) -> TokenStream {
    let fty = helpers::get_fty(num_bits);
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
        fn cbrt(x: #fty) -> #fty {
            let r = cbrt_approx(x.abs());
            let y = r + (x.abs() - r * r * r) / (3.0 * r * r);
            y.copysign(x)
        }
    )
}

pub fn gen_recip(num_bits: usize, _number_type: &str) -> TokenStream {
    let fty = helpers::get_fty(num_bits);
    // Probably better done with a reciprocal estimate and refinement.
    //
    // Given an estimate r of a reciprocal 1/x
    //
    // r' = x * ( 2.0 - x * r )
    //
    // is a better estimate.

    quote!(
        fn recip(x: #fty) -> #fty {
            //let r = exp2_approx(-log2_approx(x));
            let r = recip_approx(x);
            let r = r * (2.0 - x * r);
            let r = r * (2.0 - x * r);
            let r = r * (2.0 - x * r);
            r
        }
    )
}

pub fn gen_hypot(num_bits: usize, _number_type: &str) -> TokenStream {
    let fty = helpers::get_fty(num_bits);

    // see https://en.wikipedia.org/wiki/Hypot
    //
    quote!(
        fn hypot(x: #fty, y: #fty) -> #fty {
            let (x, y) = if x.abs() > y.abs() { (x, y) } else { (y, x) };
            if x.abs() <= #fty::MIN_POSITIVE {
                x
            } else {
                x.abs() * (1.0 + (y / x) * (y / x)).sqrt()
            }
        }
    )
}

pub fn gen_recip_sqrt(num_bits: usize, number_type: &str) -> (TokenStream, TokenStream) {
    let fty = helpers::get_fty(num_bits);

    let sqrt = gen_sqrt(num_bits, number_type);
    let cbrt = gen_cbrt(num_bits, number_type);
    let hypot = gen_hypot(num_bits, number_type);
    let recip = gen_recip(num_bits, number_type);

    let bit = (2.0_f64).powi(if num_bits == 32 { 23 } else { 52 });

    let test_hypot_a = gen_test(
        quote!(test_hypot_a),
        quote!(x.hypot(1.0)),
        quote!(hypot(x as #fty, 1.0) as f64),
        bit * 3.0,
        -1.0,
        1.0,
    );
    let test_hypot_b = gen_test(
        quote!(test_hypot_b),
        quote!(x.hypot(-1.0)),
        quote!(hypot(x as #fty, -1.0) as f64),
        bit * 3.0,
        -1.0,
        1.0,
    );
    let test_hypot_c = gen_test(
        quote!(test_hypot_c),
        quote!((1.0_f64).hypot(x)),
        quote!(hypot(1.0, x as #fty) as f64),
        bit * 3.0,
        -1.0,
        1.0,
    );
    let test_hypot_d = gen_test(
        quote!(test_hypot_d),
        quote!((-1.0_f64).hypot(x)),
        quote!(hypot(-1.0, x as #fty) as f64),
        bit * 3.0,
        -1.0,
        1.0,
    );

    let test_sqrt = gen_test(
        quote!(test_sqrt),
        quote!(x.sqrt()),
        quote!(sqrt(x as #fty) as f64),
        bit * 1.0,
        0.5,
        2.0,
    );
    let test_cbrt = gen_test(
        quote!(test_cbrt),
        quote!(x.cbrt()),
        quote!(cbrt(x as #fty) as f64),
        bit * 1.0,
        -2.0,
        2.0,
    );
    let test_recip = gen_test(
        quote!(test_recip),
        quote!(x.recip()),
        quote!(recip(x as #fty) as f64),
        bit * 2.0,
        0.5,
        1.5,
    );
    let test_recip_n = gen_test(
        quote!(test_recip_n),
        quote!(x.recip()),
        quote!(recip(x as #fty) as f64),
        bit * 2.0,
        -1.5,
        -0.5,
    );
    let test_recip_x = gen_test(
        quote!(test_recip_x),
        quote!(x.recip()),
        quote!(recip_approx(x as #fty) as f64),
        0.1,
        0.5,
        1.5,
    );
    let test_recip_y = gen_test(
        quote!(test_recip_y),
        quote!(x.recip()),
        quote!(recip_approx(x as #fty) as f64),
        0.1,
        -1.5,
        -0.5,
    );
    (
        quote!(
            #sqrt
            #cbrt
            #hypot
            #recip

        ),
        quote!(
            #test_hypot_a
            #test_hypot_b
            #test_hypot_c
            #test_hypot_d

            #test_sqrt
            #test_cbrt
            #test_recip
            #test_recip_n
            #test_recip_x
            #test_recip_y
        ),
    )
}
