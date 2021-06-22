use quote::{format_ident, quote};

pub fn gen_negate_on_odd(num_bits: usize) -> proc_macro2::TokenStream {
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

pub fn gen_exp2_approx(_num_terms: usize) -> proc_macro2::TokenStream {
    // A very approximate 2.pow(x) used for estimates +/- 0.05
    quote!(
        fn exp2_approx(x: f32) -> f32 {
            f32::from_bits(
                (x.mul_add(
                    0x00800000 as f32,
                    0x3f800000 as f32 - 0x00800000 as f32 * 0.04,
                )) as u32,
            )
        }
    )
}

pub fn gen_recip_approx(_num_terms: usize) -> proc_macro2::TokenStream {
    // A very approximate x.recip() used for estimates +/- 0.1
    quote!(
        fn recip_approx(x: f32) -> f32 {
            let y = f32::from_bits((0x3f800000 as f32 * 2.0 - (x.abs().to_bits() as f32)) as u32)
                - 0.08;
            if x < 0.0 {
                -y
            } else {
                y
            }
        }
    )
}

pub fn gen_log2_approx(_num_terms: usize) -> proc_macro2::TokenStream {
    // A very approximate x.log2() used for estimates.
    quote!(
        fn log2_approx(x: f32) -> f32 {
            let exponent = (x.to_bits() >> 23) as i32 - 0x7f;
            let x = f32::from_bits((x.to_bits() & 0x7fffff) | 0x3f800000) - 0.96;
            let y: f32 = x;
            y + (exponent as f32)
        }
    )
}
