use proc_macro2::TokenStream;
use quote::{quote};

pub fn gen_test(
    test_name: TokenStream,
    refexpr: TokenStream,
    expr: TokenStream,
    accuracy: f64,
    tmin: f64,
    tmax: f64,
) -> proc_macro2::TokenStream {
    quote!(
        #[test]
        fn #test_name() {
            const N: i32 = 0x100000;
            let tmin = #tmin;
            let tmax = #tmax;
            let mut max_error = 0.0_f64;
            for i in 0..=N {
                let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
                let y1 = #refexpr;
                let y2 = #expr;
                max_error = max_error.max((y1 - y2).abs());
                if i % (N/16) == 0 { println!("y1={:20.16} y2={:20.16} e={:20.16}", y1, y2, y2-y1); }
            }
            println!("me={:20}", max_error);
            assert!(!max_error.is_nan());
            assert!(max_error < #accuracy);
        }
    )
}
