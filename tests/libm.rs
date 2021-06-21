fn sin(x: f32) -> f32 {
    let x = x * (1.0 / (std::f32::consts::PI));
    let xh = x + 0.5;
    let xr = x.round();
    let xhr = xh.round();
    let s = x - xr;
    let c = xh - xhr;
    let sr = (-0.5878112252588481_f32)
        .mul_add(s * s, 2.5496484487855455_f32)
        .mul_add(s * s, -5.167705042516404_f32)
        .mul_add(s * s, 3.1415926342503133_f32)
        * s;
    let cr = (-0.23132925050778008_f32)
        .mul_add(c * c, 1.335050718961723_f32)
        .mul_add(c * c, -4.0587078433143_f32)
        .mul_add(c * c, 4.934802176219238_f32)
        .mul_add(c * c, -0.9999999999999999_f32);
    let ss = if (xr as i32) & 1 == 0 { sr } else { -sr };
    let cs = if (xhr as i32 & 1) == 0 { cr } else { -cr };
    if s.abs() <= 0.25 {
        ss
    } else {
        cs
    }
}

fn cos(x: f32) -> f32 {
    let x = x * (1.0 / (std::f32::consts::PI));
    let xh = x + 0.5;
    let xr = x.round();
    let xhr = xh.round();
    let c = x - xr;
    let s = xh - xhr;
    let sr = (-0.5878112252588481_f32)
        .mul_add(s * s, 2.5496484487855455_f32)
        .mul_add(s * s, -5.167705042516404_f32)
        .mul_add(s * s, 3.1415926342503133_f32)
        * s;
    let cr = (0.23132925050778008_f32)
        .mul_add(c * c, -1.335050718961723_f32)
        .mul_add(c * c, 4.0587078433143_f32)
        .mul_add(c * c, -4.934802176219238_f32)
        .mul_add(c * c, 0.9999999999999999_f32);
    let ss = if xhr as i32 & 1 == 0 { sr } else { -sr };
    let cs = if xr as i32 & 1 == 0 { cr } else { -cr };
    if s.abs() <= 0.25 {
        ss
    } else {
        cs
    }
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

fn asin(x: f32) -> f32 {
    use std::f32::consts::PI;
    let c = if x < 0.0 { -PI / 2.0 } else { PI / 2.0 };
    let s = if x < 0.0 { -1.0 } else { 1.0 };
    let x0 = x;
    let x = if x * x < 0.9f32 * 0.9f32 {
        x
    } else {
        (1.0 - x * x).sqrt()
    };
    let y = (5.974205075383973_f32)
        .mul_add(x * x, -21.40061239478606_f32)
        .mul_add(x * x, 33.11909346520203_f32)
        .mul_add(x * x, -28.553023578202207_f32)
        .mul_add(x * x, 14.987170884576244_f32)
        .mul_add(x * x, -4.8476328497648_f32)
        .mul_add(x * x, 0.9964135516015877_f32)
        .mul_add(x * x, -0.06545321396765472_f32)
        .mul_add(x * x, 0.08136564192686366_f32)
        .mul_add(x * x, 0.16652423337405486_f32)
        .mul_add(x * x, 1.0000005238394805_f32)
        * x;
    if x0 * x0 < 0.9f32 * 0.9f32 {
        y
    } else {
        c - y * s
    }
}

fn acos(x: f32) -> f32 {
    use std::f32::consts::PI;
    let c = if x < 0.0 { PI } else { 0.0 };
    let s = if x < 0.0 { 1.0 } else { -1.0 };
    let x0 = x;
    let x = if x * x < 0.9f32 * 0.9f32 {
        x
    } else {
        (1.0 - x * x).sqrt()
    };
    let y = (5.974205075383973_f32)
        .mul_add(x * x, -21.40061239478606_f32)
        .mul_add(x * x, 33.11909346520203_f32)
        .mul_add(x * x, -28.553023578202207_f32)
        .mul_add(x * x, 14.987170884576244_f32)
        .mul_add(x * x, -4.8476328497648_f32)
        .mul_add(x * x, 0.9964135516015877_f32)
        .mul_add(x * x, -0.06545321396765472_f32)
        .mul_add(x * x, 0.08136564192686366_f32)
        .mul_add(x * x, 0.16652423337405486_f32)
        .mul_add(x * x, 1.0000005238394805_f32)
        * x;
    if x0 * x0 < 0.9f32 * 0.9f32 {
        PI / 2.0 - y
    } else {
        c - y * s
    }
}

fn atan(x: f32) -> f32 {
    use std::f32::consts::PI;
    let c = if x < 0.0 { -PI / 2.0 } else { PI / 2.0 };
    let small = x.abs() < 1f32;
    let x = if small { x } else { x.recip() };
    let y = (0.0009143684233841135_f32)
        .mul_add(x * x, -0.006308821933028976_f32)
        .mul_add(x * x, 0.020365370753541973_f32)
        .mul_add(x * x, -0.042089323447971325_f32)
        .mul_add(x * x, 0.06553017681621427_f32)
        .mul_add(x * x, -0.08726065006629005_f32)
        .mul_add(x * x, 0.11034724342349803_f32)
        .mul_add(x * x, -0.1427606087864586_f32)
        .mul_add(x * x, 0.1999935712486494_f32)
        .mul_add(x * x, -0.33333316226314147_f32)
        .mul_add(x * x, 0.9999999992290513_f32)
        * x;
    if small {
        y
    } else {
        c - y
    }
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

fn exp2_approx(x: f32) -> f32 {
    f32::from_bits(
        (x.mul_add(
            0x00800000 as f32,
            0x3f800000 as f32 - 0x00800000 as f32 * 0.04,
        )) as u32,
    )
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

fn log(x: f32, base: f32) -> f32 {
    log2(x) / log2(base)
}

fn log2_approx(x: f32) -> f32 {
    let exponent = (x.to_bits() >> 23) as i32 - 0x7f;
    let x = f32::from_bits((x.to_bits() & 0x7fffff) | 0x3f800000) - 0.96;
    let y: f32 = x;
    y + (exponent as f32)
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

fn tanh(x: f32) -> f32 {
    let exp2x = exp2(x * (std::f32::consts::LOG2_E * 2.0));
    (exp2x - 1.0) / (exp2x + 1.0)
}

fn asinh(x: f32) -> f32 {
    ln(x + (x * x + 1.0).sqrt())
}

fn acosh(x: f32) -> f32 {
    ln(x + (x * x - 1.0).sqrt())
}

fn atanh(x: f32) -> f32 {
    (ln(1.0 + x) - ln(1.0 - x)) * 0.5
}

fn sin_cos(x: f32) -> (f32, f32) {
    (sin(x), cos(x))
}

fn atan2(y: f32, x: f32) -> f32 {
    use std::f32::consts::PI;
    let offset180 = if y < 0.0 { -PI } else { PI };
    let (x, y, offset) = if x < 0.0 {
        (-x, -y, offset180)
    } else {
        (x, y, 0.0)
    };
    let offset90 = if y < 0.0 { -PI / 2.0 } else { PI / 2.0 };
    let (x, y, offset) = if y.abs() > x {
        (y, -x, offset + offset90)
    } else {
        (x, y, offset)
    };
    let x = y / x;
    let y = (-0.003960257232989522_f32)
        .mul_add(x * x, 0.02165913793056782_f32)
        .mul_add(x * x, -0.055874572113835966_f32)
        .mul_add(x * x, 0.09664150837900513_f32)
        .mul_add(x * x, -0.13930208322722726_f32)
        .mul_add(x * x, 0.1995446828536437_f32)
        .mul_add(x * x, -0.33331004869942993_f32)
        .mul_add(x * x, 0.9999997955077145_f32)
        * x;
    y + offset
}

fn sqrt(x: f32) -> f32 {
    let r = exp2(log2(x) * (1.0 / 2.0));
    let y = r + (x - r * r) / (2.0 * r);
    y
}

fn cbrt(x: f32) -> f32 {
    let r = exp2(log2(x.abs()) * (1.0 / 3.0));
    let y = r + (x.abs() - r * r * r) / (3.0 * r * r);
    if x < 0.0 {
        -y
    } else {
        y
    }
}

fn hypot(x: f32, y: f32) -> f32 {
    let (x, y) = if x.abs() > y.abs() { (x, y) } else { (y, x) };
    if x.abs() <= f32::MIN_POSITIVE {
        x
    } else {
        x.abs() * (1.0 + (y / x) * (y / x)).sqrt()
    }
}

fn recip(x: f32) -> f32 {
    let r = recip_approx(x);
    let r = r * (2.0 - x * r);
    let r = r * (2.0 - x * r);
    let r = r * (2.0 - x * r);
    r
}

fn recip_approx(x: f32) -> f32 {
    let y = f32::from_bits((0x3f800000 as f32 * 2.0 - (x.abs().to_bits() as f32)) as u32) - 0.08;
    if x < 0.0 {
        -y
    } else {
        y
    }
}

fn powf(x: f32, y: f32) -> f32 {
    exp2(log2(x) * y)
}

fn powi(x: f32, y: i32) -> f32 {
    let a = x;
    let p = y.abs();
    let b = if (p & (1 << 0)) != 0 { a } else { 1.0 };
    let a = a * a;
    let b = if (p & (1 << 1)) != 0 { b * a } else { b };
    let a = a * a;
    let b = if (p & (1 << 2)) != 0 { b * a } else { b };
    let a = a * a;
    let b = if (p & (1 << 3)) != 0 { b * a } else { b };
    let b = if p < 16 { b } else { powf(x, p as f32) };
    if y < 0 {
        recip(b)
    } else {
        b
    }
}

#[test]
fn test_sin() {
    const N: i32 = 0x100000;
    let tmin = -3.141592653589793f64;
    let tmax = 3.141592653589793f64;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.sin();
        let y2 = sin(x as f32) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16} y2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("me={:20}", max_error);
    assert!(!max_error.is_nan());
    assert!(max_error < 0.0000007152557373046875f64);
}

#[test]
fn test_cos() {
    const N: i32 = 0x100000;
    let tmin = -3.141592653589793f64;
    let tmax = 3.141592653589793f64;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.cos();
        let y2 = cos(x as f32) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16} y2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("me={:20}", max_error);
    assert!(!max_error.is_nan());
    assert!(max_error < 0.00000035762786865234375f64);
}

#[test]
fn test_tan_a() {
    const N: i32 = 0x100000;
    let tmin = -0.7853981633974483f64;
    let tmax = 0.7853981633974483f64;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.tan();
        let y2 = tan(x as f32) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16} y2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("me={:20}", max_error);
    assert!(!max_error.is_nan());
    assert!(max_error < 0.0000002384185791015625f64);
}

#[test]
fn test_tan_b() {
    const N: i32 = 0x100000;
    let tmin = -1.0471975511965976f64;
    let tmax = 1.0471975511965976f64;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.tan();
        let y2 = tan(x as f32) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16} y2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("me={:20}", max_error);
    assert!(!max_error.is_nan());
    assert!(max_error < 0.0000008344650268554688f64);
}

