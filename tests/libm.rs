fn sin(x: f32) -> f32 {
    let x = x * (1.0 / (std::f32::consts::PI * 2.0));
    let x = x - x.floor() - 0.5;
    (0.6150599377704147_f32)
        .mul_add(x * x, -3.776312346215613_f32)
        .mul_add(x * x, 15.084843206874782_f32)
        .mul_add(x * x, -42.05746026953019_f32)
        .mul_add(x * x, 76.70577449290244_f32)
        .mul_add(x * x, -81.60524634871001_f32)
        .mul_add(x * x, 41.34170220158861_f32)
        .mul_add(x * x, -6.283185307093742_f32)
        * x
}

fn cos(x: f32) -> f32 {
    let x = x * (1.0 / (std::f32::consts::PI * 2.0));
    let x = x - x.floor() - 0.5;
    (-0.2437628622134172_f32)
        .mul_add(x * x, 1.6969999270888276_f32)
        .mul_add(x * x, -7.899269307802109_f32)
        .mul_add(x * x, 26.42565411950429_f32)
        .mul_add(x * x, -60.24459263234794_f32)
        .mul_add(x * x, 85.45681509594338_f32)
        .mul_add(x * x, -64.93939398117197_f32)
        .mul_add(x * x, 19.739208801937608_f32)
        .mul_add(x * x, -0.9999999999999996_f32)
}

fn exp(x: f32) -> f32 {
    let x = x * std::f32::consts::LOG2_E;
    let mul = f32::from_bits((x.floor() * 0x00800000 as f32 + 0x3f800000 as f32) as u32);
    let x = x - x.floor() - 0.5;
    (0.00021877504780304022_f32)
        .mul_add(x, 0.0018964605237938004_f32)
        .mul_add(x, 0.01360194957589631_f32)
        .mul_add(x, 0.07849305736942819_f32)
        .mul_add(x, 0.3397315896731585_f32)
        .mul_add(x, 0.980258206874906_f32)
        .mul_add(x, 1.414213562373095_f32)
        * mul
}

fn exp2(x: f32) -> f32 {
    let mul = f32::from_bits((x.floor() * 0x00800000 as f32 + 0x3f800000 as f32) as u32);
    let x = x - x.floor() - 0.5;
    (0.00021877504780304022_f32)
        .mul_add(x, 0.0018964605237938004_f32)
        .mul_add(x, 0.01360194957589631_f32)
        .mul_add(x, 0.07849305736942819_f32)
        .mul_add(x, 0.3397315896731585_f32)
        .mul_add(x, 0.980258206874906_f32)
        .mul_add(x, 1.414213562373095_f32)
        * mul
}

fn exp_m1(x: f32) -> f32 {
    let x = x * std::f32::consts::LOG2_E;
    let mul = f32::from_bits((x.floor() * 0x00800000 as f32 + 0x3f800000 as f32) as u32);
    let x = x - x.floor() - 0.5;
    (0.00021877504780304022_f32)
        .mul_add(x, 0.0018964605237938004_f32)
        .mul_add(x, 0.01360194957589631_f32)
        .mul_add(x, 0.07849305736942819_f32)
        .mul_add(x, 0.3397315896731585_f32)
        .mul_add(x, 0.980258206874906_f32)
        .mul_add(x, 0.414213562373095_f32)
        * mul
        + (mul - 1.0)
}

fn ln(x: f32) -> f32 {
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
    (y + (exponent as f32)) * (1.0 / std::f32::consts::LOG2_E)
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
    assert!(max_error < 0.0000005006790161132813);
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
    assert!(max_error < 0.0000002384185791015625);
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
