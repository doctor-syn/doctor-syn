use doctor_syn::Parity;
use doctor_syn::{expr, name};
use quote::{quote};

pub fn gen_atan2(num_terms: usize) -> proc_macro2::TokenStream {
    let xmin = -1.0;
    let xmax = 1.0;

    let approx = expr!( x.atan() )
        .approx(num_terms, xmin, xmax, name!(x), Parity::Odd)
        .unwrap()
        .use_suffix(Some("f32".to_string()))
        .unwrap()
        .into_inner();

    // TODO: calculate the recipocal without a divide.
    quote!(
        fn atan2(y: f32, x: f32) -> f32 {
            use std::f32::consts::PI;
            let offset180 = if y < 0.0 { -PI } else { PI };
            let (x, y, offset) = if x < 0.0 { (-x, -y, offset180) } else { (x, y, 0.0) };
            let offset90 = if y < 0.0 { -PI/2.0 } else { PI/2.0 };
            let (x, y, offset) = if y.abs() > x { (y, -x, offset + offset90) } else { (x, y, offset) };
            let x = y / x;
            let y = #approx ;
            y + offset
        }
    )
}

pub fn gen_asin(num_terms: usize) -> proc_macro2::TokenStream {
    const LIM : f32 = 0.9;
    let approx = expr!( x.asin() )
        .approx(num_terms, -LIM, LIM, name!(x), Parity::Odd)
        .unwrap()
        .use_suffix(Some("f32".to_string()))
        .unwrap()
        .into_inner();

    quote!(
        fn asin(x: f32) -> f32 {
            use std::f32::consts::PI;
            let c = if x < 0.0 { -PI/2.0 } else { PI/2.0 };
            let s = if x < 0.0 { -1.0 } else { 1.0  };
            let x0 = x;
            let x = if x * x < #LIM * #LIM { x } else { (1.0-x*x).sqrt() };
            let y = #approx ;
            if x0*x0 < #LIM*#LIM { y } else { c - y * s }
        }
    )
}

pub fn gen_acos(num_terms: usize) -> proc_macro2::TokenStream {
    const LIM : f32 = 0.9;
    let approx = expr!( x.asin() )
        .approx(num_terms, -LIM, LIM, name!(x), Parity::Odd)
        .unwrap()
        .use_suffix(Some("f32".to_string()))
        .unwrap()
        .into_inner();

    quote!(
        fn acos(x: f32) -> f32 {
            use std::f32::consts::PI;
            let c = if x < 0.0 { PI } else { 0.0 };
            let s = if x < 0.0 { 1.0 } else { -1.0  };
            let x0 = x;
            let x = if x * x < #LIM * #LIM { x } else { (1.0-x*x).sqrt() };
            let y = #approx ;
            if x0*x0 < #LIM*#LIM { PI/2.0 - y } else { c - y * s }
        }
    )
}

pub fn gen_atan(num_terms: usize) -> proc_macro2::TokenStream {
    const LIM : f32 = 1.0;
    let approx = expr!( x.atan() )
        .approx(num_terms, -LIM, LIM, name!(x), Parity::Odd)
        .unwrap()
        .use_suffix(Some("f32".to_string()))
        .unwrap()
        .into_inner();

    quote!(
        fn atan(x: f32) -> f32 {
            use std::f32::consts::PI;
            let c = if x < 0.0 { -PI/2.0 } else { PI/2.0 };
            let small = x.abs() < #LIM;
            let x = if small { x } else { x.recip() };
            let y = #approx ;
            if small { y } else { c - y }
        }
    )
}

