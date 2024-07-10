use doctor_syn::Parity;
use doctor_syn::{expr, name};
use proc_macro2::TokenStream;
use quote::quote;

use crate::test::gen_test;
use crate::Config;

pub fn gen_atan2(num_terms: usize, config: &Config) -> TokenStream {
    let approx = expr!(z.atan())
        .approx(
            num_terms,
            (-std::f64::consts::PI / 8.0).tan(),
            (std::f64::consts::PI / 8.0).tan(),
            name!(z),
            Parity::Odd,
            config.num_digits(),
        )
        .unwrap()
        .use_number_type(config.number_type(), config.num_bits())
        .unwrap()
        .into_inner();

    quote!(
        //  q2        y       q1
        //     \      |      /
        //       \    |    /     . pi/8
        //         \  |  /   .
        //           \|/ .
        //    --------*--------x    
        //           /|\ .
        //         /  |  \   .
        //       /    |    \     . -pi/8
        //     /      |      \
        //   q3               q4
        //
        pub fn atan2(y: fty, x: fty) -> fty {
            // Both positive (q1)
            let x1 = x.abs();
            let y1 = y.abs();

            // x > y by swapping.
            let x2 = if x1 > y1 { x1 } else { y1 };
            let y2 = if x1 > y1 { y1 } else { x1 };

            // rotate by PI/8 to get -pi/8 <= atan(z) <= pi/8 (0.3741967)
            // Note sin(pi/8)/cos(pi/8) = tan(pi/8)
            let x3 : fty = TAN_PI_BY_8 * y2 + x2;
            let y3 : fty = (-TAN_PI_BY_8) * x2 + y2;

            // 0 <= z <= 1
            let z = y3 / x3;

            // Approximate atan(y/x)
            let a : fty = #approx + PI_BY_8;

            // panic!("x={:?} y={:?} x3={:?} y3={:?} z={:?} a={:?}", x, y, x3, y3, z, a);

            // Reverse reflections.
            let q1 : fty = if x1 > y1 { a } else { PI_BY_2 - a };
            let q12 : fty = if x >= 0.0 { q1 } else { PI - q1 };
            if y >= 0.0 { q12 } else { -q12 }

            // // Rotate by PI if y , 0 so that y > 0
            // let offset180 : fty = if y < 0.0 { -PI } else { PI };
            // let x1 : fty = if x < 0.0 { -x } else { x };
            // let y1 : fty = if x < 0.0 { -y } else { y };
            // // Rotate by PI/2 if x < 0
            // let offset1 : fty = if x < 0.0 { offset180 } else { 0.0 };
            // let offset90 : fty = if y < 0.0 { -PI_BY_2 } else { PI_BY_2 };
            // let x2 : fty = if y1.abs() > x1 { y1 } else { x1 };
            // let y2 : fty = if y1.abs() > x1 { -x1 } else { y1 };
            // let offset2 : fty = if y1.abs() > x1 { offset1 + offset90 } else { offset1 };
            // let x3 : fty = y2 / x2;
            // let y3 : fty = #approx ;
            // y3 + offset2
        }
    )
}

pub fn gen_asin(num_terms: usize, config: &Config) -> TokenStream {
    // 1/sqrt(2) so that (1.0-lim*lim).sqrt() = lim
    let lim = quote!(0.70710678118654752440);

    let approx = expr!(x.asin())
        .approx(
            num_terms,
            -0.70710678118654752440,
            0.70710678118654752440,
            name!(x),
            Parity::Odd,
            config.num_digits(),
        )
        .unwrap()
        .use_number_type(config.number_type(), config.num_bits())
        .unwrap()
        .into_inner();

    quote!(
        pub fn asin(arg: fty) -> fty {
            let LIM : fty = #lim;
            // The middle is linear, but the edges require a flip.
            let c : fty = if arg < 0.0 { -PI_BY_2 } else { PI_BY_2 };
            let s : fty = if arg < 0.0 { -1.0 } else { 1.0  };

            // Use arg in the middle and (1.0-arg*arg).sqrt() at the edges.
            let x : fty = if arg * arg < LIM * LIM { arg } else { (1.0-arg*arg).sqrt() };
            let y : fty = #approx ;
            if arg*arg < LIM * LIM { y } else { c - y * s }
        }
    )
}