#[test]
fn test_asin() {
    const N: i32 = 0x100000;
    let tmin = -0.999f64;
    let tmax = 0.999f64;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.asin();
        let y2 = asin(x as f32) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16} y2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("me={:20}", max_error);
    assert!(!max_error.is_nan());
    assert!(max_error < 0.0000010728836059570313f64);
}

#[test]
fn test_acos() {
    const N: i32 = 0x100000;
    let tmin = -0.999f64;
    let tmax = 0.999f64;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.acos();
        let y2 = acos(x as f32) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16} y2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("me={:20}", max_error);
    assert!(!max_error.is_nan());
    assert!(max_error < 0.0000010728836059570313f64);
}

#[test]
fn test_atan() {
    const N: i32 = 0x100000;
    let tmin = -2f64;
    let tmax = 2f64;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.atan();
        let y2 = atan(x as f32) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16} y2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("me={:20}", max_error);
    assert!(!max_error.is_nan());
    assert!(max_error < 0.0000002384185791015625f64);
}

#[test]
fn test_exp_a() {
    const N: i32 = 0x100000;
    let tmin = 0f64;
    let tmax = 1f64;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.exp();
        let y2 = exp(x as f32) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16} y2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("me={:20}", max_error);
    assert!(!max_error.is_nan());
    assert!(max_error < 0.00000035762786865234375f64);
}

