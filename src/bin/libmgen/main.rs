use quote::quote;
use std::io::Write;

mod aux;
mod hyperbolic;
mod inv_trig;
mod log_exp;
mod recip_sqrt;
mod test;
mod trig;

use hyperbolic::*;
use inv_trig::*;
use log_exp::*;
use recip_sqrt::*;
use trig::*;
use aux::*;

fn generate_libm(path: &str, num_bits: usize) -> std::io::Result<()> {
    let mut file = std::fs::File::create(path)?;

    // write!(file, "\n{}\n", gen_sin(16, false))?;
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

    //write!(file, "\n{}\n", gen_test("powi_16", quote!(x.powi(16)), quote!(powf(x as f32, 16.0) as f64), bit*6.0, 0.5, 1.5))?;

    Ok(())
}

fn main() {
    generate_libm("tests/libm.rs", 32).unwrap();
}
