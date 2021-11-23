use doctor_syn::Parity;
use doctor_syn::{expr, name, num_digits_for};
use proc_macro2::TokenStream;
use quote::quote;

// use crate::test::*;
use crate::Config;

pub fn gen_qnorm(num_terms: usize, config: &Config) -> TokenStream {
    if !config.enabled("qnorm") {
        return TokenStream::new();
    }

    // Note this function is very similar to tan and logit.
    // let xmin = -0.499999;
    // let xmax = 0.499999;
    let xmin = -0.49;
    let xmax = 0.49;

    let approx = expr!((x + 0.5).qnorm(0, 1) * (x * x - 0.25))
        .approx(
            num_terms,
            xmin,
            xmax,
            name!(x),
            Parity::Odd,
            num_digits_for(config.num_bits()),
        )
        .unwrap()
        .use_number_type(config.number_type())
        .unwrap()
        .into_inner();

    quote!(
        fn qnorm(arg: fty) -> fty {
            let scaled : fty = arg - 0.5;
            let recip : fty = 1.0 / (x*x - 0.25);
            let y : fty = #approx ;
            y * recip
        }
    )
}

pub fn gen_stats_norm(config: &Config) -> (TokenStream, TokenStream) {
    let tan_num_terms = config.get_tan_terms();
    let qnorm = gen_qnorm(tan_num_terms, config);

    let test_qnorm = TokenStream::new();

    (
        quote!(
            #qnorm
        ),
        quote!(
            #test_qnorm
        ),
    )
}
