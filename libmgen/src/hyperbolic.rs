use crate::test::*;
use crate::Config;
use proc_macro2::TokenStream;
use quote::quote;

// https://en.wikipedia.org/wiki/Hyperbolic_functions
pub fn gen_sinh(_num_terms: usize, config: &Config) -> TokenStream {
    quote!(
        fn sinh(x: fty) -> fty {
            let a: fty = x.mul_add(LOG2_E, -1.0);
            let b: fty = x.mul_add(-LOG2_E, -1.0);
            exp2(a) - exp2(b)
        }
    )
}

// https://en.wikipedia.org/wiki/Hyperbolic_functions
pub fn gen_cosh(_num_terms: usize, config: &Config) -> TokenStream {
    quote!(
        fn cosh(x: fty) -> fty {
            let a: fty = x.mul_add(LOG2_E, -1.0);
            let b: fty = x.mul_add(-LOG2_E, -1.0);
            exp2(a) + exp2(b)
        }
    )
}

// https://en.wikipedia.org/wiki/Hyperbolic_functions
pub fn gen_tanh(_num_terms: usize, config: &Config) -> TokenStream {
    quote!(
        fn tanh(x: fty) -> fty {
            let exp2x: fty = exp2(x * (LOG2_E * 2.0));
            (exp2x - 1.0) / (exp2x + 1.0)
        }
    )
}

// https://en.wikipedia.org/wiki/Inverse_hyperbolic_functions
pub fn gen_asinh(_num_terms: usize, config: &Config) -> TokenStream {
    quote!(
        fn asinh(x: fty) -> fty {
            ln(x + (x * x + 1.0).sqrt())
        }
    )
}

// https://en.wikipedia.org/wiki/Inverse_hyperbolic_functions
pub fn gen_acosh(_num_terms: usize, config: &Config) -> TokenStream {
    quote!(
        fn acosh(x: fty) -> fty {
            ln(x + (x * x - 1.0).sqrt())
        }
    )
}

// https://en.wikipedia.org/wiki/Inverse_hyperbolic_functions
pub fn gen_atanh(_num_terms: usize, config: &Config) -> TokenStream {
    quote!(
        fn atanh(x: fty) -> fty {
            (ln(1.0 + x) - ln(1.0 - x)) * 0.5
        }
    )
}

pub fn gen_hyperbolic(config: &Config) -> (TokenStream, TokenStream) {
    let sinh = gen_sinh(7, config);
    let cosh = gen_cosh(7, config);
    let tanh = gen_tanh(7, config);

    let asinh = gen_asinh(7, config);
    let acosh = gen_acosh(7, config);
    let atanh = gen_atanh(7, config);

    let bit = (2.0_f64).powi(if config.num_bits() == 32 { 23 } else { 52 });

    let test_cosh = gen_test(
        config,
        quote!(test_cosh),
        quote!(x.cosh()),
        quote!(cosh(x as fty) as f64),
        bit * 2.0,
        -1.0,
        1.0,
    );
    let test_sinh = gen_test(
        config,
        quote!(test_sinh),
        quote!(x.sinh()),
        quote!(sinh(x as fty) as f64),
        bit * 2.0,
        -1.0,
        1.0,
    );
    let test_tanh = gen_test(
        config,
        quote!(test_tanh),
        quote!(x.tanh()),
        quote!(tanh(x as fty) as f64),
        bit * 2.0,
        -1.0,
        1.0,
    );
    let test_acosh = gen_test(
        config,
        quote!(test_acosh),
        quote!(x.acosh()),
        quote!(acosh(x as fty) as f64),
        bit * 2.0,
        -1.0,
        1.0,
    );
    let test_asinh = gen_test(
        config,
        quote!(test_asinh),
        quote!(x.asinh()),
        quote!(asinh(x as fty) as f64),
        bit * 3.0,
        -1.0,
        1.0,
    );
    let test_atanh = gen_test(
        config,
        quote!(test_atanh),
        quote!(x.atanh()),
        quote!(atanh(x as fty) as f64),
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
