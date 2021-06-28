use crate::test::*;
use proc_macro2::TokenStream;
use quote::{quote};
use crate::helpers;

// https://en.wikipedia.org/wiki/Hyperbolic_functions
pub fn gen_sinh(_num_terms: usize, num_bits: usize) -> TokenStream {
    let fty = helpers::get_fty(num_bits);
    quote!(
        fn sinh(x: #fty) -> #fty {
            let a = x.mul_add(std::#fty::consts::LOG2_E, -1.0);
            let b = x.mul_add(-std::#fty::consts::LOG2_E, -1.0);
            exp2(a) - exp2(b)
        }
    )
}

// https://en.wikipedia.org/wiki/Hyperbolic_functions
pub fn gen_cosh(_num_terms: usize, num_bits: usize) -> TokenStream {
    let fty = helpers::get_fty(num_bits);
    quote!(
        fn cosh(x: #fty) -> #fty {
            let a = x.mul_add(std::#fty::consts::LOG2_E, -1.0);
            let b = x.mul_add(-std::#fty::consts::LOG2_E, -1.0);
            exp2(a) + exp2(b)
        }
    )
}

// https://en.wikipedia.org/wiki/Hyperbolic_functions
pub fn gen_tanh(_num_terms: usize, num_bits: usize) -> TokenStream {
    let fty = helpers::get_fty(num_bits);
    quote!(
        fn tanh(x: #fty) -> #fty {
            let exp2x = exp2(x * (std::#fty::consts::LOG2_E * 2.0));
            (exp2x - 1.0) / (exp2x + 1.0)
        }
    )
}

// https://en.wikipedia.org/wiki/Inverse_hyperbolic_functions
pub fn gen_asinh(_num_terms: usize, num_bits: usize) -> TokenStream {
    let fty = helpers::get_fty(num_bits);
    quote!(
        fn asinh(x: #fty) -> #fty {
            ln(x + (x * x + 1.0).sqrt())
        }
    )
}

// https://en.wikipedia.org/wiki/Inverse_hyperbolic_functions
pub fn gen_acosh(_num_terms: usize, num_bits: usize) -> TokenStream {
    let fty = helpers::get_fty(num_bits);
    quote!(
        fn acosh(x: #fty) -> #fty {
            ln(x + (x * x - 1.0).sqrt())
        }
    )
}

// https://en.wikipedia.org/wiki/Inverse_hyperbolic_functions
pub fn gen_atanh(_num_terms: usize, num_bits: usize) -> TokenStream {
    let fty = helpers::get_fty(num_bits);
    quote!(
        fn atanh(x: #fty) -> #fty {
            (ln(1.0 + x) - ln(1.0 - x)) * 0.5
        }
    )
}

pub fn gen_hyperbolic(num_bits: usize) -> (TokenStream, TokenStream) {
    let sinh = gen_sinh(7, num_bits);
    let cosh = gen_cosh(7, num_bits);
    let tanh = gen_tanh(7, num_bits);

    let asinh = gen_asinh(7, num_bits);
    let acosh = gen_acosh(7, num_bits);
    let atanh = gen_atanh(7, num_bits);

    let bit = (2.0_f64).powi(if num_bits == 32 { 23 } else { 52 });
    let fty = helpers::get_fty(num_bits);

    let test_cosh = gen_test(
        quote!(test_cosh),
        quote!(x.cosh()),
        quote!(cosh(x as #fty) as f64),
        bit * 2.0,
        -1.0,
        1.0,
    );
    let test_sinh = gen_test(
        quote!(test_sinh),
        quote!(x.sinh()),
        quote!(sinh(x as #fty) as f64),
        bit * 2.0,
        -1.0,
        1.0,
    );
    let test_tanh = gen_test(
        quote!(test_tanh),
        quote!(x.tanh()),
        quote!(tanh(x as #fty) as f64),
        bit * 2.0,
        -1.0,
        1.0,
    );
    let test_acosh = gen_test(
        quote!(test_acosh),
        quote!(x.acosh()),
        quote!(acosh(x as #fty) as f64),
        bit * 2.0,
        -1.0,
        1.0,
    );
    let test_asinh = gen_test(
        quote!(test_asinh),
        quote!(x.asinh()),
        quote!(asinh(x as #fty) as f64),
        bit * 3.0,
        -1.0,
        1.0,
    );
    let test_atanh = gen_test(
        quote!(test_atanh),
        quote!(x.atanh()),
        quote!(atanh(x as #fty) as f64),
        bit * 3.0,
        -0.9,
        0.9,
    );
    (
        quote!(
            #sinh
            #cosh
            #tanh

            #asinh
            #acosh
            #atanh
        ),
        quote!(
            #test_cosh
            #test_sinh
            #test_tanh
            #test_acosh
            #test_asinh
            #test_atanh
        ),
    )
}
