use proc_macro2::TokenStream;
use quote::quote;

// use crate::test::gen_test;
use crate::Config;

pub fn gen_sqrt(_num_terms: usize, _config: &Config) -> TokenStream {
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
        pub fn sqrt(x: fty) -> fty {
            let r: fty = sqrt_approx(x);
            let y: fty = r + (x - r * r) / (2.0 * r);
            y
        }
    )
}

pub fn gen_cbrt(_num_terms: usize, _config: &Config) -> TokenStream {
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
        pub fn cbrt(x: fty) -> fty {
            // initial estimate.
            let r : fty = fty::from_bits(
                ((x.abs().to_bits() as fty).mul_add(ONE_THIRD, EXP2_ONE * TWO_THIRDS)) as uty
            );

            // Newton-Raphson step.
            let r: fty = r + (x.abs() - r * r * r) / (3.0 * r * r);
            let r: fty = r + (x.abs() - r * r * r) / (3.0 * r * r);
            let r: fty = r + (x.abs() - r * r * r) / (3.0 * r * r);
            let r: fty = r + (x.abs() - r * r * r) / (3.0 * r * r);
            r.copysign(x)
        }
    )
}

pub fn gen_recip(_num_terms: usize, _config: &Config) -> TokenStream {
    // Probably better done with a reciprocal estimate and refinement.
    //
    // Given an estimate r of a reciprocal 1/x
    //
    // r' = x * ( 2.0 - x * r )
    //
    // is a better estimate.

    quote!(
        pub fn recip(x: fty) -> fty {
            //let r = exp2_approx(-log2_approx(x));
            let r: fty = recip_approx(x);
            let r1: fty = r * (2.0 - x * r);
            let r2: fty = r1 * (2.0 - x * r1);
            let r3: fty = r2 * (2.0 - x * r2);
            r3
        }
    )
}

pub fn gen_hypot(_num_terms: usize, _config: &Config) -> TokenStream {
    // see https://en.wikipedia.org/wiki/Hypot
    //
    quote!(
        pub fn hypot(x: fty, y: fty) -> fty {
            let xgty: bool = x.abs() > y.abs();
            let x2: fty = if xgty { x } else { y };
            let y2: fty = if xgty { y } else { x };
            if x2.abs() <= MIN_POSITIVE { x2 } else { x2.abs() * (1.0 + (y2 / x2) * (y2 / x2)).sqrt() }
        }
    )
}

// pub fn gen_recip_sqrt(config: &Config) -> (TokenStream, TokenStream) {
//     let sqrt = gen_sqrt(config);
//     let cbrt = gen_cbrt(config);
//     let hypot = gen_hypot(config);
//     let recip = gen_recip(config);

//     let bit = (2.0_f64).powi(if config.num_bits() == 32 { 23 } else { 52 });

//     let test_hypot_a = gen_test(
//         config,
//         quote!(test_hypot_a),
//         quote!(x.hypot(1.0)),
//         quote!(hypot(x as fty, 1.0) as f64),
//         bit * 3.0,
//         -1.0,
//         1.0,
//     );
//     let test_hypot_b = gen_test(
//         config,
//         quote!(test_hypot_b),
//         quote!(x.hypot(-1.0)),
//         quote!(hypot(x as fty, -1.0) as f64),
//         bit * 3.0,
//         -1.0,
//         1.0,
//     );
//     let test_hypot_c = gen_test(
//         config,
//         quote!(test_hypot_c),
//         quote!((1.0_f64).hypot(x)),
//         quote!(hypot(1.0, x as fty) as f64),
//         bit * 3.0,
//         -1.0,
//         1.0,
//     );
//     let test_hypot_d = gen_test(
//         config,
//         quote!(test_hypot_d),
//         quote!((-1.0_f64).hypot(x)),
//         quote!(hypot(-1.0, x as fty) as f64),
//         bit * 3.0,
//         -1.0,
//         1.0,
//     );

//     let test_sqrt = gen_test(
//         config,
//         quote!(test_sqrt),
//         quote!(x.sqrt()),
//         quote!(sqrt(x as fty) as f64),
//         bit * 1.0,
//         0.5,
//         2.0,
//     );
//     let test_cbrt = gen_test(
//         config,
//         quote!(test_cbrt),
//         quote!(x.cbrt()),
//         quote!(cbrt(x as fty) as f64),
//         bit * 1.0,
//         -2.0,
//         2.0,
//     );
//     let test_recip = gen_test(
//         config,
//         quote!(test_recip),
//         quote!(x.recip()),
//         quote!(recip(x as fty) as f64),
//         bit * 2.0,
//         0.5,
//         1.5,
//     );
//     let test_recip_n = gen_test(
//         config,
//         quote!(test_recip_n),
//         quote!(x.recip()),
//         quote!(recip(x as fty) as f64),
//         bit * 2.0,
//         -1.5,
//         -0.5,
//     );
//     let test_recip_x = gen_test(
//         config,
//         quote!(test_recip_x),
//         quote!(x.recip()),
//         quote!(recip_approx(x as fty) as f64),
//         0.1,
//         0.5,
//         1.5,
//     );
//     let test_recip_y = gen_test(
//         config,
//         quote!(test_recip_y),
//         quote!(x.recip()),
//         quote!(recip_approx(x as fty) as f64),
//         0.1,
//         -1.5,
//         -0.5,
//     );
//     (
//         quote!(
//             #sqrt
//             #cbrt
//             #hypot
//             #recip

//         ),
//         quote!(
//             #test_hypot_a
//             #test_hypot_b
//             #test_hypot_c
//             #test_hypot_d

//             #test_sqrt
//             #test_cbrt
//             #test_recip
//             #test_recip_n
//             #test_recip_x
//             #test_recip_y
//         ),
//     )
// }
