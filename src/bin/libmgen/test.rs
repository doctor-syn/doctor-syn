use doctor_syn::bdmath::*;
use doctor_syn::bigdecimal::ToPrimitive;
use doctor_syn::*;
use proc_macro2::TokenStream;
use quote::quote;
use crate::Config;

pub fn gen_test(
    config: &Config,
    test_name: TokenStream,
    refexpr: TokenStream,
    expr: TokenStream,
    accuracy: f64,
    tmin: f64,
    tmax: f64,
) -> TokenStream {
    if !config.generate_tests() {
        return TokenStream::new();
    }
    const N: i32 = 128;
    let num_digits = 20;
    let refe: Expression = syn::parse2::<syn::Expr>(refexpr.clone()).unwrap().into();
    let variable = name!(x);
    let mut accurate_values = TokenStream::new();
    for i in 1..N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let xe: Expression = x.try_into().unwrap();
        let mut vars = VariableList::new();
        vars.add_var(variable.clone(), xe.into());
        let subst = refe.subst(vars).unwrap();
        if let Ok(ye) = subst.eval(num_digits) {
            let y: BigDecimal = ye.try_into().unwrap();
            let y = y.to_f64().unwrap();
            let row = quote!((#x,#y),);
            accurate_values.extend(row.into_iter());
        }
    }

    quote!(
        #[test]
        fn #test_name() {
            let accurate_values : &[(f64, f64)] = &[#accurate_values];

            let mut max_ref_error = 0.0_f64;
            let mut max_lib_error = 0.0_f64;
            for (x, y) in accurate_values {
                let x = *x;
                let y = *y;
                let y1 = #refexpr;
                let y2 = #expr;
                let eref = (y1 - y).abs();
                let elib = (y2 - y).abs();
                max_ref_error = max_ref_error.max(eref);
                max_lib_error = max_lib_error.max(elib);
                println!(" x={:25.20} y1={:25.20} y2={:25.20} eref={:25.20} elib={:25.20}", x, y1, y2, eref, elib);
            }
            println!("# eref={:25.20} elib={:25.20}", max_ref_error, max_lib_error);
            assert!(!max_lib_error.is_nan());
            assert!(max_lib_error <= #accuracy);

            const N: i32 = 0x100000;
            let tmin = #tmin;
            let tmax = #tmax;
            let mut max_error = 0.0_f64;
            let mut xmax = tmin;
            let mut y1max = 0.0;
            let mut y2max = 0.0;
            for i in 0..=N {
                let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
                let y1 = #refexpr;
                let y2 = #expr;
                let error = (y1 - y2).abs();
                if error > max_error {
                    max_error = error;
                    xmax = x;
                    y1max = y1;
                    y2max = y2;
                }
                if i % (N/16) == 0 { println!(" x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}", x, y1, y2, y2-y1); }
                // if i % (N/16) == 0 { println!("x={:x} y1={:x} y2={:x} e={:x}", x.to_bits(), y1.to_bits(), y2.to_bits(), ((y2.to_bits() as i64).wrapping_sub(y1.to_bits() as i64)).abs()); }
            }
            println!("!x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}", xmax, y1max, y2max, max_error);
            assert!(!max_error.is_nan());
            assert!(max_error < #accuracy);
        }
    )
}
