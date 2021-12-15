use doctor_syn::Parity;
use doctor_syn::{expr, name};
use proc_macro2::TokenStream;
use quote::quote;

// use crate::test::*;
use crate::Config;

pub fn gen_dnorm(num_terms: usize, config: &Config) -> TokenStream {
    quote!(
        pub fn dnorm(arg: fty, mean: fty, sigma: fty) -> fty {
            let rsigma: fty = (1.0 as fty) / sigma;
            let y: fty = (arg - mean) * rsigma;
            let e: fty = (-0.5 * LOG2_E) as fty * y * y;
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
    let xmin = -0.4999;
    let xmax = 0.4999;

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

pub fn gen_qnorm_2(num_terms: usize, config: &Config) -> TokenStream {
    // Note this function is very similar to tan and logit.
    // df <- data.frame(x=2**x, y=qnorm(2**x)/(1-sqrt(-log(2**x,2))))

    let approx_edge = expr!(xe.exp2().qnorm(0, 1))
        .approx(
            6,
            -4.0,
            -2.0,
            name!(xe),
            Parity::Neither,
            config.num_digits(),
        )
        .unwrap()
        .use_number_type(config.number_type(), config.num_bits())
        .unwrap()
        .into_inner();

    let approx_centre = expr!((xc + 0.5).qnorm(0, 1))
        .approx(
            num_terms,
            -0.25,
            0.25,
            name!(xc),
            Parity::Odd,
            config.num_digits(),
        )
        .unwrap()
        .use_number_type(config.number_type(), config.num_bits())
        .unwrap()
        .into_inner();

    quote!(
        pub fn qnorm(arg: fty, mean: fty, sigma: fty) -> fty {
            // Edge portion - qnorm(exp2(log2(x)))
            let xe : fty = log2(arg.min(1.0-arg));
            let ye : fty = #approx_edge;

            // Centre portion.
            let xc : fty = arg - 0.5;
            let yc : fty = #approx_centre;

            let y = select(xc <= 0.25, yc, ye.copysign(xc));
            y * sigma + mean
        }
    )
}

pub fn gen_qnorm_new(num_terms: usize, config: &Config) -> TokenStream {
    // df <- data.frame(x=2**x, y=qnorm(2**x)/(1-sqrt(-x)))

    // TODO: gradient descent control points.
    let approx = expr!(x.exp2().qnorm(0, 1)/(1-(-x).sqrt()))
        .approx(
            24,
            -30.0,
            -1.0001,
            name!(x),
            Parity::Neither,
            config.num_digits(),
        )
        .unwrap()
        .use_number_type(config.number_type(), config.num_bits())
        .unwrap()
        .into_inner();

    quote!(
        pub fn qnorm(arg: fty, mean: fty, sigma: fty) -> fty {
            let xmin : fty = arg.min(1.0 - arg);
            let x = log2(xmin);
            let mul : fty = 1.0 - (-x).sqrt();
            let y : fty = #approx;
            // println!("arg={} xmin={} x={} mul={} y={}", arg, xmin, x, mul, y);
            y.copysign(0.5 - arg) * (mul * sigma) + mean
        }
    )
}
