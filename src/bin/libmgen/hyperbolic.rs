use quote::{quote};

// https://en.wikipedia.org/wiki/Hyperbolic_functions
pub fn gen_sinh(_num_terms: usize) -> proc_macro2::TokenStream {
    quote!(
        fn sinh(x: f32) -> f32 {
            let a = x.mul_add(std::f32::consts::LOG2_E, -1.0);
            let b = x.mul_add(-std::f32::consts::LOG2_E, -1.0);
            exp2(a) - exp2(b)
        }
    )
}

// https://en.wikipedia.org/wiki/Hyperbolic_functions
pub fn gen_cosh(_num_terms: usize) -> proc_macro2::TokenStream {
    quote!(
        fn cosh(x: f32) -> f32 {
            let a = x.mul_add(std::f32::consts::LOG2_E, -1.0);
            let b = x.mul_add(-std::f32::consts::LOG2_E, -1.0);
            exp2(a) + exp2(b)
        }
    )
}

// https://en.wikipedia.org/wiki/Hyperbolic_functions
pub fn gen_tanh(_num_terms: usize) -> proc_macro2::TokenStream {
    quote!(
        fn tanh(x: f32) -> f32 {
            let exp2x = exp2(x*(std::f32::consts::LOG2_E*2.0));
            (exp2x - 1.0) / (exp2x + 1.0)
        }
    )
}

// https://en.wikipedia.org/wiki/Inverse_hyperbolic_functions
pub fn gen_asinh(_num_terms: usize) -> proc_macro2::TokenStream {
    quote!(
        fn asinh(x: f32) -> f32 {
            ln(x + (x*x+1.0).sqrt())
        }
    )
}

// https://en.wikipedia.org/wiki/Inverse_hyperbolic_functions
pub fn gen_acosh(_num_terms: usize) -> proc_macro2::TokenStream {
    quote!(
        fn acosh(x: f32) -> f32 {
            ln(x + (x*x-1.0).sqrt())
        }
    )
}

// https://en.wikipedia.org/wiki/Inverse_hyperbolic_functions
pub fn gen_atanh(_num_terms: usize) -> proc_macro2::TokenStream {
    quote!(
        fn atanh(x: f32) -> f32 {
            (ln(1.0 + x) - ln(1.0 - x)) * 0.5
        }
    )
}

