fn sin(x: f32) -> f32 {
    let x = x * (1.0 / (std::f32::consts::PI * 2.0));
    let x = x - x.round();
    (-0.6150599377704147_f32)
        .mul_add(x * x, 3.776312346215613_f32)
        .mul_add(x * x, -15.084843206874782_f32)
        .mul_add(x * x, 42.05746026953019_f32)
        .mul_add(x * x, -76.70577449290244_f32)
        .mul_add(x * x, 81.60524634871001_f32)
        .mul_add(x * x, -41.34170220158861_f32)
        .mul_add(x * x, 6.283185307093742_f32)
        * x
}

fn cos(x: f32) -> f32 {
    let x = x * (1.0 / (std::f32::consts::PI * 2.0));
    let x = x - x.round();
    (0.2437628622134172_f32)
        .mul_add(x * x, -1.6969999270888276_f32)
        .mul_add(x * x, 7.899269307802109_f32)
        .mul_add(x * x, -26.42565411950429_f32)
        .mul_add(x * x, 60.24459263234794_f32)
        .mul_add(x * x, -85.45681509594338_f32)
        .mul_add(x * x, 64.93939398117197_f32)
        .mul_add(x * x, -19.739208801937608_f32)
        .mul_add(x * x, 0.9999999999999996_f32)
}

fn exp(x: f32) -> f32 {
    exp2(x * std::f32::consts::LOG2_E)
}

fn exp2(x: f32) -> f32 {
    let r = x.round();
    let mul = f32::from_bits((r.mul_add(0x00800000 as f32, 0x3f800000 as f32)) as u32);
    let x = x - r;
    (0.00015469731983042893_f32)
        .mul_add(x, 0.0013410000966213198_f32)
        .mul_add(x, 0.00961803078248943_f32)
        .mul_add(x, 0.05550297314198923_f32)
        .mul_add(x, 0.24022651084117352_f32)
        .mul_add(x, 0.6931472253950114_f32)
        .mul_add(x, 1_f32)
        * mul
}

fn exp_m1(x: f32) -> f32 {
    let x = x * std::f32::consts::LOG2_E;
    let r = x.round();
    let mul = f32::from_bits((r.mul_add(0x00800000 as f32, 0x3f800000 as f32)) as u32);
    let x = x - r;
    (0.00015469731983042893_f32)
        .mul_add(x, 0.0013410000966213198_f32)
        .mul_add(x, 0.00961803078248943_f32)
        .mul_add(x, 0.05550297314198923_f32)
        .mul_add(x, 0.24022651084117352_f32)
        .mul_add(x, 0.6931472253950114_f32)
        .mul_add(x, 0.00000000000000003991219299848534_f32)
        * mul
        + (mul - 1.0)
}

fn ln(x: f32) -> f32 {
    log2(x) * (1.0 / std::f32::consts::LOG2_E)
}

fn ln_1p(x: f32) -> f32 {
    let exponent = ((x + 1.0).to_bits() >> 23) as i32 - 0x7f;
    let x = if exponent == 0 {
        x
    } else {
        f32::from_bits(((x + 1.0).to_bits() & 0x7fffff) | 0x3f800000) - 1.0
    };
    let y: f32 = (-0.008874696649844664_f32)
        .mul_add(x, 0.05061587563631424_f32)
        .mul_add(x, -0.13573160547317467_f32)
        .mul_add(x, 0.24089975662254867_f32)
        .mul_add(x, -0.34715166209993725_f32)
        .mul_add(x, 0.4787361538138562_f32)
        .mul_add(x, -0.72118516198798_f32)
        .mul_add(x, 1.4426913401382175_f32)
        .mul_add(x, 0_f32);
    (y + (exponent as f32)) * (1.0 / std::f32::consts::LOG2_E)
}

fn log2(x: f32) -> f32 {
    let exponent = (x.to_bits() >> 23) as i32 - 0x7f;
    let x = f32::from_bits((x.to_bits() & 0x7fffff) | 0x3f800000) - 1.5;
    let y: f32 = (-0.008874696649735232_f32)
        .mul_add(x, 0.01511708903698527_f32)
        .mul_add(x, -0.020698917295047616_f32)
        .mul_add(x, 0.03731541074473215_f32)
        .mul_add(x, -0.07127813300215657_f32)
        .mul_add(x, 0.14254471630463159_f32)
        .mul_add(x, -0.32059812016799316_f32)
        .mul_add(x, 0.9617954032360934_f32)
        .mul_add(x, 0.5849625007211562_f32);
    y + (exponent as f32)
}

fn log10(x: f32) -> f32 {
    log2(x) * (1.0 / std::f32::consts::LOG2_10)
}

fn sin_cos(x: f32) -> (f32, f32) {
    (sin(x), cos(x))
}

fn tan(x: f32) -> f32 {
    let x = x * (1.0 / (std::f32::consts::PI));
    let x = x - x.round();
    let recip = 1.0 / (x * x - 0.25);
    let y = (0.003724279591527462_f32)
        .mul_add(x * x, 0.0033400372042702393_f32)
        .mul_add(x * x, 0.010160277239937699_f32)
        .mul_add(x * x, 0.02247279470368399_f32)
        .mul_add(x * x, 0.05263967271784766_f32)
        .mul_add(x * x, 0.1347692905516595_f32)
        .mul_add(x * x, 0.5577362650687226_f32)
        .mul_add(x * x, -0.7853981634007947_f32)
        * x;
    y * recip
}

