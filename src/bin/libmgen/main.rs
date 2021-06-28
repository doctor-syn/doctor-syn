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

fn generate_libm(path: &str, num_bits: usize) -> std::io::Result<()> {
    let mut file = std::fs::File::create(path)?;

    let (trig, trig_tests) = gen_quadrant_trig(num_bits);
    let (inv_trig, inv_trig_tests) = gen_inv_trig(num_bits);
    let (log_exp, log_exp_tests) = gen_log_exp(num_bits);
    let (hyperbolic, hyperbolic_tests) = gen_hyperbolic(num_bits);
    let (recip_sqrt, recip_sqrt_tests) = gen_recip_sqrt(num_bits);
    let (aux, aux_tests) = gen_aux(num_bits);

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
    let val = doctor_syn::expr!((2.16065388452409390396).cos()).eval(60).unwrap();
    println!("val={}", val);
    generate_libm("tests/libm32.rs", 32).unwrap();
    generate_libm("tests/libm64.rs", 64).unwrap();
}
