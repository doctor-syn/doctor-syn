use crate::Config;
use doctor_syn::*;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_quote, Expr};
use crate::functions::{TestSpec, TestType};

fn gen_max_abs(t: &TestSpec, config: &Config, min: &str, max: &str, bits32: f64, bits64: f64, n: usize) -> TokenStream {
    let num_digits = config.num_digits();
    use std::str::FromStr;
    let refexpr = TokenStream::from_str(&t.ref_expr).unwrap();
    let refe: Expression = syn::parse2::<syn::Expr>(refexpr.clone()).unwrap().into();
    let variable = name!(x);
    let mut accurate_values = TokenStream::new();
    let tmin = TokenStream::from_str(min).unwrap();
    let tmax = TokenStream::from_str(max).unwrap();
    for i in 0..n {
        let xee: Expr = parse_quote!((#i * ((#tmax) - (#tmin)) / (#n) + (#tmin)));
        let xeee: Expression = xee.clone().into();
        let x: Expr = xeee.eval(num_digits).unwrap().into();
        let mut vars = VariableList::new();
        vars.add_var(variable.clone(), xee.into());
        let subst = refe.subst(vars).unwrap();
        if let Ok(ye) = subst.eval(num_digits) {
            let ye: Expr = ye.into();
            let row = quote!((#x as fty,#ye as fty),);
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
    quote!(
        #[test]
        pub fn #test_name() {
            let accurate_values : &[(fty, fty)] = &[#accurate_values];
            test_function(#test_name_str, accurate_values, #accuracy as fty, |x| #expr);
        }
    )

}

pub fn gen_test(t: &TestSpec, config: &Config) -> TokenStream {
    match t.test {
        TestType::MaxAbs(min, max, bits32, bits64, n) => {
            gen_max_abs(t, config, min, max, bits32, bits64, n)
        }
    }
}
