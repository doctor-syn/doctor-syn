use quote::{quote};

pub fn gen_sqrt(_num_terms: usize) -> proc_macro2::TokenStream {
    // Probably better done with a reciprocal estimate or bitcast log divide.
    //
    // Given an estimate r of a square root:
    //
    // if (r + e).pow(2) = x
    //
    // r.pow(2) + 2*r*e + e.pow(2) = x
    //
    // e = (x - r.pow(2)) / 2*r.pow(2) + O(e.pow(2))
    //
    // ie. the Babylonian!

    quote!(
        fn sqrt(x: f32) -> f32 {
            let r = exp2(log2(x) * (1.0/2.0));
            let y = r + (x - r*r) / (2.0*r);
            y
        }
    )
}

pub fn gen_cbrt(_num_terms: usize) -> proc_macro2::TokenStream {
    // Probably better done with a bitcast log divide.
    //
    // Given an estimate r of a cube root:
    //
    // if (r + e).pow(3) = x
    //
    // r.pow(3) + 3*r.pow(2)*e + 3*r*e.pow(2) + e.pow(3) = x
    //
    // e = (x - r.pow(3)) / 3*r.pow(2) + O(e.pow(2))

    quote!(
        fn cbrt(x: f32) -> f32 {
            let r = exp2(log2(x.abs()) * (1.0/3.0));
            let y = r + (x.abs() - r*r*r) / (3.0*r*r);
            if x < 0.0 { -y } else { y }
        }
    )
}

pub fn gen_recip(_num_terms: usize) -> proc_macro2::TokenStream {
    // Probably better done with a reciprocal estimate and refinement.
    //
    // Given an estimate r of a reciprocal 1/x
    //
    // r' = x * ( 2.0 - x * r )
    //
    // is a better estimate.

    quote!(
        fn recip(x: f32) -> f32 {
            //let r = exp2_approx(-log2_approx(x));
            let r = recip_approx(x);
            let r = r * ( 2.0 - x * r );
            let r = r * ( 2.0 - x * r );
            let r = r * ( 2.0 - x * r );
            r
        }
    )
}

pub fn gen_hypot(_num_terms: usize) -> proc_macro2::TokenStream {
    // see https://en.wikipedia.org/wiki/Hypot
    //
    quote!(
        fn hypot(x: f32, y: f32) -> f32 {
            let (x, y) = if x.abs() > y.abs() { (x, y) } else { (y, x) };
            if x.abs() <= f32::MIN_POSITIVE { x } else { x.abs()*(1.0 + (y/x)*(y/x)).sqrt() }
        }
    )
}

