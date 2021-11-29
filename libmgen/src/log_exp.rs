use doctor_syn::Parity;
use doctor_syn::{expr, name};
use proc_macro2::TokenStream;
use quote::quote;

use crate::Config;

pub fn gen_exp2(num_terms: usize, config: &Config) -> TokenStream {
    let one = config.get_one();
    let escale = config.get_escale();

    let xmin = -0.5;
    let xmax = 0.5;

    let approx = expr!(2.0.powf(x))
        .approx(
            num_terms,
            xmin,
            xmax,
            name!(x),
            Parity::Neither,
            config.num_digits(),
        )
        .unwrap()
        .use_number_type(config.number_type())
        .unwrap()
        .into_inner();

    quote!(
        pub fn exp2(arg: fty) -> fty {
            let r: fty = arg.round();
            let mul: fty = fty::from_bits((r.mul_add(#escale as fty, #one as fty)) as uty);
            let x: fty = arg - r;
            #approx * mul
        }
    )
}

pub fn gen_exp(_num_terms: usize, config: &Config) -> TokenStream {
    quote!(
        pub fn exp(arg: fty) -> fty {
            exp2(arg * LOG2_E)
        }
    )
}

pub fn gen_exp_m1(num_terms: usize, config: &Config) -> TokenStream {
    let one = config.get_one();
    let escale = config.get_escale();

    let xmin = -0.5;
    let xmax = 0.5;

    let approx = expr!(2.0.powf(x) - 1.0)
        .approx(
            num_terms,
            xmin,
            xmax,
            name!(x),
            Parity::Neither,
            config.num_digits(),
        )
        .unwrap()
        .use_number_type(config.number_type())
        .unwrap()
        .into_inner();

    quote!(
        pub fn exp_m1(arg: fty) -> fty {
            let scaled : fty = arg * LOG2_E;
            let r : fty = scaled.round();
            let mul : fty = fty::from_bits((r.mul_add(#escale as fty, #one as fty)) as uty);
            let x : fty = scaled - r;
            #approx * mul + (mul - 1.0)
        }
    )
}

pub fn gen_ln_1p(num_terms: usize, config: &Config) -> TokenStream {
    let xmin = 0.0;
    let xmax = 1.0;

    let approx = expr!((x + 1.0).log2())
        .approx(
            num_terms,
            xmin,
            xmax,
            name!(x),
            Parity::Neither,
            config.num_digits(),
        )
        .unwrap()
        .use_number_type(config.number_type())
        .unwrap()
        .into_inner();

    quote!(
        pub fn ln_1p(arg: fty) -> fty {
            //let exponent : uty = (arg_bits >> mkuty(#eshift)) - mkuty(#eoffset);
            let exponent : ity = ((arg+1.0).to_bits() >> 23) as ity - 0x7f;
            let x : fty = select(exponent == 0, arg, fty::from_bits(((arg+1.0).to_bits() & 0x7fffff) | 0x3f800000) - 1.0 );
            let y: fty = #approx;
            (y + (exponent as fty)) * (1.0 / LOG2_E)
        }
    )
}

pub fn gen_log2(num_terms: usize, config: &Config) -> TokenStream {
    let one = config.get_one();
    let escale = config.get_escale();
    let eshift = config.get_shift();
    let eoffset = config.get_eoffset();

    let xmin = -0.5;
    let xmax = 0.5;

    let approx = expr!((x + 1.5).log2())
        .approx(
            num_terms,
            xmin,
            xmax,
            name!(x),
            Parity::Neither,
            config.num_digits(),
        )
        .unwrap()
        .use_number_type(config.number_type())
        .unwrap()
        .into_inner();

    quote!(
        pub fn log2(arg: fty) -> fty {
            let arg_bits : uty = reinterpret_fty_uty(arg);
            let exponent : uty = (arg_bits >> mkuty(#eshift)) - mkuty(#eoffset);
            let x : fty = reinterpret_uty_fty((arg_bits & mkuty(#escale-1)) | mkuty(#one)) - f(1.5);
            let y : fty = #approx;
            y + (exponent as fty)
        }
    )
}

pub fn gen_ln(_num_terms: usize, _config: &Config) -> TokenStream {
    quote!(
        pub fn ln(arg: fty) -> fty {
            log2(arg) * f(1.0 / LOG2_E)
        }
    )
}

pub fn gen_log10(_num_terms: usize, _config: &Config) -> TokenStream {
    quote!(
        pub fn log10(arg: fty) -> fty {
            log2(arg) * (1.0 / LOG2_10)
        }
    )
}

pub fn gen_log(_num_terms: usize, _config: &Config) -> TokenStream {
    quote!(
        pub fn log(arg: fty, base: fty) -> fty {
            log2(arg) / log2(base)
        }
    )
}

pub fn gen_powf(_num_terms: usize, _config: &Config) -> TokenStream {
    quote!(
        pub fn powf(arg: fty, y: fty) -> fty {
            exp2(log2(arg) * y)
        }
    )
}

pub fn gen_powi(_num_terms: usize, _config: &Config) -> TokenStream {
    // Note, for constant values under 16, the code path is very short.
    quote!(
        pub fn powi(x: fty, y: ity) -> fty {
            // do 0..15 as multiplies.
            let a: fty = x;
            let p: ity = iabs(y);
            let b: fty = select((p & (1 << 0)) != 0, a, 1.0);
            let a1: fty = a * a;
            let b1: fty = select((p & (1 << 1)) != 0, b * a1, b);
            let a2: fty = a1 * a1;
            let b2: fty = select((p & (1 << 2)) != 0, b1 * a2, b1);
            let a3: fty = a2 * a2;
            let b3: fty = select((p & (1 << 3)) != 0, b2 * a3, b2);

            // do 16.. as logs.
            let b4: fty = select(p < 16, b3, powf(x, p as fty));

            // negative powers are reciprocals.
            select(y < 0, recip(b4), b4)
        }
    )
}

// pub fn gen_log_exp(config: &Config) -> (TokenStream, TokenStream) {
//     let exp = gen_exp(7, config);
//     let exp2 = gen_exp2(7, config);
//     let exp_m1 = gen_exp_m1(7, config);

//     let log2 = gen_log2(if config.num_bits() == 32 { 9 } else { 19 }, config);
//     let ln = gen_ln(9, config);
//     let ln_1p = gen_ln_1p(9, config);
//     let log10 = gen_log10(9, config);
//     let log = gen_log(9, config);
//     let powf = gen_powf(16, config);
//     let powi = gen_powi(16, config);

//     let bit = (2.0_f64).powi(if config.num_bits() == 32 { 23 } else { 52 });

//     let test_exp_a = gen_test(
//         config,
//         quote!(test_exp_a),
//         quote!(x.exp()),
//         quote!(exp(x as fty) as f64),
//         bit * 3.0,
//         0.0,
//         1.0,
//     );
//     let test_exp_b = gen_test(
//         config,
//         quote!(test_exp_b),
//         quote!(x.exp()),
//         quote!(exp(x as fty) as f64),
//         bit * 10.0,
//         1.0,
//         2.0,
//     );
//     let test_exp_m1 = gen_test(
//         config,
//         quote!(test_exp_m1),
//         quote!(x.exp_m1()),
//         quote!(exp_m1(x as fty) as f64),
//         bit * 3.0,
//         0.0,
//         1.0,
//     );
//     let test_exp2 = gen_test(
//         config,
//         quote!(test_exp2),
//         quote!(x.exp2()),
//         quote!(exp2(x as fty) as f64),
//         bit * 2.0,
//         0.0,
//         1.0,
//     );
//     // let test_exp2_x = gen_test(
//     //     config,
//     //     quote!(test_exp2_x),
//     //     quote!(x.exp2()),
//     //     quote!(exp2_approx(x as fty) as f64),
//     //     0.05,
//     //     0.0,
//     //     1.0,
//     // );

//     let test_ln = gen_test(
//         config,
//         quote!(test_ln),
//         quote!(x.ln()),
//         quote!(ln(x as fty) as f64),
//         bit * 2.0,
//         1.0,
//         std::f64::consts::E,
//     );
//     let test_ln_1p_a = gen_test(
//         config,
//         quote!(test_ln_1p_a),
//         quote!(x.ln_1p()),
//         quote!(ln_1p(x as fty) as f64),
//         bit * 2.0,
//         0.0,
//         1.0,
//     );
//     let test_ln_1p_b = gen_test(
//         config,
//         quote!(test_ln_1p_b),
//         quote!(x.ln_1p()),
//         quote!(ln_1p(x as fty) as f64),
//         bit * 3.0,
//         1.0,
//         std::f64::consts::E * 3.0 - 1.0,
//     );
//     let test_log2 = gen_test(
//         config,
//         quote!(test_log2),
//         quote!(x.log2()),
//         quote!(log2(x as fty) as f64),
//         bit * 2.0,
//         0.25,
//         4.25,
//     );
//     let test_log10 = gen_test(
//         config,
//         quote!(test_log10),
//         quote!(x.log10()),
//         quote!(log10(x as fty) as f64),
//         bit * 2.0,
//         0.1,
//         10.1,
//     );
//     let test_log_2 = gen_test(
//         config,
//         quote!(test_log_2),
//         quote!(x.log(2.0)),
//         quote!(log(x as fty, 2.0) as f64),
//         bit * 2.0,
//         0.5,
//         1.5,
//     );
//     let test_log_e = gen_test(
//         config,
//         quote!(test_log_e),
//         quote!(x.log(std::f64::consts::E)),
//         quote!(log(x as fty, std::f64::consts::E as fty) as f64),
//         bit * 2.0,
//         0.5,
//         1.5,
//     );

//     let test_powf_2 = gen_test(
//         config,
//         quote!(test_powf_2),
//         quote!(x.powf(2.0)),
//         quote!(powf(x as fty, 2.0) as f64),
//         bit * 4.0,
//         0.5,
//         1.5,
//     );
//     let test_powf_m1 = gen_test(
//         config,
//         quote!(test_powf_m1),
//         quote!(x.powf(-1.0)),
//         quote!(powf(x as fty, -1.0) as f64),
//         bit * 4.0,
//         0.5,
//         1.5,
//     );

//     let test_powi_2 = gen_test(
//         config,
//         quote!(test_powi_2),
//         quote!(x.powi(2)),
//         quote!(powi(x as fty, 2) as f64),
//         bit * 2.0,
//         0.5,
//         1.5,
//     );
//     let test_powi_3 = gen_test(
//         config,
//         quote!(test_powi_3),
//         quote!(x.powi(3)),
//         quote!(powi(x as fty, 3) as f64),
//         bit * 4.0,
//         0.12,
//         1.2,
//     );
//     let test_powi_m1 = gen_test(
//         config,
//         quote!(test_powi_m1),
//         quote!(x.powi(-1)),
//         quote!(powi(x as fty, -1) as f64),
//         bit * 2.0,
//         0.5,
//         1.5,
//     );
//     let test_powi_m2 = gen_test(
//         config,
//         quote!(test_powi_m2),
//         quote!(x.powi(-2)),
//         quote!(powi(x as fty, -2) as f64),
//         bit * 6.0,
//         0.5,
//         1.5,
//     );
//     let test_powi_16 = gen_test(
//         config,
//         quote!(test_powi_16),
//         quote!(x.powi(16)),
//         quote!(powi(x as fty, 16) as f64),
//         bit * 7.0,
//         0.25,
//         1.0,
//     );

//     (
//         quote!(
//             #exp2
//             #exp
//             #exp_m1
//             #log2
//             #ln
//             #ln_1p
//             #log10
//             #log
//             #powi
//             #powf
//         ),
//         quote!(
//             #test_exp_a
//             #test_exp_b
//             #test_exp_m1
//             #test_exp2
//             #test_ln
//             #test_ln_1p_a
//             #test_ln_1p_b
//             #test_log2
//             #test_log10
//             #test_log_2
//             #test_log_e

//             #test_powf_2
//             #test_powf_m1

//             #test_powi_2
//             #test_powi_3
//             #test_powi_m1
//             #test_powi_m2
//             #test_powi_16
//         ),
//     )
// }