fn sinh(x: f32) -> f32 {
    let a = x.mul_add(std::f32::consts::LOG2_E, -1.0);
    let b = x.mul_add(-std::f32::consts::LOG2_E, -1.0);
    exp2(a) - exp2(b)
}

fn cosh(x: f32) -> f32 {
    let a = x.mul_add(std::f32::consts::LOG2_E, -1.0);
    let b = x.mul_add(-std::f32::consts::LOG2_E, -1.0);
    exp2(a) + exp2(b)
}

#[test]
fn test_sin() {
    const N: i32 = 0x100000;
    let tmin = -3.1415926535897931;
    let tmax = 3.1415926535897931;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.sin();
        let y2 = sin(x as f32) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16}\ny2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("sin me={:20}", max_error);
    assert!(max_error < 0.0000007152557373046875);
}

#[test]
fn test_cos() {
    const N: i32 = 0x100000;
    let tmin = -3.1415926535897931;
    let tmax = 3.1415926535897931;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.cos();
        let y2 = cos(x as f32) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16}\ny2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("cos me={:20}", max_error);
    assert!(max_error < 0.00000035762786865234375);
}

#[test]
fn test_tan_a() {
    const N: i32 = 0x100000;
    let tmin = -0.7853981633974483;
    let tmax = 0.7853981633974483;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.tan();
        let y2 = tan(x as f32) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16}\ny2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("tan me={:20}", max_error);
    assert!(max_error < 0.0000002384185791015625);
}

#[test]
fn test_tan_b() {
    const N: i32 = 0x100000;
    let tmin = -1.0471975511965976;
    let tmax = 1.0471975511965976;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.tan();
        let y2 = tan(x as f32) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16}\ny2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("tan me={:20}", max_error);
    assert!(max_error < 0.0000008344650268554688);
}

#[test]
fn test_exp_a() {
    const N: i32 = 0x100000;
    let tmin = 0.0000000000000000;
    let tmax = 1.0000000000000000;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.exp();
        let y2 = exp(x as f32) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16}\ny2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("exp me={:20}", max_error);
    assert!(max_error < 0.00000035762786865234375);
}

#[test]
fn test_exp_b() {
    const N: i32 = 0x100000;
    let tmin = 1.0000000000000000;
    let tmax = 2.0000000000000000;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.exp();
        let y2 = exp(x as f32) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16}\ny2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("exp me={:20}", max_error);
    assert!(max_error < 0.0000011920928955078125);
}

#[test]
fn test_exp_m1() {
    const N: i32 = 0x100000;
    let tmin = 0.0000000000000000;
    let tmax = 1.0000000000000000;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.exp_m1();
        let y2 = exp_m1(x as f32) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16}\ny2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("exp_m1 me={:20}", max_error);
    assert!(max_error < 0.00000035762786865234375);
}

#[test]
fn test_exp2() {
    const N: i32 = 0x100000;
    let tmin = 0.0000000000000000;
    let tmax = 1.0000000000000000;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.exp2();
        let y2 = exp2(x as f32) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16}\ny2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("exp2 me={:20}", max_error);
    assert!(max_error < 0.0000002384185791015625);
}

#[test]
fn test_ln() {
    const N: i32 = 0x100000;
    let tmin = 1.0000000000000000;
    let tmax = 2.7182818284590451;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.ln();
        let y2 = ln(x as f32) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16}\ny2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("ln me={:20}", max_error);
    assert!(max_error < 0.0000002384185791015625);
}

#[test]
fn test_ln_1p_a() {
    const N: i32 = 0x100000;
    let tmin = 0.0000000000000000;
    let tmax = 1.0000000000000000;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.ln_1p();
        let y2 = ln_1p(x as f32) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16}\ny2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("ln_1p me={:20}", max_error);
    assert!(max_error < 0.0000002384185791015625);
}

#[test]
fn test_ln_1p_b() {
    const N: i32 = 0x100000;
    let tmin = 1.0000000000000000;
    let tmax = 7.1548454853771357;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.ln_1p();
        let y2 = ln_1p(x as f32) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16}\ny2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("ln_1p me={:20}", max_error);
    assert!(max_error < 0.00000035762786865234375);
}

#[test]
fn test_log2() {
    const N: i32 = 0x100000;
    let tmin = 0.2500000000000000;
    let tmax = 4.2500000000000000;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.log2();
        let y2 = log2(x as f32) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16}\ny2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("log2 me={:20}", max_error);
    assert!(max_error < 0.0000002384185791015625);
}

#[test]
fn test_log10() {
    const N: i32 = 0x100000;
    let tmin = 0.1000000000000000;
    let tmax = 10.0999999999999996;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.log10();
        let y2 = log10(x as f32) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16}\ny2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("log10 me={:20}", max_error);
    assert!(max_error < 0.0000002384185791015625);
}

#[test]
fn test_cosh() {
    const N: i32 = 0x100000;
    let tmin = -1.0000000000000000;
    let tmax = 1.0000000000000000;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.cosh();
        let y2 = cosh(x as f32) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16}\ny2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("cosh me={:20}", max_error);
    assert!(max_error < 0.0000002384185791015625);
}

#[test]
fn test_sinh() {
    const N: i32 = 0x100000;
    let tmin = -1.0000000000000000;
    let tmax = 1.0000000000000000;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.sinh();
        let y2 = sinh(x as f32) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16}\ny2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("sinh me={:20}", max_error);
    assert!(max_error < 0.0000002384185791015625);
}
