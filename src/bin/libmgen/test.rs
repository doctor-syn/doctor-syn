use proc_macro2::TokenStream;
use quote::quote;

pub fn gen_test(
    test_name: TokenStream,
    refexpr: TokenStream,
    expr: TokenStream,
    accuracy: f64,
    tmin: f64,
    tmax: f64,
) -> TokenStream {
    quote!(
        #[test]
        fn #test_name() {
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