#[test]
fn test_exp_b() {
    const N: i32 = 0x100000;
    let tmin = 1f64;
    let tmax = 2f64;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.exp();
        let y2 = exp(x as f32) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16} y2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("me={:20}", max_error);
    assert!(!max_error.is_nan());
    assert!(max_error < 0.0000011920928955078125f64);
}

#[test]
fn test_exp_m1() {
    const N: i32 = 0x100000;
    let tmin = 0f64;
    let tmax = 1f64;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.exp_m1();
        let y2 = exp_m1(x as f32) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16} y2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("me={:20}", max_error);
    assert!(!max_error.is_nan());
    assert!(max_error < 0.00000035762786865234375f64);
}

#[test]
fn test_exp2() {
    const N: i32 = 0x100000;
    let tmin = 0f64;
    let tmax = 1f64;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.exp2();
        let y2 = exp2(x as f32) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16} y2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("me={:20}", max_error);
    assert!(!max_error.is_nan());
    assert!(max_error < 0.0000002384185791015625f64);
}

#[test]
fn test_exp2_x() {
    const N: i32 = 0x100000;
    let tmin = 0f64;
    let tmax = 1f64;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.exp2();
        let y2 = exp2_approx(x as f32) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16} y2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("me={:20}", max_error);
    assert!(!max_error.is_nan());
    assert!(max_error < 0.05f64);
}

