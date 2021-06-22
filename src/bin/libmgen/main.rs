use quote::quote;
use std::io::Write;

mod aux;
mod hyperbolic;
mod inv_trig;
mod log_exp;
mod recip_sqrt;
mod test;
mod trig;

use aux::*;
use hyperbolic::*;
use inv_trig::*;
use log_exp::*;
use recip_sqrt::*;
use test::*;
use trig::*;

fn generate_libm(path: &str, num_bits: usize) -> std::io::Result<()> {
    let mut file = std::fs::File::create(path)?;

    // write!(file, "\n{}\n", gen_sin(16, false))?;
    let (trig, trig_tests) = gen_quadrant_trig(num_bits);
    let (inv_trig, inv_trig_tests) = gen_inv_trig(num_bits);

    let functions = quote!(
        #trig
        #inv_trig
    );

    let tests = quote!(
        #trig_tests
        #inv_trig_tests
    );

    file.write_all(functions.to_string().as_bytes())?;
    file.write_all(tests.to_string().as_bytes())?;

    write!(file, "\n{}\n", gen_exp(7))?;
    write!(file, "\n{}\n", gen_exp2(7))?;
    write!(file, "\n{}\n", gen_exp_m1(7))?;
    write!(file, "\n{}\n", gen_exp2_approx(1))?;
    write!(file, "\n{}\n", gen_negate_on_odd(32))?;

    write!(file, "\n{}\n", gen_ln(9))?;
    write!(file, "\n{}\n", gen_ln_1p(9))?;
    write!(file, "\n{}\n", gen_log2(9))?;
    write!(file, "\n{}\n", gen_log10(9))?;
    write!(file, "\n{}\n", gen_log(9))?;
    write!(file, "\n{}\n", gen_log2_approx(1))?;

    write!(file, "\n{}\n", gen_sinh(7))?;
    write!(file, "\n{}\n", gen_cosh(7))?;
    write!(file, "\n{}\n", gen_tanh(7))?;

    write!(file, "\n{}\n", gen_asinh(7))?;
    write!(file, "\n{}\n", gen_acosh(7))?;
    write!(file, "\n{}\n", gen_atanh(7))?;

    write!(file, "\n{}\n", gen_sqrt(16))?;
    write!(file, "\n{}\n", gen_cbrt(16))?;
    write!(file, "\n{}\n", gen_hypot(16))?;
    write!(file, "\n{}\n", gen_recip(16))?;
    write!(file, "\n{}\n", gen_recip_approx(1))?;
    write!(file, "\n{}\n", gen_powf(16))?;
    write!(file, "\n{}\n", gen_powi(16))?;

    // One bit of a f32 mantissa.
    let bit = (2.0_f64).powi(-23);


    write!(
        file,
        "\n{}\n",
        gen_test(
            quote!(test_exp_a),
            quote!(x.exp()),
            quote!(exp(x as f32) as f64),
            bit * 3.0,
            0.0,
            1.0
        )
    )?;
    write!(
        file,
        "\n{}\n",
        gen_test(
            quote!(test_exp_b),
            quote!(x.exp()),
            quote!(exp(x as f32) as f64),
            bit * 10.0,
            1.0,
            2.0
        )
    )?;
    write!(
        file,
        "\n{}\n",
        gen_test(
            quote!(test_exp_m1),
            quote!(x.exp_m1()),
            quote!(exp_m1(x as f32) as f64),
            bit * 3.0,
            0.0,
            1.0
        )
    )?;
    write!(
        file,
        "\n{}\n",
        gen_test(
            quote!(test_exp2),
            quote!(x.exp2()),
            quote!(exp2(x as f32) as f64),
            bit * 2.0,
            0.0,
            1.0
        )
    )?;
    write!(
        file,
        "\n{}\n",
        gen_test(
            quote!(test_exp2_x),
            quote!(x.exp2()),
            quote!(exp2_approx(x as f32) as f64),
            0.05,
            0.0,
            1.0
        )
    )?;

    write!(
        file,
        "\n{}\n",
        gen_test(
            quote!(test_ln),
            quote!(x.ln()),
            quote!(ln(x as f32) as f64),
            bit * 2.0,
            1.0,
            std::f64::consts::E
        )
    )?;
    write!(
        file,
        "\n{}\n",
        gen_test(
            quote!(test_ln_1p_a),
            quote!(x.ln_1p()),
            quote!(ln_1p(x as f32) as f64),
            bit * 2.0,
            0.0,
            1.0
        )
    )?;
    write!(
        file,
        "\n{}\n",
        gen_test(
            quote!(test_ln_1p_b),
            quote!(x.ln_1p()),
            quote!(ln_1p(x as f32) as f64),
            bit * 3.0,
            1.0,
            std::f64::consts::E * 3.0 - 1.0
        )
    )?;
    write!(
        file,
        "\n{}\n",
        gen_test(
            quote!(test_log2),
            quote!(x.log2()),
            quote!(log2(x as f32) as f64),
            bit * 2.0,
            0.25,
            4.25
        )
    )?;
    write!(
        file,
        "\n{}\n",
        gen_test(
            quote!(test_log10),
            quote!(x.log10()),
            quote!(log10(x as f32) as f64),
            bit * 2.0,
            0.1,
            10.1
        )
    )?;
    write!(
        file,
        "\n{}\n",
        gen_test(
            quote!(test_log_2),
            quote!(x.log(2.0)),
            quote!(log(x as f32, 2.0) as f64),
            bit * 2.0,
            0.5,
            1.5
        )
    )?;
    write!(
        file,
        "\n{}\n",
        gen_test(
            quote!(test_log_e),
            quote!(x.log(std::f64::consts::E)),
            quote!(log(x as f32, std::f32::consts::E) as f64),
            bit * 2.0,
            0.5,
            1.5
        )
    )?;
    write!(
        file,
        "\n{}\n",
        gen_test(
            quote!(test_log_2x),
            quote!(x.log2()),
            quote!(log2_approx(x as f32) as f64),
            0.05,
            0.5,
            10.5
        )
    )?;

    write!(
        file,
        "\n{}\n",
        gen_test(
            quote!(test_cosh),
            quote!(x.cosh()),
            quote!(cosh(x as f32) as f64),
            bit * 2.0,
            -1.0,
            1.0
        )
    )?;
    write!(
        file,
        "\n{}\n",
        gen_test(
            quote!(test_sinh),
            quote!(x.sinh()),
            quote!(sinh(x as f32) as f64),
            bit * 2.0,
            -1.0,
            1.0
        )
    )?;
    write!(
        file,
        "\n{}\n",
        gen_test(
            quote!(test_tanh),
            quote!(x.tanh()),
            quote!(tanh(x as f32) as f64),
            bit * 2.0,
            -1.0,
            1.0
        )
    )?;

    write!(
        file,
        "\n{}\n",
        gen_test(
            quote!(test_acosh),
            quote!(x.acosh()),
            quote!(acosh(x as f32) as f64),
            bit * 2.0,
            -1.0,
            1.0
        )
    )?;
    write!(
        file,
        "\n{}\n",
        gen_test(
            quote!(test_asinh),
            quote!(x.asinh()),
            quote!(asinh(x as f32) as f64),
            bit * 3.0,
            -1.0,
            1.0
        )
    )?;
    write!(
        file,
        "\n{}\n",
        gen_test(
            quote!(test_atanh),
            quote!(x.atanh()),
            quote!(atanh(x as f32) as f64),
            bit * 3.0,
            -0.9,
            0.9
        )
    )?;



    write!(
        file,
        "\n{}\n",
        gen_test(
            quote!(test_hypot_a),
            quote!(x.hypot(1.0)),
            quote!(hypot(x as f32, 1.0) as f64),
            bit * 3.0,
            -1.0,
            1.0
        )
    )?;
    write!(
        file,
        "\n{}\n",
        gen_test(
            quote!(test_hypot_b),
            quote!(x.hypot(-1.0)),
            quote!(hypot(x as f32, -1.0) as f64),
            bit * 3.0,
            -1.0,
            1.0
        )
    )?;
    write!(
        file,
        "\n{}\n",
        gen_test(
            quote!(test_hypot_c),
            quote!((1.0_f64).hypot(x)),
            quote!(hypot(1.0, x as f32) as f64),
            bit * 3.0,
            -1.0,
            1.0
        )
    )?;
    write!(
        file,
        "\n{}\n",
        gen_test(
            quote!(test_hypot_d),
            quote!((-1.0_f64).hypot(x)),
            quote!(hypot(-1.0, x as f32) as f64),
            bit * 3.0,
            -1.0,
            1.0
        )
    )?;

    write!(
        file,
        "\n{}\n",
        gen_test(
            quote!(test_sqrt),
            quote!(x.sqrt()),
            quote!(sqrt(x as f32) as f64),
            bit * 1.0,
            0.5,
            2.0
        )
    )?;
    write!(
        file,
        "\n{}\n",
        gen_test(
            quote!(test_cbrt),
            quote!(x.cbrt()),
            quote!(cbrt(x as f32) as f64),
            bit * 1.0,
            -2.0,
            2.0
        )
    )?;
    write!(
        file,
        "\n{}\n",
        gen_test(
            quote!(test_recip),
            quote!(x.recip()),
            quote!(recip(x as f32) as f64),
            bit * 2.0,
            0.5,
            1.5
        )
    )?;
    write!(
        file,
        "\n{}\n",
        gen_test(
            quote!(test_recip_n),
            quote!(x.recip()),
            quote!(recip(x as f32) as f64),
            bit * 2.0,
            -1.5,
            -0.5
        )
    )?;
    write!(
        file,
        "\n{}\n",
        gen_test(
            quote!(test_recip_x),
            quote!(x.recip()),
            quote!(recip_approx(x as f32) as f64),
            0.1,
            0.5,
            1.5
        )
    )?;
    write!(
        file,
        "\n{}\n",
        gen_test(
            quote!(test_recip_y),
            quote!(x.recip()),
            quote!(recip_approx(x as f32) as f64),
            0.1,
            -1.5,
            -0.5
        )
    )?;

    write!(
        file,
        "\n{}\n",
        gen_test(
            quote!(test_powf_2),
            quote!(x.powf(2.0)),
            quote!(powf(x as f32, 2.0) as f64),
            bit * 4.0,
            0.5,
            1.5
        )
    )?;
    write!(
        file,
        "\n{}\n",
        gen_test(
            quote!(test_powf_m1),
            quote!(x.powf(-1.0)),
            quote!(powf(x as f32, -1.0) as f64),
            bit * 4.0,
            0.5,
            1.5
        )
    )?;

    write!(
        file,
        "\n{}\n",
        gen_test(
            quote!(test_powi_2),
            quote!(x.powi(2)),
            quote!(powi(x as f32, 2) as f64),
            bit * 2.0,
            0.5,
            1.5
        )
    )?;
    write!(
        file,
        "\n{}\n",
        gen_test(
            quote!(test_powi_3),
            quote!(x.powi(3)),
            quote!(powi(x as f32, 3) as f64),
            bit * 4.0,
            0.12,
            1.2
        )
    )?;
    write!(
        file,
        "\n{}\n",
        gen_test(
            quote!(test_powi_m1),
            quote!(x.powi(-1)),
            quote!(powi(x as f32, -1) as f64),
            bit * 2.0,
            0.5,
            1.5
        )
    )?;
    write!(
        file,
        "\n{}\n",
        gen_test(
            quote!(test_powi_m2),
            quote!(x.powi(-2)),
            quote!(powi(x as f32, -2) as f64),
            bit * 6.0,
            0.5,
            1.5
        )
    )?;
    write!(
        file,
        "\n{}\n",
        gen_test(
            quote!(test_powi_16),
            quote!(x.powi(16)),
            quote!(powi(x as f32, 16) as f64),
            bit * 7.0,
            0.25,
            1.0
        )
    )?;
    //write!(file, "\n{}\n", gen_test("powi_16", quote!(x.powi(16)), quote!(powf(x as f32, 16.0) as f64), bit*6.0, 0.5, 1.5))?;

    Ok(())
}

fn main() {
    generate_libm("tests/libm.rs", 32).unwrap();
}
