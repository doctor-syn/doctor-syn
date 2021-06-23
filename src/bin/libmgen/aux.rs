use quote::{format_ident, quote};
use proc_macro2::TokenStream;

pub fn gen_negate_on_odd(num_bits: usize) -> TokenStream {
    let shift = num_bits - 1;
    let fty = format_ident!("f{}", num_bits);
    let uty = format_ident!("u{}", num_bits);
    quote!(
        // If x is odd, negate y.
        fn negate_on_odd(x: #fty, y: #fty) -> #fty {
            let sign_bit = ((x as #uty) & 1) << #shift;
            #fty::from_bits(sign_bit ^ y.to_bits())
        }
    )
}

pub fn gen_exp2_approx(num_bits: usize) -> TokenStream {
    let fty = format_ident!("f{}", num_bits);
    // A very approximate 2.pow(x) used for estimates +/- 0.05
    quote!(
        fn exp2_approx(x: #fty) -> #fty {
            #fty::from_bits(
                (x.mul_add(
                    0x00800000 as #fty,
                    0x3f800000 as #fty - 0x00800000 as #fty * 0.04,
                )) as u32,
            )
        }
    )
}

pub fn gen_recip_approx(num_bits: usize) -> TokenStream {
    let fty = format_ident!("f{}", num_bits);
    // A very approximate x.recip() used for estimates +/- 0.1
    quote!(
        fn recip_approx(x: #fty) -> #fty {
            let y = #fty::from_bits((0x3f800000 as #fty * 2.0 - (x.abs().to_bits() as #fty)) as u32)
                - 0.08;
            if x < 0.0 {
                -y
            } else {
                y
            }
        }
    )
}

pub fn gen_log2_approx(num_bits: usize) -> TokenStream {
    let fty = format_ident!("f{}", num_bits);
    // A very approximate x.log2() used for estimates.
    quote!(
        fn log2_approx(x: #fty) -> #fty {
            let exponent = (x.to_bits() >> 23) as i32 - 0x7f;
            let x = #fty::from_bits((x.to_bits() & 0x7fffff) | 0x3f800000) - 0.96;
            let y: #fty = x;
            y + (exponent as #fty)
        }
    )
}

pub fn gen_aux(num_bits: usize) -> (TokenStream, TokenStream) {
    // let fty = format_ident!("f{}", num_bits);
    // let suffix = format!("f{}", num_bits);

    let _negate_on_odd = gen_negate_on_odd(num_bits);
    let _exp2_approx = gen_exp2_approx(num_bits);
    let recip_approx = gen_recip_approx(num_bits);
    let _log2_approx = gen_log2_approx(num_bits);

    (
        quote!(
            // #negate_on_odd
            // #exp2_approx
            #recip_approx
            // #log2_approx
        ), quote!(
        )
    )
}