#[test]
fn test_ln() {
    const N: i32 = 0x100000;
    let tmin = 1f64;
    let tmax = 2.718281828459045f64;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.ln();
        let y2 = ln(x as f32) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16} y2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("me={:20}", max_error);
    assert!(!max_error.is_nan());
    assert!(max_error < 0.0000002384185791015625f64);
}

#[test]
fn test_ln_1p_a() {
    const N: i32 = 0x100000;
    let tmin = 0f64;
    let tmax = 1f64;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.ln_1p();
        let y2 = ln_1p(x as f32) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16} y2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("me={:20}", max_error);
    assert!(!max_error.is_nan());
    assert!(max_error < 0.0000002384185791015625f64);
}

#[test]
fn test_ln_1p_b() {
    const N: i32 = 0x100000;
    let tmin = 1f64;
    let tmax = 7.154845485377136f64;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.ln_1p();
        let y2 = ln_1p(x as f32) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16} y2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("me={:20}", max_error);
    assert!(!max_error.is_nan());
    assert!(max_error < 0.00000035762786865234375f64);
}

#[test]
fn test_log2() {
    const N: i32 = 0x100000;
    let tmin = 0.25f64;
    let tmax = 4.25f64;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.log2();
        let y2 = log2(x as f32) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16} y2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("me={:20}", max_error);
    assert!(!max_error.is_nan());
    assert!(max_error < 0.0000002384185791015625f64);
}

#[test]
fn test_log10() {
    const N: i32 = 0x100000;
    let tmin = 0.1f64;
    let tmax = 10.1f64;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.log10();
        let y2 = log10(x as f32) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16} y2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("me={:20}", max_error);
    assert!(!max_error.is_nan());
    assert!(max_error < 0.0000002384185791015625f64);
}

