use quote::quote;
use std::io::Write;

mod auxfuncs;
mod hyperbolic;
mod inv_trig;
mod log_exp;
mod recip_sqrt;
mod test;
mod trig;
mod helpers;

use hyperbolic::*;
use inv_trig::*;
use log_exp::*;
use recip_sqrt::*;
use trig::*;
use auxfuncs::*;

fn generate_libm(path: &str, num_bits: usize, number_type: &str) -> std::io::Result<()> {
    let mut file = std::fs::File::create(path)?;

    let (trig, trig_tests) = gen_quadrant_trig(num_bits, number_type);
    let (inv_trig, inv_trig_tests) = gen_inv_trig(num_bits, number_type);
    let (log_exp, log_exp_tests) = gen_log_exp(num_bits, number_type);
    let (hyperbolic, hyperbolic_tests) = gen_hyperbolic(num_bits, number_type);
    let (recip_sqrt, recip_sqrt_tests) = gen_recip_sqrt(num_bits, number_type);
    let (aux, aux_tests) = gen_aux(num_bits, number_type);

    let functions = quote!(
        #trig
        #inv_trig
        #log_exp
        #hyperbolic
        #recip_sqrt
        #aux
    );

    let tests = quote!(
        #trig_tests
        #inv_trig_tests
        #log_exp_tests
        #hyperbolic_tests
        #recip_sqrt_tests
        #aux_tests
    );

    file.write_all(functions.to_string().as_bytes())?;
    file.write_all(tests.to_string().as_bytes())?;

    Ok(())
}


fn main() {
    // let val = doctor_syn::expr!((2.16065388452409390396).cos()).eval(60).unwrap();
    // let bd : doctor_syn::bigdecimal::BigDecimal = val.into();
    //let val = doctor_syn::expr!(123456789.123456789123456789123456789123456789).eval(60).unwrap();
    //let bd : bigdecimal::BigDecimal = "123456789.123456789123456789123456789123456789".parse().unwrap();
    //let val = doctor_syn::expr!(123456789.123456789123456789123456789123456789);
    //let val = doctor_syn::Expression::from(bd);
    // println!("val={}", val);
    // println!("bd={}", bd);
    generate_libm("tests/libm32.rs", 32, "f32_hex").unwrap();
    generate_libm("tests/libm64.rs", 64, "f64_hex").unwrap();
}
