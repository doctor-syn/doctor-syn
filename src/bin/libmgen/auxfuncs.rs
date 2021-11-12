use crate::Config;
use proc_macro2::TokenStream;
use quote::quote;

pub fn gen_negate_on_odd(config: &Config) -> (TokenStream, TokenStream) {
    if config.enabled("negate_on_odd") {
        return (TokenStream::new(), TokenStream::new());
    }

    let shift = (config.num_bits() - 1) as i32;
    let fty = config.get_fty();
    let uty = config.get_uty();
    let ity = config.get_ity();
    let func = 
        quote!(
            // If x is odd, negate y.
            fn negate_on_odd(x: #fty, y: #fty) -> #fty {
                let sign_bit : #uty = (((x as #ity) & 1) << #shift) as #uty;
                #fty::from_bits(sign_bit ^ y.to_bits())
            }
        );
    let test = if config.generate_tests() {
        quote!(
            #[test]
            fn test_negate_on_odd() {
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
    } else {
        TokenStream::new()
    };

    (func, test)
}

// pub fn gen_exp2_approx(config: &Config) -> TokenStream {
//     let fty = config.get_fty();
//     // A very approximate 2.pow(x) used for estimates +/- 0.05
//     quote!(
//         fn exp2_approx(x: #fty) -> #fty {
//             #fty::from_bits(
//                 (x.mul_add(
//                     0x00800000 as #fty,
//                     0x3f800000 as #fty - 0x00800000 as #fty * 0.04,
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
    let fty = config.get_fty();
    let uty = config.get_uty();
    let one = config.get_one();

    // A very approximate x.recip() used for estimates +/- 0.1
    quote!(
        fn #name(x: #fty) -> #fty {
            let y  : #fty = #fty::from_bits(
                (((x.abs().to_bits() as #fty).mul_add(#scale, #one as #fty * #offset))) as #uty
            );
            #correction
        }
    )
}

pub fn gen_recip_approx(config: &Config) -> TokenStream {
    if config.enabled("recip_approx") {
        return TokenStream::new();
    }

    let scale = quote!(-1.0);
    let offset = quote!(2.0);
    let correction = quote!((y - 0.08).copysign(x));
    gen_power_scale(config, quote!(recip_approx), scale, offset, correction)
}

pub fn gen_sqrt_approx(config: &Config) -> TokenStream {
    if config.enabled("sqrt_approx") {
        return TokenStream::new();
    }

    let scale = quote!(0.5);
    let offset = quote!(0.5);
    let correction = quote!(y - 0.08);
    gen_power_scale(config, quote!(sqrt_approx), scale, offset, correction)
}

pub fn gen_cbrt_approx(config: &Config) -> TokenStream {
    if config.enabled("cbrt_approx") {
        return TokenStream::new();
    }

    let scale = quote!(1.0 / 3.0);
    let offset = quote!(2.0 / 3.0);
    let correction = quote!((y - 0.08).copysign(x));
    gen_power_scale(config, quote!(cbrt_approx), scale, offset, correction)
}

// pub fn gen_log2_approx(config: &Config) -> TokenStream {
//     let fty = config.get_fty();
//     // A very approximate x.log2() used for estimates.
//     quote!(
//         fn log2_approx(x: #fty) -> #fty {
//             let exponent = (x.to_bits() >> 23) as i32 - 0x7f;
//             let x = #fty::from_bits((x.to_bits() & 0x7fffff) | 0x3f800000) - 0.96;
//             let y: #fty = x;
//             y + (exponent as #fty)
//         }
//     )
// }

pub fn gen_aux(config: &Config) -> (TokenStream, TokenStream) {
    // let fty = config.get_fty();
    // let suffix = helpers::get_suffix(num_bits);

    let (negate_on_odd, negate_on_odd_test) = gen_negate_on_odd(config);
    // let _exp2_approx = gen_exp2_approx(num_bits);
    let recip_approx = gen_recip_approx(config);
    let sqrt_approx = gen_sqrt_approx(config);
    let cbrt_approx = gen_cbrt_approx(config);
    // let _log2_approx = gen_log2_approx(num_bits);

    (
        quote!(
            #negate_on_odd
            #recip_approx
            #sqrt_approx
            #cbrt_approx
        ),
        quote!(
            #negate_on_odd_test
        ),
    )
}