#[test]
fn test_log_2() {
    const N: i32 = 0x100000;
    let tmin = 0.5f64;
    let tmax = 1.5f64;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.log(2.0);
        let y2 = log(x as f32, 2.0) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16} y2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("me={:20}", max_error);
    assert!(!max_error.is_nan());
    assert!(max_error < 0.0000002384185791015625f64);
}

#[test]
fn test_log_e() {
    const N: i32 = 0x100000;
    let tmin = 0.5f64;
    let tmax = 1.5f64;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.log(std::f64::consts::E);
        let y2 = log(x as f32, std::f32::consts::E) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16} y2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("me={:20}", max_error);
    assert!(!max_error.is_nan());
    assert!(max_error < 0.0000002384185791015625f64);
}

#[test]
fn test_log_2x() {
    const N: i32 = 0x100000;
    let tmin = 0.5f64;
    let tmax = 10.5f64;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.log2();
        let y2 = log2_approx(x as f32) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16} y2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("me={:20}", max_error);
    assert!(!max_error.is_nan());
    assert!(max_error < 0.05f64);
}

#[test]
fn test_cosh() {
    const N: i32 = 0x100000;
    let tmin = -1f64;
    let tmax = 1f64;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.cosh();
        let y2 = cosh(x as f32) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16} y2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("me={:20}", max_error);
    assert!(!max_error.is_nan());
    assert!(max_error < 0.0000002384185791015625f64);
}

#[test]
fn test_sinh() {
    const N: i32 = 0x100000;
    let tmin = -1f64;
    let tmax = 1f64;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.sinh();
        let y2 = sinh(x as f32) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16} y2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("me={:20}", max_error);
    assert!(!max_error.is_nan());
    assert!(max_error < 0.0000002384185791015625f64);
}

#[test]
fn test_tanh() {
    const N: i32 = 0x100000;
    let tmin = -1f64;
    let tmax = 1f64;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.tanh();
        let y2 = tanh(x as f32) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16} y2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("me={:20}", max_error);
    assert!(!max_error.is_nan());
    assert!(max_error < 0.0000002384185791015625f64);
}

#[test]
fn test_acosh() {
    const N: i32 = 0x100000;
    let tmin = -1f64;
    let tmax = 1f64;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.acosh();
        let y2 = acosh(x as f32) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16} y2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("me={:20}", max_error);
    assert!(!max_error.is_nan());
    assert!(max_error < 0.0000002384185791015625f64);
}

#[test]
fn test_asinh() {
    const N: i32 = 0x100000;
    let tmin = -1f64;
    let tmax = 1f64;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.asinh();
        let y2 = asinh(x as f32) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16} y2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("me={:20}", max_error);
    assert!(!max_error.is_nan());
    assert!(max_error < 0.00000035762786865234375f64);
}

#[test]
fn test_atanh() {
    const N: i32 = 0x100000;
    let tmin = -0.9f64;
    let tmax = 0.9f64;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.atanh();
        let y2 = atanh(x as f32) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16} y2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("me={:20}", max_error);
    assert!(!max_error.is_nan());
    assert!(max_error < 0.00000035762786865234375f64);
}

#[test]
fn test_sin_cos_s() {
    const N: i32 = 0x100000;
    let tmin = -3.141592653589793f64;
    let tmax = 3.141592653589793f64;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.sin_cos().0;
        let y2 = sin_cos(x as f32).0 as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16} y2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("me={:20}", max_error);
    assert!(!max_error.is_nan());
    assert!(max_error < 0.0000007152557373046875f64);
}

#[test]
fn test_sin_cos_c() {
    const N: i32 = 0x100000;
    let tmin = -3.141592653589793f64;
    let tmax = 3.141592653589793f64;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.sin_cos().1;
        let y2 = sin_cos(x as f32).1 as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16} y2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("me={:20}", max_error);
    assert!(!max_error.is_nan());
    assert!(max_error < 0.00000035762786865234375f64);
}

