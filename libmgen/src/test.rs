use crate::functions::{TestSpec, TestType};
use crate::Config;
use doctor_syn::*;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_quote, Expr};

/// Generate a set of accurate points within a range and
/// compare the result with these values. The result should
/// be accurate to a certain value scaled so that 1.0 is the LSB
/// of 0.5..1
fn gen_max_abs(
    t: &TestSpec,
    config: &Config,
    min: &str,
    max: &str,
    bits32: f64,
    bits64: f64,
    n: usize,
) -> TokenStream {
    let num_digits = config.num_digits();
    use std::str::FromStr;
    let refexpr = TokenStream::from_str(&t.ref_expr).unwrap();
    let refe: Expression = syn::parse2::<syn::Expr>(refexpr.clone()).unwrap().into();
    let variable = name!(x);
    let mut accurate_values = TokenStream::new();
    let tmin = TokenStream::from_str(min).unwrap();
    let tmax = TokenStream::from_str(max).unwrap();
    for i in 0..n {
        let xee: Expr = parse_quote!(((#i * ((#tmax) - (#tmin)) / (#n) + (#tmin))));
        let xeee: Expression = xee.clone().into();
        let x: Expr = xeee.eval(num_digits).unwrap().into();
        let mut vars = VariableList::new();
        vars.add_var(variable.clone(), xee.into());
        let subst = refe.subst(vars).unwrap();
        if let Ok(ye) = subst.eval(num_digits) {
            let ye: Expr = ye.into();
            let row = quote!((#x as fty, #ye as fty),);
            accurate_values.extend(row.into_iter());
        } else {
            panic!("subst failure building test {}", t.test_name);
        }
    }

    let test_name = format_ident!("{}", t.test_name);
    let test_name_str = t.test_name;
    let expr = TokenStream::from_str(&t.rust_expr).unwrap();
    let accuracy = if config.num_bits() == 32 {
        bits32 * 2.0_f64.powi(-23)
    } else {
        bits64 * 2.0_f64.powi(-53)
    };

    let plot_function = if config.generate_plots() {
        quote!{
            plot_function(#test_name_str, accurate_values, |x| #expr);
        }
    } else {
        quote! {}
    };

    quote!(
        #[test]
        pub fn #test_name() {
            let accurate_values : &[(fty, fty)] = &[#accurate_values];
            test_function(#test_name_str, accurate_values, #accuracy as fty, |x| #expr);
            #plot_function
        }
    )
}

/// Generate a histogram for a random function
/// and check it against the PDF.
fn gen_histogram(
    t: &TestSpec,
    config: &Config,
    min: &str,
    max: &str,
) -> TokenStream {
    let nbuckets = 32_usize;
    let niter = 1000000_usize;
    let num_digits = config.num_digits();
    use std::str::FromStr;
    let refexpr = TokenStream::from_str(&t.ref_expr).unwrap();
    let refe: Expression = syn::parse2::<syn::Expr>(refexpr.clone()).unwrap().into();
    let tmin = TokenStream::from_str(min).unwrap();
    let tmax = TokenStream::from_str(max).unwrap();

    let test_name = format_ident!("{}", t.test_name);
    let test_name_str = t.test_name;
    let expr = TokenStream::from_str(&t.rust_expr).unwrap();
    quote!(
        #[test]
        pub fn #test_name() {
            let mut h = [0; #nbuckets];
            for i in 0..#niter {
                let y = #expr as f64;
                let idx = ((y - #tmin) / (#tmax - #tmin) * #nbuckets as f64).floor() as isize;
                if idx >= 0 && idx < (#nbuckets) as isize {
                    h[idx as usize] += 1;
                }
            }
            let dx = (#tmax - #tmin) as f64 / (#nbuckets) as f64;
            let mut max_err : f64 = 0.0;
            for i in 0..#nbuckets {
                let x = ((i as f64 + 0.5) / #nbuckets as f64) * (#tmax - #tmin) + #tmin;
                let pdf_est = h[i] as f64 / ((#niter) as f64 * dx);
                let pdf_ref = (#refexpr) as f64;
                println!("{} {} {}", x, pdf_est, pdf_ref);
                max_err = (pdf_est - pdf_ref).abs();
            }
            println!("max err = {}", max_err);
            assert!(max_err < 0.001);
        }
    )
}

pub fn gen_test(t: &TestSpec, config: &Config) -> TokenStream {
    match t.test {
        TestType::MaxAbs(min, max, bits32, bits64, n) => {
            gen_max_abs(t, config, min, max, bits32, bits64, n)
        }
        TestType::Histogram(min, max) => {
            gen_histogram(t, config, min, max)
        }
    }
}
