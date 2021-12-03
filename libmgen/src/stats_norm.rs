use doctor_syn::Parity;
use doctor_syn::{expr, name};
use proc_macro2::TokenStream;
use quote::quote;

// use crate::test::*;
use crate::Config;

pub fn gen_dnorm(num_terms: usize, config: &Config) -> TokenStream {
    quote!(
        pub fn dnorm(arg: fty, mean: fty, sigma: fty) -> fty {
            let rsigma: fty = recip(sigma);
            let y: fty = (arg - mean) * rsigma;
            let e: fty = (0.5 * LOG2_E) * y * y;
            rsigma * SQRT_RECIP_2PI * exp2(e)
        }
    )
}

pub fn gen_pnorm(num_terms: usize, config: &Config) -> TokenStream {
    quote!(
        // pub fn pnorm(arg: fty, mean: fty, sigma: fty) -> fty {
        //     0.0
        // }
    )
}

pub fn gen_rnorm(num_terms: usize, config: &Config) -> TokenStream {
    quote!(
        pub fn rnorm(index: usize, mean: fty, sigma: fty) -> fty {
            const MIN: fty = 0.000001;
            const MAX: fty = 1.0 - MIN;
            let x: fty = runif(index, MIN, MAX);
            qnorm(x, mean, sigma)
        }
    )
}

pub fn gen_qnorm(num_terms: usize, config: &Config) -> TokenStream {
    // Note this function is very similar to tan and logit.
    // let xmin = -0.499999;
    // let xmax = 0.499999;
    let xmin = -0.49;
    let xmax = 0.49;

    let approx = expr!((x + 0.5).qnorm(0, 1) * (x * x - 0.5 * 0.5))
        .approx(
            num_terms,
            xmin,
            xmax,
            name!(x),
            Parity::Odd,
            config.num_digits(),
        )
        .unwrap()
        .use_number_type(config.number_type(), config.num_bits())
        .unwrap()
        .into_inner();

    quote!(
        pub fn qnorm(arg: fty, mean: fty, sigma: fty) -> fty {
            // Range reduction
            let scaled : fty = arg - 0.5;
            let x = scaled;

            // Pole elimination
            let recip : fty = sigma / (x * x - 0.5 * 0.5);

            // Polynomial approximation
            let y : fty = #approx ;

            // Reassembly.
            y * recip + mean
        }
    )
}