#[test]
fn test_atan2_a() {
    const N: i32 = 0x100000;
    let tmin = -1f64;
    let tmax = 1f64;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.atan2(1.0);
        let y2 = atan2(x as f32, 1.0) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16} y2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("me={:20}", max_error);
    assert!(!max_error.is_nan());
    assert!(max_error < 0.00000035762786865234375f64);
}

#[test]
fn test_atan2_b() {
    const N: i32 = 0x100000;
    let tmin = -1f64;
    let tmax = 1f64;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.atan2(-1.0);
        let y2 = atan2(x as f32, -1.0) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16} y2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("me={:20}", max_error);
    assert!(!max_error.is_nan());
    assert!(max_error < 0.00000035762786865234375f64);
}

#[test]
fn test_atan2_c() {
    const N: i32 = 0x100000;
    let tmin = -1f64;
    let tmax = 1f64;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = (1.0_f64).atan2(x);
        let y2 = atan2(1.0, x as f32) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16} y2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("me={:20}", max_error);
    assert!(!max_error.is_nan());
    assert!(max_error < 0.00000035762786865234375f64);
}

#[test]
fn test_atan2_d() {
    const N: i32 = 0x100000;
    let tmin = -1f64;
    let tmax = 1f64;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = (-1.0_f64).atan2(x);
        let y2 = atan2(-1.0, x as f32) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16} y2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("me={:20}", max_error);
    assert!(!max_error.is_nan());
    assert!(max_error < 0.00000035762786865234375f64);
}

#[test]
fn test_hypot_a() {
    const N: i32 = 0x100000;
    let tmin = -1f64;
    let tmax = 1f64;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.hypot(1.0);
        let y2 = hypot(x as f32, 1.0) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16} y2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("me={:20}", max_error);
    assert!(!max_error.is_nan());
    assert!(max_error < 0.00000035762786865234375f64);
}

#[test]
fn test_hypot_b() {
    const N: i32 = 0x100000;
    let tmin = -1f64;
    let tmax = 1f64;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.hypot(-1.0);
        let y2 = hypot(x as f32, -1.0) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16} y2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("me={:20}", max_error);
    assert!(!max_error.is_nan());
    assert!(max_error < 0.00000035762786865234375f64);
}

#[test]
fn test_hypot_c() {
    const N: i32 = 0x100000;
    let tmin = -1f64;
    let tmax = 1f64;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = (1.0_f64).hypot(x);
        let y2 = hypot(1.0, x as f32) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16} y2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("me={:20}", max_error);
    assert!(!max_error.is_nan());
    assert!(max_error < 0.00000035762786865234375f64);
}

#[test]
fn test_hypot_d() {
    const N: i32 = 0x100000;
    let tmin = -1f64;
    let tmax = 1f64;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = (-1.0_f64).hypot(x);
        let y2 = hypot(-1.0, x as f32) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16} y2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("me={:20}", max_error);
    assert!(!max_error.is_nan());
    assert!(max_error < 0.00000035762786865234375f64);
}

#[test]
fn test_sqrt() {
    const N: i32 = 0x100000;
    let tmin = 0.5f64;
    let tmax = 2f64;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.sqrt();
        let y2 = sqrt(x as f32) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16} y2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("me={:20}", max_error);
    assert!(!max_error.is_nan());
    assert!(max_error < 0.00000011920928955078125f64);
}

#[test]
fn test_cbrt() {
    const N: i32 = 0x100000;
    let tmin = -2f64;
    let tmax = 2f64;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.cbrt();
        let y2 = cbrt(x as f32) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16} y2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("me={:20}", max_error);
    assert!(!max_error.is_nan());
    assert!(max_error < 0.00000011920928955078125f64);
}

#[test]
fn test_recip() {
    const N: i32 = 0x100000;
    let tmin = 0.5f64;
    let tmax = 1.5f64;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.recip();
        let y2 = recip(x as f32) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16} y2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("me={:20}", max_error);
    assert!(!max_error.is_nan());
    assert!(max_error < 0.0000002384185791015625f64);
}