pub fn gen_acos(num_terms: usize, config: &Config) -> TokenStream {
    let lim = quote!(0.70710678118654752440);

    let approx = expr!(x.asin())
        .approx(
            num_terms,
            -0.70710678118654752440,
            0.70710678118654752440,
            name!(x),
            Parity::Odd,
            config.num_digits(),
        )
        .unwrap()
        .use_number_type(config.number_type(), config.num_bits())
        .unwrap()
        .into_inner();

    quote!(
        pub fn acos(arg: fty) -> fty {
            let LIM : fty = #lim;
            let c : fty = if arg < 0.0 { PI } else { 0.0 };
            let s : fty = if arg < 0.0 { 1.0 } else { -1.0  };
            let x : fty = if arg * arg < LIM * LIM { arg } else { (1.0-arg*arg).sqrt() };
            let y : fty = #approx ;
            if arg*arg < LIM * LIM { PI_BY_2 - y } else { c - y * s }
        }
    )
}

pub fn gen_atan(num_terms: usize, config: &Config) -> TokenStream {
    let lim = quote!(1.0);

    let approx = expr!(x.atan())
        .approx(
            num_terms,
            -1.0,
            1.0,
            name!(x),
            Parity::Odd,
            config.num_digits(),
        )
        .unwrap()
        .use_number_type(config.number_type(), config.num_bits())
        .unwrap()
        .into_inner();

    quote!(
        pub fn atan(arg: fty) -> fty {
            let LIM : fty = #lim;
            let c : fty = if arg < 0.0 { -PI_BY_2 } else { PI_BY_2 };
            let x : fty = if arg.abs() < LIM { arg } else { arg.recip() };
            let y : fty = #approx ;
            if arg.abs() < LIM { y } else { c - y }
        }
    )
}

// Generate accurate sin, cos, tan, sin_cos.
// Return functions and tests.
// pub fn gen_inv_trig(config: &Config) -> (TokenStream, TokenStream) {
//     let atan2 = gen_atan2(16, config);
//     let asin = gen_asin(22, config);
//     let acos = gen_acos(22, config);
//     let atan = gen_atan(22, config);

//     let bit = (2.0_f64).powi(if config.num_bits() == 32 { 23 } else { 52 });

//     let test_asin = gen_test(
//         config,
//         quote!(test_asin),
//         quote!(x.asin()),
//         quote!(asin(x as fty) as f64),
//         bit * 9.0,
//         -0.99,
//         0.99,
//     );
//     let test_acos = gen_test(
//         config,
//         quote!(test_acos),
//         quote!(x.acos()),
//         quote!(acos(x as fty) as f64),
//         bit * 9.0,
//         -0.99,
//         0.99,
//     );
//     let test_atan = gen_test(
//         config,
//         quote!(test_atan),
//         quote!(x.atan()),
//         quote!(atan(x as fty) as f64),
//         bit * 2.0,
//         -2.0,
//         2.0,
//     );

//     let test_atan2_a = gen_test(
//         config,
//         quote!(test_atan2_a),
//         quote!(x.atan2(1.0)),
//         quote!(atan2(x as fty, 1.0) as f64),
//         bit * 3.0,
//         -1.0,
//         1.0,
//     );
//     let test_atan2_b = gen_test(
//         config,
//         quote!(test_atan2_b),
//         quote!(x.atan2(-1.0)),
//         quote!(atan2(x as fty, -1.0) as f64),
//         bit * 3.0,
//         -1.0,
//         1.0,
//     );
//     let test_atan2_c = gen_test(
//         config,
//         quote!(test_atan2_c),
//         quote!((1.0_f64).atan2(x)),
//         quote!(atan2(1.0, x as fty) as f64),
//         bit * 3.0,
//         -1.0,
//         1.0,
//     );
//     let test_atan2_d = gen_test(
//         config,
//         quote!(test_atan2_d),
//         quote!((-1.0_f64).atan2(x)),
//         quote!(atan2(-1.0, x as fty) as f64),
//         bit * 3.0,
//         -1.0,
//         1.0,
//     );

//     (
//         quote! {
//             #asin #acos #atan #atan2
//         },
//         quote! {
//             #test_asin #test_acos #test_atan #test_atan2_a #test_atan2_b #test_atan2_c #test_atan2_d
//         },
//     )
// }
