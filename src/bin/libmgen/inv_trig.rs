use crate::helpers;
use doctor_syn::Parity;
use doctor_syn::{expr, name, num_digits_for};
use proc_macro2::TokenStream;
use quote::quote;

use crate::test::gen_test;

pub fn gen_atan2(num_terms: usize, num_bits: usize, number_type: &str) -> TokenStream {
    let fty = helpers::get_fty(num_bits);

    let xmin = -1.0;
    let xmax = 1.0;

    let approx = expr!(x3.atan())
        .approx(
            num_terms,
            xmin,
            xmax,
            name!(x3),
            Parity::Odd,
            num_digits_for(num_bits),
        )
        .unwrap()
        .use_number_type(number_type)
        .unwrap()
        .into_inner();

    // TODO: calculate the recipocal without a divide.
    quote!(
        fn atan2(y: #fty, x: #fty) -> #fty {
            let offset180 : #fty = select(y < 0.0, -PI, PI );
            let x1 : #fty = select(x < 0.0, -x, x );
            let y1 : #fty = select(x < 0.0, -y, y );
            let offset1 : #fty = select(x < 0.0, offset180, 0.0 );
            let offset90 : #fty = select(y < 0.0, -PI/2.0, PI/2.0 );
            let x2 : #fty = select(y1.abs() > x1, y1, x1 );
            let y2 : #fty = select(y1.abs() > x1, -x1, y1 );
            let offset2 : #fty = select(y1.abs() > x1, offset1 + offset90, offset1 );
            let x3 : #fty = y2 / x2;
            let y3 : #fty = #approx ;
            y3 + offset2
        }
    )
}

pub fn gen_asin(num_terms: usize, num_bits: usize, number_type: &str) -> TokenStream {
    let fty = helpers::get_fty(num_bits);
    let lim = quote!(0.9);

    let approx = expr!(x.asin())
        .approx(
            num_terms,
            -0.9,
            0.9,
            name!(x),
            Parity::Odd,
            num_digits_for(num_bits),
        )
        .unwrap()
        .use_number_type(number_type)
        .unwrap()
        .into_inner();

    quote!(
        fn asin(arg: #fty) -> #fty {
            const LIM : #fty = #lim;
            let c : #fty = select(arg < 0.0, -PI/2.0, PI/2.0 );
            let s : #fty = select(arg < 0.0 , -1.0, 1.0  );
            let x : #fty = select(arg * arg < LIM * LIM, arg, (1.0-arg*arg).sqrt() );
            let y : #fty = #approx ;
            select(arg*arg < LIM * LIM, y, c - y * s)
        }
    )
}

pub fn gen_acos(num_terms: usize, num_bits: usize, number_type: &str) -> TokenStream {
    let fty = helpers::get_fty(num_bits);
    let lim = quote!(0.9);

    let approx = expr!(x.asin())
        .approx(
            num_terms,
            -0.9,
            0.9,
            name!(x),
            Parity::Odd,
            num_digits_for(num_bits),
        )
        .unwrap()
        .use_number_type(number_type)
        .unwrap()
        .into_inner();

    quote!(
        fn acos(arg: #fty) -> #fty {
            const LIM : #fty = #lim;
            let c : #fty = select(arg < 0.0, PI, 0.0 );
            let s : #fty = select(arg < 0.0, 1.0, -1.0  );
            let x : #fty = select(arg * arg < LIM * LIM, arg, (1.0-arg*arg).sqrt() );
            let y : #fty = #approx ;
            select(arg*arg < LIM * LIM, PI/2.0 - y, c - y * s )
        }
    )
}

pub fn gen_atan(num_terms: usize, num_bits: usize, number_type: &str) -> TokenStream {
    let fty = helpers::get_fty(num_bits);
    let lim = quote!(1.0);

    let approx = expr!(x.atan())
        .approx(
            num_terms,
            -1.0,
            1.0,
            name!(x),
            Parity::Odd,
            num_digits_for(num_bits),
        )
        .unwrap()
        .use_number_type(number_type)
        .unwrap()
        .into_inner();

    quote!(
        fn atan(arg: #fty) -> #fty {
            const LIM : #fty = #lim;

            let c : #fty = select(arg < 0.0, -PI/2.0, PI/2.0);
            let small : bool = arg.abs() < LIM;
            let x : #fty = select(small, arg, arg.recip());
            let y : #fty = #approx ;
            select(small, y, c - y)
        }
    )
}

// Generate accurate sin, cos, tan, sin_cos.
// Return functions and tests.
pub fn gen_inv_trig(num_bits: usize, number_type: &str) -> (TokenStream, TokenStream) {
    let atan2 = gen_atan2(16, num_bits, number_type);
    let asin = gen_asin(22, num_bits, number_type);
    let acos = gen_acos(22, num_bits, number_type);
    let atan = gen_atan(22, num_bits, number_type);

    let fty = helpers::get_fty(num_bits);

    let bit = (2.0_f64).powi(if num_bits == 32 { 23 } else { 52 });

    let test_asin = gen_test(
        quote!(test_asin),
        quote!(x.asin()),
        quote!(asin(x as #fty) as f64),
        bit * 9.0,
        -0.99,
        0.99,
    );
    let test_acos = gen_test(
        quote!(test_acos),
        quote!(x.acos()),
        quote!(acos(x as #fty) as f64),
        bit * 9.0,
        -0.99,
        0.99,
    );
    let test_atan = gen_test(
        quote!(test_atan),
        quote!(x.atan()),
        quote!(atan(x as #fty) as f64),
        bit * 2.0,
        -2.0,
        2.0,
    );

    let test_atan2_a = gen_test(
        quote!(test_atan2_a),
        quote!(x.atan2(1.0)),
        quote!(atan2(x as #fty, 1.0) as f64),
        bit * 3.0,
        -1.0,
        1.0,
    );
    let test_atan2_b = gen_test(
        quote!(test_atan2_b),
        quote!(x.atan2(-1.0)),
        quote!(atan2(x as #fty, -1.0) as f64),
        bit * 3.0,
        -1.0,
        1.0,
    );
    let test_atan2_c = gen_test(
        quote!(test_atan2_c),
        quote!((1.0_f64).atan2(x)),
        quote!(atan2(1.0, x as #fty) as f64),
        bit * 3.0,
        -1.0,
        1.0,
    );
    let test_atan2_d = gen_test(
        quote!(test_atan2_d),
        quote!((-1.0_f64).atan2(x)),
        quote!(atan2(-1.0, x as #fty) as f64),
        bit * 3.0,
        -1.0,
        1.0,
    );

    (
        quote! {
            #asin #acos #atan #atan2
        },
        quote! {
            #test_asin #test_acos #test_atan #test_atan2_a #test_atan2_b #test_atan2_c #test_atan2_d
        },
    )
}