#[test]
fn test_recip_n() {
    const N: i32 = 0x100000;
    let tmin = -1.5f64;
    let tmax = -0.5f64;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.recip();
        let y2 = recip(x as f32) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16} y2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("me={:20}", max_error);
    assert!(!max_error.is_nan());
    assert!(max_error < 0.0000002384185791015625f64);
}

#[test]
fn test_recip_x() {
    const N: i32 = 0x100000;
    let tmin = 0.5f64;
    let tmax = 1.5f64;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.recip();
        let y2 = recip_approx(x as f32) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16} y2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("me={:20}", max_error);
    assert!(!max_error.is_nan());
    assert!(max_error < 0.1f64);
}

#[test]
fn test_recip_y() {
    const N: i32 = 0x100000;
    let tmin = -1.5f64;
    let tmax = -0.5f64;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.recip();
        let y2 = recip_approx(x as f32) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16} y2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("me={:20}", max_error);
    assert!(!max_error.is_nan());
    assert!(max_error < 0.1f64);
}

#[test]
fn test_powf_2() {
    const N: i32 = 0x100000;
    let tmin = 0.5f64;
    let tmax = 1.5f64;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.powf(2.0);
        let y2 = powf(x as f32, 2.0) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16} y2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("me={:20}", max_error);
    assert!(!max_error.is_nan());
    assert!(max_error < 0.000000476837158203125f64);
}

#[test]
fn test_powf_m1() {
    const N: i32 = 0x100000;
    let tmin = 0.5f64;
    let tmax = 1.5f64;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.powf(-1.0);
        let y2 = powf(x as f32, -1.0) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16} y2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("me={:20}", max_error);
    assert!(!max_error.is_nan());
    assert!(max_error < 0.000000476837158203125f64);
}

#[test]
fn test_powi_2() {
    const N: i32 = 0x100000;
    let tmin = 0.5f64;
    let tmax = 1.5f64;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.powi(2);
        let y2 = powi(x as f32, 2) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16} y2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("me={:20}", max_error);
    assert!(!max_error.is_nan());
    assert!(max_error < 0.0000002384185791015625f64);
}

#[test]
fn test_powi_3() {
    const N: i32 = 0x100000;
    let tmin = 0.12f64;
    let tmax = 1.2f64;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.powi(3);
        let y2 = powi(x as f32, 3) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16} y2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("me={:20}", max_error);
    assert!(!max_error.is_nan());
    assert!(max_error < 0.000000476837158203125f64);
}

#[test]
fn test_powi_m1() {
    const N: i32 = 0x100000;
    let tmin = 0.5f64;
    let tmax = 1.5f64;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.powi(-1);
        let y2 = powi(x as f32, -1) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16} y2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("me={:20}", max_error);
    assert!(!max_error.is_nan());
    assert!(max_error < 0.0000002384185791015625f64);
}

#[test]
fn test_powi_m2() {
    const N: i32 = 0x100000;
    let tmin = 0.5f64;
    let tmax = 1.5f64;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.powi(-2);
        let y2 = powi(x as f32, -2) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16} y2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("me={:20}", max_error);
    assert!(!max_error.is_nan());
    assert!(max_error < 0.0000007152557373046875f64);
}

#[test]
fn test_powi_16() {
    const N: i32 = 0x100000;
    let tmin = 0.25f64;
    let tmax = 1f64;
    let mut max_error = 0.0_f64;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.powi(16);
        let y2 = powi(x as f32, 16) as f64;
        max_error = max_error.max((y1 - y2).abs());
        if i % (N / 16) == 0 {
            println!("y1={:20.16} y2={:20.16} e={:20.16}", y1, y2, y2 - y1);
        }
    }
    println!("me={:20}", max_error);
    assert!(!max_error.is_nan());
    assert!(max_error < 0.0000008344650268554688f64);
}
