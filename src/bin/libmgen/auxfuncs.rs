use crate::helpers;
use proc_macro2::TokenStream;
use quote::quote;

pub fn gen_negate_on_odd(num_bits: usize, _number_type: &str) -> (TokenStream, TokenStream) {
    let shift = (num_bits - 1) as i32;
    let fty = helpers::get_fty(num_bits);
    let uty = helpers::get_uty(num_bits);
    let ity = helpers::get_ity(num_bits);
    (
        quote!(
            // If x is odd, negate y.
            fn negate_on_odd(x: #fty, y: #fty) -> #fty {
                let sign_bit = (((x as #ity) & 1) << #shift) as #uty;
                #fty::from_bits(sign_bit ^ y.to_bits())
            }
        ),
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
        ),
    )
}

// pub fn gen_exp2_approx(num_bits: usize, _number_type: &str) -> TokenStream {
//     let fty = helpers::get_fty(num_bits);
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
    num_bits: usize,
    name: TokenStream,
    scale: TokenStream,
    offset: TokenStream,
    correction: TokenStream,
) -> TokenStream {
    let fty = helpers::get_fty(num_bits);
    let uty = helpers::get_uty(num_bits);
    let one = helpers::get_one(num_bits);

    // A very approximate x.recip() used for estimates +/- 0.1
    quote!(
        fn #name(x: #fty) -> #fty {
            let y = #fty::from_bits(
                (((x.abs().to_bits() as #fty).mul_add(#scale, #one as #fty * #offset))) as #uty
            );
            #correction
        }
    )
}

pub fn gen_recip_approx(num_bits: usize, _number_type: &str) -> TokenStream {
    let scale = quote!(-1.0);
    let offset = quote!(2.0);
    let correction = quote!((y - 0.08).copysign(x));
    gen_power_scale(num_bits, quote!(recip_approx), scale, offset, correction)
}

pub fn gen_sqrt_approx(num_bits: usize, _number_type: &str) -> TokenStream {
    let scale = quote!(0.5);
    let offset = quote!(0.5);
    let correction = quote!(y - 0.08);
    gen_power_scale(num_bits, quote!(sqrt_approx), scale, offset, correction)
}

pub fn gen_cbrt_approx(num_bits: usize, _number_type: &str) -> TokenStream {
    let scale = quote!(1.0 / 3.0);
    let offset = quote!(2.0 / 3.0);
    let correction = quote!((y - 0.08).copysign(x));
    gen_power_scale(num_bits, quote!(cbrt_approx), scale, offset, correction)
}

// pub fn gen_log2_approx(num_bits: usize, _number_type: &str) -> TokenStream {
//     let fty = helpers::get_fty(num_bits);
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

pub fn gen_aux(num_bits: usize, number_type: &str) -> (TokenStream, TokenStream) {
    // let fty = helpers::get_fty(num_bits);
    // let suffix = helpers::get_suffix(num_bits);

    let (negate_on_odd, negate_on_odd_test) = gen_negate_on_odd(num_bits, number_type);
    // let _exp2_approx = gen_exp2_approx(num_bits);
    let recip_approx = gen_recip_approx(num_bits, number_type);
    let sqrt_approx = gen_sqrt_approx(num_bits, number_type);
    let cbrt_approx = gen_cbrt_approx(num_bits, number_type);
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
