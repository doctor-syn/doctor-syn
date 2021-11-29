use crate::Config;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use doctor_syn::expr;

#[allow(non_snake_case)]
pub fn gen_PI(_terms: usize, config: &Config) -> TokenStream {
    let value : syn::Expr = expr!(PI).eval(config.num_digits()).unwrap().into();
    quote!( const PI : fty = value; )
}

#[allow(non_snake_case)]
pub fn gen_RECIP_2PI(_terms: usize, config: &Config) -> TokenStream {
    let value : syn::Expr = expr!(1/(2*PI)).eval(config.num_digits()).unwrap().into();
    quote!( const RECIP_2PI : fty = #value; )
}

pub fn gen_fty(_terms: usize, config: &Config) -> TokenStream {
    let fty = format_ident!("f{}", config.num_bits());
    quote!( 
        #[allow(non_camel_case_types)]
        type fty = #fty; )
}

pub fn gen_ity(_terms: usize, config: &Config) -> TokenStream {
    let ity = format_ident!("i{}", config.num_bits());
    quote!( 
        #[allow(non_camel_case_types)]
        type ity = #ity; )
}

pub fn gen_uty(_terms: usize, config: &Config) -> TokenStream {
    let uty = format_ident!("u{}", config.num_bits());
    quote!(
        #[allow(non_camel_case_types)]
        type uty = #uty; )
}

pub fn gen_negate_on_odd(_terms: usize, config: &Config) -> TokenStream {
    let shift = (config.num_bits() - 1) as i32;

    quote!(
        // If x is odd, negate y.
        pub fn negate_on_odd(x: fty, y: fty) -> fty {
            let sign_bit : uty = (((x as ity) & 1) << #shift) as uty;
            fty::from_bits(sign_bit ^ y.to_bits())
        }
    )
}

pub fn _gen_negate_on_odd_test(_config: &Config) -> TokenStream {
    quote!(
        #[test]
        pub fn test_negate_on_odd() {
            assert_eq!(negate_on_odd(-4.0, 1.0), 1.0);
            assert_eq!(negate_on_odd(-3.0, 1.0), -1.0);
            assert_eq!(negate_on_odd(-2.0, 1.0), 1.0);
            assert_eq!(negate_on_odd(-1.0, 1.0), -1.0);
            assert_eq!(negate_on_odd(0.0, 1.0), 1.0);
            assert_eq!(negate_on_odd(1.0, 1.0), -1.0);
            assert_eq!(negate_on_odd(2.0, 1.0), 1.0);
            assert_eq!(negate_on_odd(3.0, 1.0), -1.0);
        }
    )
}

// pub fn gen_exp2_approx(config: &Config) -> TokenStream {
//
//     // A very approximate 2.pow(x) used for estimates +/- 0.05
//     quote!(
//         pub fn exp2_approx(x: fty) -> fty {
//             fty::from_bits(
//                 (x.mul_add(
//                     0x00800000 as fty,
//                     0x3f800000 as fty - 0x00800000 as fty * 0.04,
//                 )) as u32,
//             )
//         }
//     )
// }

fn gen_power_scale(
    config: &Config,
    name: TokenStream,
    scale: TokenStream,
    offset: TokenStream,
    correction: TokenStream,
) -> TokenStream {
    let one = config.get_one();

    // A very approximate x.recip() used for estimates +/- 0.1
    quote!(
        pub fn #name(x: fty) -> fty {
            let y  : fty = fty::from_bits(
                (((x.abs().to_bits() as fty).mul_add(#scale, #one as fty * #offset))) as uty
            );
            #correction
        }
    )
}

pub fn gen_recip_approx(_terms: usize, config: &Config) -> TokenStream {
    let scale = quote!(-1.0);
    let offset = quote!(2.0);
    let correction = quote!((y - 0.08).copysign(x));
    gen_power_scale(config, quote!(recip_approx), scale, offset, correction)
}

pub fn gen_sqrt_approx(_terms: usize, config: &Config) -> TokenStream {
    let scale = quote!(0.5);
    let offset = quote!(0.5);
    let correction = quote!(y - 0.08);
    gen_power_scale(config, quote!(sqrt_approx), scale, offset, correction)
}

pub fn gen_cbrt_approx(_terms: usize, config: &Config) -> TokenStream {
    let scale = quote!(1.0 / 3.0);
    let offset = quote!(2.0 / 3.0);
    let correction = quote!((y - 0.08).copysign(x));
    gen_power_scale(config, quote!(cbrt_approx), scale, offset, correction)
}

pub fn gen_select(_terms: usize, _config: &Config) -> TokenStream {
    quote!(
        fn select(a: bool, b: fty, c: fty) -> fty {
            if a {b} else {c}
        }
    )
}

pub fn gen_nextafter(_terms: usize, _config: &Config) -> TokenStream {
    quote!(
        pub fn nextafter(arg: fty) -> fty {
            fty::from_bits(arg.to_bits()+1)
        }
    )
}

// pub fn gen_log2_approx(config: &Config) -> TokenStream {
//
//     // A very approximate x.log2() used for estimates.
//     quote!(
//         pub fn log2_approx(x: fty) -> fty {
//             let exponent = (x.to_bits() >> 23) as i32 - 0x7f;
//             let x = fty::from_bits((x.to_bits() & 0x7fffff) | 0x3f800000) - 0.96;
//             let y: fty = x;
//             y + (exponent as fty)
//         }
//     )
// }
pub fn gen_test_function(_terms: usize, _config: &Config) -> TokenStream {
    quote! {
        fn test_function<F : Fn(fty) -> fty>(test_name: &str, accurate_values: &[(fty, fty)], limit: fty, f: F) {
            let mut max_ref_error : fty = 0.0;
            let mut bad_ycalc : fty = 0.0;
            let mut bad_yref : fty = 0.0;
            let mut bad_x : fty = 0.0;
            for &(x , yref) in accurate_values {
                let ycalc = f(x);
                let eref = (ycalc - yref). abs ();
                if eref > max_ref_error {
                    max_ref_error = eref;
                    bad_ycalc = ycalc;
                    bad_yref = yref;
                    bad_x = x;
                }
            }
            println!("{}:", test_name);
            println!("max_ref_error={:25.20}", max_ref_error);
            println!("x    ={:016x} {:25.20}", bad_x.to_bits(), bad_x);
            println!("ycalc={:016x} {:25.20}", bad_ycalc.to_bits(), bad_ycalc);
            println!("yref ={:016x} {:25.20}", bad_yref.to_bits(), bad_yref);
            assert ! (! max_ref_error . is_nan ());
            assert ! (max_ref_error <= limit);
        }
    }
}