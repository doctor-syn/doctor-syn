fn sin(x: f64) -> f64 {
    let x = x * (1.0 / (std::f64::consts::PI));
    let xh = x + 0.5;
    let xr = x.round();
    let xhr = xh.round();
    let s = x - xr;
    let c = xh - xhr;
    let sr = (-0.007276272925775644_f64)
        .mul_add(s * s, 0.082138657661446141933242_f64)
        .mul_add(s * s, -0.5992642701204297_f64)
        .mul_add(s * s, 2.550164035609007384159119_f64)
        .mul_add(s * s, -5.16771278002373_f64)
        .mul_add(s * s, 3.141592653589766353998613_f64)
        * s;
    let cr = (-0.001906810359460933_f64)
        .mul_add(c * c, 0.025804963990871485402163_f64)
        .mul_add(c * c, -0.23533055117543247_f64)
        .mul_add(c * c, 1.335262767253115701736531_f64)
        .mul_add(c * c, -4.058712126402666_f64)
        .mul_add(c * c, 4.934802200544643092309145_f64)
        .mul_add(c * c, -1.0_f64);
    let ss = if (xr as i32) & 1 == 0 { sr } else { -sr };
    let cs = if (xhr as i32 & 1) == 0 { cr } else { -cr };
    if s.abs() <= 0.25 {
        ss
    } else {
        cs
    }
}
fn cos(x: f64) -> f64 {
    let x = x * (1.0 / (std::f64::consts::PI));
    let xh = x + 0.5;
    let xr = x.round();
    let xhr = xh.round();
    let c = x - xr;
    let s = xh - xhr;
    let sr = (-0.007276272925775644_f64)
        .mul_add(s * s, 0.082138657661446141933242_f64)
        .mul_add(s * s, -0.5992642701204297_f64)
        .mul_add(s * s, 2.550164035609007384159119_f64)
        .mul_add(s * s, -5.16771278002373_f64)
        .mul_add(s * s, 3.141592653589766353998613_f64)
        * s;
    let cr = (0.001906810359460932823652_f64)
        .mul_add(c * c, -0.025804963990871486_f64)
        .mul_add(c * c, 0.235330551175432468378890_f64)
        .mul_add(c * c, -1.3352627672531157_f64)
        .mul_add(c * c, 4.058712126402666198495983_f64)
        .mul_add(c * c, -4.9348022005446435_f64)
        .mul_add(c * c, 1.000000000000000000000000_f64);
    let ss = if xhr as i32 & 1 == 0 { sr } else { -sr };
    let cs = if xr as i32 & 1 == 0 { cr } else { -cr };
    if s.abs() <= 0.25 {
        ss
    } else {
        cs
    }
}
fn tan(x: f64) -> f64 {
    let x = x * (1.0 / (std::f64::consts::PI));
    let x = x - x.round();
    let recip = 1.0 / (x * x - 0.25);
    let y = (0.003179012998420303318776_f64)
        .mul_add(x * x, 0.003783064688471169223152_f64)
        .mul_add(x * x, 0.010019701763828131711280_f64)
        .mul_add(x * x, 0.022494759523670229158284_f64)
        .mul_add(x * x, 0.052637925525689075092777_f64)
        .mul_add(x * x, 0.134769356070922490389309_f64)
        .mul_add(x * x, 0.557736264158740053162619_f64)
        .mul_add(x * x, -0.7853981633987636_f64)
        * x;
    y * recip
}
fn sin_cos(x: f64) -> (f64, f64) {
    (sin(x), cos(x))
}
fn asin(x: f64) -> f64 {
    use std::f64::consts::PI;
    const LIM: f64 = 0.9;
    let c = if x < 0.0 { -PI / 2.0 } else { PI / 2.0 };
    let s = if x < 0.0 { -1.0 } else { 1.0 };
    let x0 = x;
    let x = if x * x < LIM * LIM {
        x
    } else {
        (1.0 - x * x).sqrt()
    };
    let y = (5.974209066035325206514890_f64)
        .mul_add(x * x, -21.400628208800804_f64)
        .mul_add(x * x, 33.119120053667824208900243_f64)
        .mul_add(x * x, -28.55304827959498_f64)
        .mul_add(x * x, 14.987184720975770081421851_f64)
        .mul_add(x * x, -4.84763763396207_f64)
        .mul_add(x * x, 0.996414555544273345740989_f64)
        .mul_add(x * x, -0.06545333453273161_f64)
        .mul_add(x * x, 0.081365649244162800417868_f64)
        .mul_add(x * x, 0.166524233201920323827779_f64)
        .mul_add(x * x, 1.000000523840070413298558_f64)
        * x;
    if x0 * x0 < LIM * LIM {
        y
    } else {
        c - y * s
    }
}
fn acos(x: f64) -> f64 {
    use std::f64::consts::PI;
    const LIM: f64 = 0.9;
    let c = if x < 0.0 { PI } else { 0.0 };
    let s = if x < 0.0 { 1.0 } else { -1.0 };
    let x0 = x;
    let x = if x * x < LIM * LIM {
        x
    } else {
        (1.0 - x * x).sqrt()
    };
    let y = (5.974209066035325206514890_f64)
        .mul_add(x * x, -21.400628208800804_f64)
        .mul_add(x * x, 33.119120053667824208900243_f64)
        .mul_add(x * x, -28.55304827959498_f64)
        .mul_add(x * x, 14.987184720975770081421851_f64)
        .mul_add(x * x, -4.84763763396207_f64)
        .mul_add(x * x, 0.996414555544273345740989_f64)
        .mul_add(x * x, -0.06545333453273161_f64)
        .mul_add(x * x, 0.081365649244162800417868_f64)
        .mul_add(x * x, 0.166524233201920323827779_f64)
        .mul_add(x * x, 1.000000523840070413298558_f64)
        * x;
    if x0 * x0 < LIM * LIM {
        PI / 2.0 - y
    } else {
        c - y * s
    }
}
fn atan(x: f64) -> f64 {
    use std::f64::consts::PI;
    const LIM: f64 = 1.0;
    let c = if x < 0.0 { -PI / 2.0 } else { PI / 2.0 };
    let small = x.abs() < LIM;
    let x = if small { x } else { x.recip() };
    let y = (-3120.2670506577906_f64)
        .mul_add(x * x, 18527.875938866914811172096692_f64)
        .mul_add(x * x, -47529.629874571656_f64)
        .mul_add(x * x, 68707.050859427355801413325263_f64)
        .mul_add(x * x, -61112.06307839758_f64)
        .mul_add(x * x, 34182.620421021387932469297209_f64)
        .mul_add(x * x, -11727.252675330543_f64)
        .mul_add(x * x, 2270.363451249629032853114175_f64)
        .mul_add(x * x, -204.98167697459724_f64)
        .mul_add(x * x, 6.098990506304593876323005_f64)
        .mul_add(x * x, 0.970093023972372205888058_f64)
        * x;
    if small {
        y
    } else {
        c - y
    }
}
fn atan2(y: f64, x: f64) -> f64 {
    use std::f64::consts::PI;
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
    let y = (-95.7012638384253_f64)
        .mul_add(x * x, 424.999070228060595384359263_f64)
        .mul_add(x * x, -767.4825968004056_f64)
        .mul_add(x * x, 714.519530122242234058908346_f64)
        .mul_add(x * x, -354.32654395426965_f64)
        .mul_add(x * x, 83.961798971485391843983733_f64)
        .mul_add(x * x, -6.239581707154415_f64)
        .mul_add(x * x, 1.054985141864275241700106_f64)
        * x;
    y + offset
}
fn exp(x: f64) -> f64 {
    exp2(x * std::f64::consts::LOG2_E)
}
fn exp2(x: f64) -> f64 {
    let r = x.round();
    let mul = f64::from_bits(
        (r.mul_add(0x0010000000000000_u64 as f64, 0x3ff0000000000000_u64 as f64)) as u64,
    );
    let x = x - r;
    (0.000154697319774755481135_f64)
        .mul_add(x, 0.001341000096610015706912_f64)
        .mul_add(x, 0.009618030782505153797811_f64)
        .mul_add(x, 0.055502973141993247574765_f64)
        .mul_add(x, 0.240226510841172935738043_f64)
        .mul_add(x, 0.693147225395011086525471_f64)
        .mul_add(x, 1.000000000000000000000000_f64)
        * mul
}
fn exp_m1(x: f64) -> f64 {
    let x = x * std::f64::consts::LOG2_E;
    let r = x.round();
    let mul = f64::from_bits(
        (r.mul_add(0x0010000000000000_u64 as f64, 0x3ff0000000000000_u64 as f64)) as u64,
    );
    let x = x - r;
    (0.000154697319774755481135_f64)
        .mul_add(x, 0.001341000096610015706912_f64)
        .mul_add(x, 0.009618030782505153797811_f64)
        .mul_add(x, 0.055502973141993247574765_f64)
        .mul_add(x, 0.240226510841172935738043_f64)
        .mul_add(x, 0.693147225395011086525471_f64)
        .mul_add(x, 0.000000000000000000000000_f64)
        * mul
        + (mul - 1.0)
}
fn ln(x: f64) -> f64 {
    log2(x) * (1.0 / std::f64::consts::LOG2_E)
}
fn ln_1p(x: f64) -> f64 {
    let exponent = ((x + 1.0).to_bits() >> 23) as i32 - 0x7f;
    let x = if exponent == 0 {
        x
    } else {
        f64::from_bits(((x + 1.0).to_bits() & 0x7fffff) | 0x3f800000) - 1.0
    };
    let y: f64 = (-0.008874696650310257_f64)
        .mul_add(x, 0.050615875638515576401052_f64)
        .mul_add(x, -0.13573160547743965_f64)
        .mul_add(x, 0.240899756626915140830490_f64)
        .mul_add(x, -0.3471516621024704_f64)
        .mul_add(x, 0.478736153814679583206005_f64)
        .mul_add(x, -0.7211851619881156_f64)
        .mul_add(x, 1.442691340138225593039231_f64)
        .mul_add(x, 0.000000000000000000000000_f64);
    (y + (exponent as f64)) * (1.0 / std::f64::consts::LOG2_E)
}
fn log2(x: f64) -> f64 {
    let exponent = (x.to_bits() >> 23) as i32 - 0x7f;
    let x =
        f64::from_bits((x.to_bits() & (0x0010000000000000_u64 - 1)) | 0x3ff0000000000000_u64) - 1.5;
    let y: f64 = (-0.008874696650310257_f64)
        .mul_add(x, 0.015117089037274548008148_f64)
        .mul_add(x, -0.020698917294806933_f64)
        .mul_add(x, 0.037315410744631165047217_f64)
        .mul_add(x, -0.07127813300218297_f64)
        .mul_add(x, 0.142544716304637367636681_f64)
        .mul_add(x, -0.32059812016799244_f64)
        .mul_add(x, 0.961795403236093795462751_f64)
        .mul_add(x, 0.584962500721156181453739_f64);
    y + (exponent as f64)
}
fn log10(x: f64) -> f64 {
    log2(x) * (1.0 / std::f64::consts::LOG2_10)
}
fn log(x: f64, base: f64) -> f64 {
    log2(x) / log2(base)
}
fn powi(x: f64, y: i32) -> f64 {
    let a = x;
    let p = y.abs();
    let b = if (p & (1 << 0)) != 0 { a } else { 1.0 };
    let a = a * a;
    let b = if (p & (1 << 1)) != 0 { b * a } else { b };
    let a = a * a;
    let b = if (p & (1 << 2)) != 0 { b * a } else { b };
    let a = a * a;
    let b = if (p & (1 << 3)) != 0 { b * a } else { b };
    let b = if p < 16 { b } else { powf(x, p as f64) };
    if y < 0 {
        recip(b)
    } else {
        b
    }
}
fn powf(x: f64, y: f64) -> f64 {
    exp2(log2(x) * y)
}
fn sinh(x: f64) -> f64 {
    let a = x.mul_add(std::f64::consts::LOG2_E, -1.0);
    let b = x.mul_add(-std::f64::consts::LOG2_E, -1.0);
    exp2(a) - exp2(b)
}
fn cosh(x: f64) -> f64 {
    let a = x.mul_add(std::f64::consts::LOG2_E, -1.0);
    let b = x.mul_add(-std::f64::consts::LOG2_E, -1.0);
    exp2(a) + exp2(b)
}
fn tanh(x: f64) -> f64 {
    let exp2x = exp2(x * (std::f64::consts::LOG2_E * 2.0));
    (exp2x - 1.0) / (exp2x + 1.0)
}
fn asinh(x: f64) -> f64 {
    ln(x + (x * x + 1.0).sqrt())
}
fn acosh(x: f64) -> f64 {
    ln(x + (x * x - 1.0).sqrt())
}
fn atanh(x: f64) -> f64 {
    (ln(1.0 + x) - ln(1.0 - x)) * 0.5
}
fn sqrt(x: f64) -> f64 {
    let r = sqrt_approx(x);
    let y = r + (x - r * r) / (2.0 * r);
    y
}
fn cbrt(x: f64) -> f64 {
    let r = cbrt_approx(x.abs());
    let y = r + (x.abs() - r * r * r) / (3.0 * r * r);
    y.copysign(x)
}
fn hypot(x: f64, y: f64) -> f64 {
    let (x, y) = if x.abs() > y.abs() { (x, y) } else { (y, x) };
    if x.abs() <= f64::MIN_POSITIVE {
        x
    } else {
        x.abs() * (1.0 + (y / x) * (y / x)).sqrt()
    }
}
fn recip(x: f64) -> f64 {
    let r = recip_approx(x);
    let r = r * (2.0 - x * r);
    let r = r * (2.0 - x * r);
    let r = r * (2.0 - x * r);
    r
}
fn recip_approx(x: f64) -> f64 {
    let y = f64::from_bits(
        ((x.abs().to_bits() as f64).mul_add(-1.0, 0x3ff0000000000000_u64 as f64 * 2.0)) as u64,
    );
    (y - 0.08).copysign(x)
}
fn sqrt_approx(x: f64) -> f64 {
    let y = f64::from_bits(
        ((x.abs().to_bits() as f64).mul_add(0.5, 0x3ff0000000000000_u64 as f64 * 0.5)) as u64,
    );
    y - 0.08
}
fn cbrt_approx(x: f64) -> f64 {
    let y = f64::from_bits(
        ((x.abs().to_bits() as f64).mul_add(1.0 / 3.0, 0x3ff0000000000000_u64 as f64 * 2.0 / 3.0))
            as u64,
    );
    (y - 0.08).copysign(x)
}
#[test]
fn test_sin() {
    const N: i32 = 0x100000;
    let tmin = -3.141592653589793f64;
    let tmax = 3.141592653589793f64;
    let mut max_error = 0.0_f64;
    let mut xmax = tmin;
    let mut y1max = 0.0;
    let mut y2max = 0.0;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.sin();
        let y2 = sin(x as f64) as f64;
        let error = (y1 - y2).abs();
        if error > max_error {
            max_error = error;
            xmax = x;
            y1max = y1;
            y2max = y2;
        }
        if i % (N / 16) == 0 {
            println!(
                " x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
                x,
                y1,
                y2,
                y2 - y1
            );
        }
    }
    println!(
        "!x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
        xmax, y1max, y2max, max_error
    );
    assert!(!max_error.is_nan());
    assert!(max_error < 0.0000000000000004440892098500626f64);
}
#[test]
fn test_cos() {
    const N: i32 = 0x100000;
    let tmin = -3.141592653589793f64;
    let tmax = 3.141592653589793f64;
    let mut max_error = 0.0_f64;
    let mut xmax = tmin;
    let mut y1max = 0.0;
    let mut y2max = 0.0;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.cos();
        let y2 = cos(x as f64) as f64;
        let error = (y1 - y2).abs();
        if error > max_error {
            max_error = error;
            xmax = x;
            y1max = y1;
            y2max = y2;
        }
        if i % (N / 16) == 0 {
            println!(
                " x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
                x,
                y1,
                y2,
                y2 - y1
            );
        }
    }
    println!(
        "!x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
        xmax, y1max, y2max, max_error
    );
    assert!(!max_error.is_nan());
    assert!(max_error < 0.0000000000000004440892098500626f64);
}
#[test]
fn test_tan() {
    const N: i32 = 0x100000;
    let tmin = -0.7853981633974483f64;
    let tmax = 0.7853981633974483f64;
    let mut max_error = 0.0_f64;
    let mut xmax = tmin;
    let mut y1max = 0.0;
    let mut y2max = 0.0;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.tan();
        let y2 = tan(x as f64) as f64;
        let error = (y1 - y2).abs();
        if error > max_error {
            max_error = error;
            xmax = x;
            y1max = y1;
            y2max = y2;
        }
        if i % (N / 16) == 0 {
            println!(
                " x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
                x,
                y1,
                y2,
                y2 - y1
            );
        }
    }
    println!(
        "!x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
        xmax, y1max, y2max, max_error
    );
    assert!(!max_error.is_nan());
    assert!(max_error < 0.0000000000000006661338147750939f64);
}
#[test]
fn test_sin_cos_1() {
    const N: i32 = 0x100000;
    let tmin = -3.141592653589793f64;
    let tmax = 3.141592653589793f64;
    let mut max_error = 0.0_f64;
    let mut xmax = tmin;
    let mut y1max = 0.0;
    let mut y2max = 0.0;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.sin();
        let y2 = sin_cos(x as f64).0 as f64;
        let error = (y1 - y2).abs();
        if error > max_error {
            max_error = error;
            xmax = x;
            y1max = y1;
            y2max = y2;
        }
        if i % (N / 16) == 0 {
            println!(
                " x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
                x,
                y1,
                y2,
                y2 - y1
            );
        }
    }
    println!(
        "!x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
        xmax, y1max, y2max, max_error
    );
    assert!(!max_error.is_nan());
    assert!(max_error < 0.0000000000000004440892098500626f64);
}
#[test]
fn test_sin_cos_2() {
    const N: i32 = 0x100000;
    let tmin = -3.141592653589793f64;
    let tmax = 3.141592653589793f64;
    let mut max_error = 0.0_f64;
    let mut xmax = tmin;
    let mut y1max = 0.0;
    let mut y2max = 0.0;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.cos();
        let y2 = sin_cos(x as f64).1 as f64;
        let error = (y1 - y2).abs();
        if error > max_error {
            max_error = error;
            xmax = x;
            y1max = y1;
            y2max = y2;
        }
        if i % (N / 16) == 0 {
            println!(
                " x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
                x,
                y1,
                y2,
                y2 - y1
            );
        }
    }
    println!(
        "!x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
        xmax, y1max, y2max, max_error
    );
    assert!(!max_error.is_nan());
    assert!(max_error < 0.0000000000000004440892098500626f64);
}
#[test]
fn test_asin() {
    const N: i32 = 0x100000;
    let tmin = -0.999f64;
    let tmax = 0.999f64;
    let mut max_error = 0.0_f64;
    let mut xmax = tmin;
    let mut y1max = 0.0;
    let mut y2max = 0.0;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.asin();
        let y2 = asin(x as f64) as f64;
        let error = (y1 - y2).abs();
        if error > max_error {
            max_error = error;
            xmax = x;
            y1max = y1;
            y2max = y2;
        }
        if i % (N / 16) == 0 {
            println!(
                " x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
                x,
                y1,
                y2,
                y2 - y1
            );
        }
    }
    println!(
        "!x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
        xmax, y1max, y2max, max_error
    );
    assert!(!max_error.is_nan());
    assert!(max_error < 40532396646334460f64);
}
#[test]
fn test_acos() {
    const N: i32 = 0x100000;
    let tmin = -0.999f64;
    let tmax = 0.999f64;
    let mut max_error = 0.0_f64;
    let mut xmax = tmin;
    let mut y1max = 0.0;
    let mut y2max = 0.0;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.acos();
        let y2 = acos(x as f64) as f64;
        let error = (y1 - y2).abs();
        if error > max_error {
            max_error = error;
            xmax = x;
            y1max = y1;
            y2max = y2;
        }
        if i % (N / 16) == 0 {
            println!(
                " x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
                x,
                y1,
                y2,
                y2 - y1
            );
        }
    }
    println!(
        "!x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
        xmax, y1max, y2max, max_error
    );
    assert!(!max_error.is_nan());
    assert!(max_error < 40532396646334460f64);
}
#[test]
fn test_atan() {
    const N: i32 = 0x100000;
    let tmin = -2f64;
    let tmax = 2f64;
    let mut max_error = 0.0_f64;
    let mut xmax = tmin;
    let mut y1max = 0.0;
    let mut y2max = 0.0;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.atan();
        let y2 = atan(x as f64) as f64;
        let error = (y1 - y2).abs();
        if error > max_error {
            max_error = error;
            xmax = x;
            y1max = y1;
            y2max = y2;
        }
        if i % (N / 16) == 0 {
            println!(
                " x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
                x,
                y1,
                y2,
                y2 - y1
            );
        }
    }
    println!(
        "!x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
        xmax, y1max, y2max, max_error
    );
    assert!(!max_error.is_nan());
    assert!(max_error < 9007199254740992f64);
}
#[test]
fn test_atan2_a() {
    const N: i32 = 0x100000;
    let tmin = -1f64;
    let tmax = 1f64;
    let mut max_error = 0.0_f64;
    let mut xmax = tmin;
    let mut y1max = 0.0;
    let mut y2max = 0.0;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.atan2(1.0);
        let y2 = atan2(x as f64, 1.0) as f64;
        let error = (y1 - y2).abs();
        if error > max_error {
            max_error = error;
            xmax = x;
            y1max = y1;
            y2max = y2;
        }
        if i % (N / 16) == 0 {
            println!(
                " x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
                x,
                y1,
                y2,
                y2 - y1
            );
        }
    }
    println!(
        "!x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
        xmax, y1max, y2max, max_error
    );
    assert!(!max_error.is_nan());
    assert!(max_error < 13510798882111488f64);
}
#[test]
fn test_atan2_b() {
    const N: i32 = 0x100000;
    let tmin = -1f64;
    let tmax = 1f64;
    let mut max_error = 0.0_f64;
    let mut xmax = tmin;
    let mut y1max = 0.0;
    let mut y2max = 0.0;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.atan2(-1.0);
        let y2 = atan2(x as f64, -1.0) as f64;
        let error = (y1 - y2).abs();
        if error > max_error {
            max_error = error;
            xmax = x;
            y1max = y1;
            y2max = y2;
        }
        if i % (N / 16) == 0 {
            println!(
                " x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
                x,
                y1,
                y2,
                y2 - y1
            );
        }
    }
    println!(
        "!x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
        xmax, y1max, y2max, max_error
    );
    assert!(!max_error.is_nan());
    assert!(max_error < 13510798882111488f64);
}
#[test]
fn test_atan2_c() {
    const N: i32 = 0x100000;
    let tmin = -1f64;
    let tmax = 1f64;
    let mut max_error = 0.0_f64;
    let mut xmax = tmin;
    let mut y1max = 0.0;
    let mut y2max = 0.0;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = (1.0_f64).atan2(x);
        let y2 = atan2(1.0, x as f64) as f64;
        let error = (y1 - y2).abs();
        if error > max_error {
            max_error = error;
            xmax = x;
            y1max = y1;
            y2max = y2;
        }
        if i % (N / 16) == 0 {
            println!(
                " x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
                x,
                y1,
                y2,
                y2 - y1
            );
        }
    }
    println!(
        "!x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
        xmax, y1max, y2max, max_error
    );
    assert!(!max_error.is_nan());
    assert!(max_error < 13510798882111488f64);
}
#[test]
fn test_atan2_d() {
    const N: i32 = 0x100000;
    let tmin = -1f64;
    let tmax = 1f64;
    let mut max_error = 0.0_f64;
    let mut xmax = tmin;
    let mut y1max = 0.0;
    let mut y2max = 0.0;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = (-1.0_f64).atan2(x);
        let y2 = atan2(-1.0, x as f64) as f64;
        let error = (y1 - y2).abs();
        if error > max_error {
            max_error = error;
            xmax = x;
            y1max = y1;
            y2max = y2;
        }
        if i % (N / 16) == 0 {
            println!(
                " x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
                x,
                y1,
                y2,
                y2 - y1
            );
        }
    }
    println!(
        "!x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
        xmax, y1max, y2max, max_error
    );
    assert!(!max_error.is_nan());
    assert!(max_error < 13510798882111488f64);
}
#[test]
fn test_exp_a() {
    const N: i32 = 0x100000;
    let tmin = 0f64;
    let tmax = 1f64;
    let mut max_error = 0.0_f64;
    let mut xmax = tmin;
    let mut y1max = 0.0;
    let mut y2max = 0.0;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.exp();
        let y2 = exp(x as f64) as f64;
        let error = (y1 - y2).abs();
        if error > max_error {
            max_error = error;
            xmax = x;
            y1max = y1;
            y2max = y2;
        }
        if i % (N / 16) == 0 {
            println!(
                " x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
                x,
                y1,
                y2,
                y2 - y1
            );
        }
    }
    println!(
        "!x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
        xmax, y1max, y2max, max_error
    );
    assert!(!max_error.is_nan());
    assert!(max_error < 13510798882111488f64);
}
#[test]
fn test_exp_b() {
    const N: i32 = 0x100000;
    let tmin = 1f64;
    let tmax = 2f64;
    let mut max_error = 0.0_f64;
    let mut xmax = tmin;
    let mut y1max = 0.0;
    let mut y2max = 0.0;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.exp();
        let y2 = exp(x as f64) as f64;
        let error = (y1 - y2).abs();
        if error > max_error {
            max_error = error;
            xmax = x;
            y1max = y1;
            y2max = y2;
        }
        if i % (N / 16) == 0 {
            println!(
                " x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
                x,
                y1,
                y2,
                y2 - y1
            );
        }
    }
    println!(
        "!x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
        xmax, y1max, y2max, max_error
    );
    assert!(!max_error.is_nan());
    assert!(max_error < 45035996273704960f64);
}
#[test]
fn test_exp_m1() {
    const N: i32 = 0x100000;
    let tmin = 0f64;
    let tmax = 1f64;
    let mut max_error = 0.0_f64;
    let mut xmax = tmin;
    let mut y1max = 0.0;
    let mut y2max = 0.0;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.exp_m1();
        let y2 = exp_m1(x as f64) as f64;
        let error = (y1 - y2).abs();
        if error > max_error {
            max_error = error;
            xmax = x;
            y1max = y1;
            y2max = y2;
        }
        if i % (N / 16) == 0 {
            println!(
                " x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
                x,
                y1,
                y2,
                y2 - y1
            );
        }
    }
    println!(
        "!x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
        xmax, y1max, y2max, max_error
    );
    assert!(!max_error.is_nan());
    assert!(max_error < 13510798882111488f64);
}
#[test]
fn test_exp2() {
    const N: i32 = 0x100000;
    let tmin = 0f64;
    let tmax = 1f64;
    let mut max_error = 0.0_f64;
    let mut xmax = tmin;
    let mut y1max = 0.0;
    let mut y2max = 0.0;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.exp2();
        let y2 = exp2(x as f64) as f64;
        let error = (y1 - y2).abs();
        if error > max_error {
            max_error = error;
            xmax = x;
            y1max = y1;
            y2max = y2;
        }
        if i % (N / 16) == 0 {
            println!(
                " x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
                x,
                y1,
                y2,
                y2 - y1
            );
        }
    }
    println!(
        "!x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
        xmax, y1max, y2max, max_error
    );
    assert!(!max_error.is_nan());
    assert!(max_error < 9007199254740992f64);
}
#[test]
fn test_ln() {
    const N: i32 = 0x100000;
    let tmin = 1f64;
    let tmax = 2.718281828459045f64;
    let mut max_error = 0.0_f64;
    let mut xmax = tmin;
    let mut y1max = 0.0;
    let mut y2max = 0.0;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.ln();
        let y2 = ln(x as f64) as f64;
        let error = (y1 - y2).abs();
        if error > max_error {
            max_error = error;
            xmax = x;
            y1max = y1;
            y2max = y2;
        }
        if i % (N / 16) == 0 {
            println!(
                " x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
                x,
                y1,
                y2,
                y2 - y1
            );
        }
    }
    println!(
        "!x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
        xmax, y1max, y2max, max_error
    );
    assert!(!max_error.is_nan());
    assert!(max_error < 9007199254740992f64);
}
#[test]
fn test_ln_1p_a() {
    const N: i32 = 0x100000;
    let tmin = 0f64;
    let tmax = 1f64;
    let mut max_error = 0.0_f64;
    let mut xmax = tmin;
    let mut y1max = 0.0;
    let mut y2max = 0.0;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.ln_1p();
        let y2 = ln_1p(x as f64) as f64;
        let error = (y1 - y2).abs();
        if error > max_error {
            max_error = error;
            xmax = x;
            y1max = y1;
            y2max = y2;
        }
        if i % (N / 16) == 0 {
            println!(
                " x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
                x,
                y1,
                y2,
                y2 - y1
            );
        }
    }
    println!(
        "!x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
        xmax, y1max, y2max, max_error
    );
    assert!(!max_error.is_nan());
    assert!(max_error < 9007199254740992f64);
}
#[test]
fn test_ln_1p_b() {
    const N: i32 = 0x100000;
    let tmin = 1f64;
    let tmax = 7.154845485377136f64;
    let mut max_error = 0.0_f64;
    let mut xmax = tmin;
    let mut y1max = 0.0;
    let mut y2max = 0.0;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.ln_1p();
        let y2 = ln_1p(x as f64) as f64;
        let error = (y1 - y2).abs();
        if error > max_error {
            max_error = error;
            xmax = x;
            y1max = y1;
            y2max = y2;
        }
        if i % (N / 16) == 0 {
            println!(
                " x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
                x,
                y1,
                y2,
                y2 - y1
            );
        }
    }
    println!(
        "!x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
        xmax, y1max, y2max, max_error
    );
    assert!(!max_error.is_nan());
    assert!(max_error < 13510798882111488f64);
}
#[test]
fn test_log2() {
    const N: i32 = 0x100000;
    let tmin = 0.25f64;
    let tmax = 4.25f64;
    let mut max_error = 0.0_f64;
    let mut xmax = tmin;
    let mut y1max = 0.0;
    let mut y2max = 0.0;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.log2();
        let y2 = log2(x as f64) as f64;
        let error = (y1 - y2).abs();
        if error > max_error {
            max_error = error;
            xmax = x;
            y1max = y1;
            y2max = y2;
        }
        if i % (N / 16) == 0 {
            println!(
                " x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
                x,
                y1,
                y2,
                y2 - y1
            );
        }
    }
    println!(
        "!x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
        xmax, y1max, y2max, max_error
    );
    assert!(!max_error.is_nan());
    assert!(max_error < 9007199254740992f64);
}
#[test]
fn test_log10() {
    const N: i32 = 0x100000;
    let tmin = 0.1f64;
    let tmax = 10.1f64;
    let mut max_error = 0.0_f64;
    let mut xmax = tmin;
    let mut y1max = 0.0;
    let mut y2max = 0.0;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.log10();
        let y2 = log10(x as f64) as f64;
        let error = (y1 - y2).abs();
        if error > max_error {
            max_error = error;
            xmax = x;
            y1max = y1;
            y2max = y2;
        }
        if i % (N / 16) == 0 {
            println!(
                " x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
                x,
                y1,
                y2,
                y2 - y1
            );
        }
    }
    println!(
        "!x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
        xmax, y1max, y2max, max_error
    );
    assert!(!max_error.is_nan());
    assert!(max_error < 9007199254740992f64);
}
#[test]
fn test_log_2() {
    const N: i32 = 0x100000;
    let tmin = 0.5f64;
    let tmax = 1.5f64;
    let mut max_error = 0.0_f64;
    let mut xmax = tmin;
    let mut y1max = 0.0;
    let mut y2max = 0.0;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.log(2.0);
        let y2 = log(x as f64, 2.0) as f64;
        let error = (y1 - y2).abs();
        if error > max_error {
            max_error = error;
            xmax = x;
            y1max = y1;
            y2max = y2;
        }
        if i % (N / 16) == 0 {
            println!(
                " x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
                x,
                y1,
                y2,
                y2 - y1
            );
        }
    }
    println!(
        "!x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
        xmax, y1max, y2max, max_error
    );
    assert!(!max_error.is_nan());
    assert!(max_error < 9007199254740992f64);
}
#[test]
fn test_log_e() {
    const N: i32 = 0x100000;
    let tmin = 0.5f64;
    let tmax = 1.5f64;
    let mut max_error = 0.0_f64;
    let mut xmax = tmin;
    let mut y1max = 0.0;
    let mut y2max = 0.0;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.log(std::f64::consts::E);
        let y2 = log(x as f64, std::f64::consts::E as f64) as f64;
        let error = (y1 - y2).abs();
        if error > max_error {
            max_error = error;
            xmax = x;
            y1max = y1;
            y2max = y2;
        }
        if i % (N / 16) == 0 {
            println!(
                " x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
                x,
                y1,
                y2,
                y2 - y1
            );
        }
    }
    println!(
        "!x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
        xmax, y1max, y2max, max_error
    );
    assert!(!max_error.is_nan());
    assert!(max_error < 9007199254740992f64);
}
#[test]
fn test_powf_2() {
    const N: i32 = 0x100000;
    let tmin = 0.5f64;
    let tmax = 1.5f64;
    let mut max_error = 0.0_f64;
    let mut xmax = tmin;
    let mut y1max = 0.0;
    let mut y2max = 0.0;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.powf(2.0);
        let y2 = powf(x as f64, 2.0) as f64;
        let error = (y1 - y2).abs();
        if error > max_error {
            max_error = error;
            xmax = x;
            y1max = y1;
            y2max = y2;
        }
        if i % (N / 16) == 0 {
            println!(
                " x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
                x,
                y1,
                y2,
                y2 - y1
            );
        }
    }
    println!(
        "!x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
        xmax, y1max, y2max, max_error
    );
    assert!(!max_error.is_nan());
    assert!(max_error < 18014398509481984f64);
}
#[test]
fn test_powf_m1() {
    const N: i32 = 0x100000;
    let tmin = 0.5f64;
    let tmax = 1.5f64;
    let mut max_error = 0.0_f64;
    let mut xmax = tmin;
    let mut y1max = 0.0;
    let mut y2max = 0.0;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.powf(-1.0);
        let y2 = powf(x as f64, -1.0) as f64;
        let error = (y1 - y2).abs();
        if error > max_error {
            max_error = error;
            xmax = x;
            y1max = y1;
            y2max = y2;
        }
        if i % (N / 16) == 0 {
            println!(
                " x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
                x,
                y1,
                y2,
                y2 - y1
            );
        }
    }
    println!(
        "!x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
        xmax, y1max, y2max, max_error
    );
    assert!(!max_error.is_nan());
    assert!(max_error < 18014398509481984f64);
}
#[test]
fn test_powi_2() {
    const N: i32 = 0x100000;
    let tmin = 0.5f64;
    let tmax = 1.5f64;
    let mut max_error = 0.0_f64;
    let mut xmax = tmin;
    let mut y1max = 0.0;
    let mut y2max = 0.0;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.powi(2);
        let y2 = powi(x as f64, 2) as f64;
        let error = (y1 - y2).abs();
        if error > max_error {
            max_error = error;
            xmax = x;
            y1max = y1;
            y2max = y2;
        }
        if i % (N / 16) == 0 {
            println!(
                " x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
                x,
                y1,
                y2,
                y2 - y1
            );
        }
    }
    println!(
        "!x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
        xmax, y1max, y2max, max_error
    );
    assert!(!max_error.is_nan());
    assert!(max_error < 9007199254740992f64);
}
#[test]
fn test_powi_3() {
    const N: i32 = 0x100000;
    let tmin = 0.12f64;
    let tmax = 1.2f64;
    let mut max_error = 0.0_f64;
    let mut xmax = tmin;
    let mut y1max = 0.0;
    let mut y2max = 0.0;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.powi(3);
        let y2 = powi(x as f64, 3) as f64;
        let error = (y1 - y2).abs();
        if error > max_error {
            max_error = error;
            xmax = x;
            y1max = y1;
            y2max = y2;
        }
        if i % (N / 16) == 0 {
            println!(
                " x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
                x,
                y1,
                y2,
                y2 - y1
            );
        }
    }
    println!(
        "!x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
        xmax, y1max, y2max, max_error
    );
    assert!(!max_error.is_nan());
    assert!(max_error < 18014398509481984f64);
}
#[test]
fn test_powi_m1() {
    const N: i32 = 0x100000;
    let tmin = 0.5f64;
    let tmax = 1.5f64;
    let mut max_error = 0.0_f64;
    let mut xmax = tmin;
    let mut y1max = 0.0;
    let mut y2max = 0.0;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.powi(-1);
        let y2 = powi(x as f64, -1) as f64;
        let error = (y1 - y2).abs();
        if error > max_error {
            max_error = error;
            xmax = x;
            y1max = y1;
            y2max = y2;
        }
        if i % (N / 16) == 0 {
            println!(
                " x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
                x,
                y1,
                y2,
                y2 - y1
            );
        }
    }
    println!(
        "!x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
        xmax, y1max, y2max, max_error
    );
    assert!(!max_error.is_nan());
    assert!(max_error < 9007199254740992f64);
}
#[test]
fn test_powi_m2() {
    const N: i32 = 0x100000;
    let tmin = 0.5f64;
    let tmax = 1.5f64;
    let mut max_error = 0.0_f64;
    let mut xmax = tmin;
    let mut y1max = 0.0;
    let mut y2max = 0.0;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.powi(-2);
        let y2 = powi(x as f64, -2) as f64;
        let error = (y1 - y2).abs();
        if error > max_error {
            max_error = error;
            xmax = x;
            y1max = y1;
            y2max = y2;
        }
        if i % (N / 16) == 0 {
            println!(
                " x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
                x,
                y1,
                y2,
                y2 - y1
            );
        }
    }
    println!(
        "!x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
        xmax, y1max, y2max, max_error
    );
    assert!(!max_error.is_nan());
    assert!(max_error < 27021597764222976f64);
}
#[test]
fn test_powi_16() {
    const N: i32 = 0x100000;
    let tmin = 0.25f64;
    let tmax = 1f64;
    let mut max_error = 0.0_f64;
    let mut xmax = tmin;
    let mut y1max = 0.0;
    let mut y2max = 0.0;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.powi(16);
        let y2 = powi(x as f64, 16) as f64;
        let error = (y1 - y2).abs();
        if error > max_error {
            max_error = error;
            xmax = x;
            y1max = y1;
            y2max = y2;
        }
        if i % (N / 16) == 0 {
            println!(
                " x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
                x,
                y1,
                y2,
                y2 - y1
            );
        }
    }
    println!(
        "!x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
        xmax, y1max, y2max, max_error
    );
    assert!(!max_error.is_nan());
    assert!(max_error < 31525197391593470f64);
}
#[test]
fn test_cosh() {
    const N: i32 = 0x100000;
    let tmin = -1f64;
    let tmax = 1f64;
    let mut max_error = 0.0_f64;
    let mut xmax = tmin;
    let mut y1max = 0.0;
    let mut y2max = 0.0;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.cosh();
        let y2 = cosh(x as f64) as f64;
        let error = (y1 - y2).abs();
        if error > max_error {
            max_error = error;
            xmax = x;
            y1max = y1;
            y2max = y2;
        }
        if i % (N / 16) == 0 {
            println!(
                " x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
                x,
                y1,
                y2,
                y2 - y1
            );
        }
    }
    println!(
        "!x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
        xmax, y1max, y2max, max_error
    );
    assert!(!max_error.is_nan());
    assert!(max_error < 9007199254740992f64);
}
#[test]
fn test_sinh() {
    const N: i32 = 0x100000;
    let tmin = -1f64;
    let tmax = 1f64;
    let mut max_error = 0.0_f64;
    let mut xmax = tmin;
    let mut y1max = 0.0;
    let mut y2max = 0.0;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.sinh();
        let y2 = sinh(x as f64) as f64;
        let error = (y1 - y2).abs();
        if error > max_error {
            max_error = error;
            xmax = x;
            y1max = y1;
            y2max = y2;
        }
        if i % (N / 16) == 0 {
            println!(
                " x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
                x,
                y1,
                y2,
                y2 - y1
            );
        }
    }
    println!(
        "!x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
        xmax, y1max, y2max, max_error
    );
    assert!(!max_error.is_nan());
    assert!(max_error < 9007199254740992f64);
}
#[test]
fn test_tanh() {
    const N: i32 = 0x100000;
    let tmin = -1f64;
    let tmax = 1f64;
    let mut max_error = 0.0_f64;
    let mut xmax = tmin;
    let mut y1max = 0.0;
    let mut y2max = 0.0;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.tanh();
        let y2 = tanh(x as f64) as f64;
        let error = (y1 - y2).abs();
        if error > max_error {
            max_error = error;
            xmax = x;
            y1max = y1;
            y2max = y2;
        }
        if i % (N / 16) == 0 {
            println!(
                " x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
                x,
                y1,
                y2,
                y2 - y1
            );
        }
    }
    println!(
        "!x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
        xmax, y1max, y2max, max_error
    );
    assert!(!max_error.is_nan());
    assert!(max_error < 9007199254740992f64);
}
#[test]
fn test_acosh() {
    const N: i32 = 0x100000;
    let tmin = -1f64;
    let tmax = 1f64;
    let mut max_error = 0.0_f64;
    let mut xmax = tmin;
    let mut y1max = 0.0;
    let mut y2max = 0.0;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.acosh();
        let y2 = acosh(x as f64) as f64;
        let error = (y1 - y2).abs();
        if error > max_error {
            max_error = error;
            xmax = x;
            y1max = y1;
            y2max = y2;
        }
        if i % (N / 16) == 0 {
            println!(
                " x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
                x,
                y1,
                y2,
                y2 - y1
            );
        }
    }
    println!(
        "!x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
        xmax, y1max, y2max, max_error
    );
    assert!(!max_error.is_nan());
    assert!(max_error < 9007199254740992f64);
}
#[test]
fn test_asinh() {
    const N: i32 = 0x100000;
    let tmin = -1f64;
    let tmax = 1f64;
    let mut max_error = 0.0_f64;
    let mut xmax = tmin;
    let mut y1max = 0.0;
    let mut y2max = 0.0;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.asinh();
        let y2 = asinh(x as f64) as f64;
        let error = (y1 - y2).abs();
        if error > max_error {
            max_error = error;
            xmax = x;
            y1max = y1;
            y2max = y2;
        }
        if i % (N / 16) == 0 {
            println!(
                " x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
                x,
                y1,
                y2,
                y2 - y1
            );
        }
    }
    println!(
        "!x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
        xmax, y1max, y2max, max_error
    );
    assert!(!max_error.is_nan());
    assert!(max_error < 13510798882111488f64);
}
#[test]
fn test_atanh() {
    const N: i32 = 0x100000;
    let tmin = -0.9f64;
    let tmax = 0.9f64;
    let mut max_error = 0.0_f64;
    let mut xmax = tmin;
    let mut y1max = 0.0;
    let mut y2max = 0.0;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.atanh();
        let y2 = atanh(x as f64) as f64;
        let error = (y1 - y2).abs();
        if error > max_error {
            max_error = error;
            xmax = x;
            y1max = y1;
            y2max = y2;
        }
        if i % (N / 16) == 0 {
            println!(
                " x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
                x,
                y1,
                y2,
                y2 - y1
            );
        }
    }
    println!(
        "!x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
        xmax, y1max, y2max, max_error
    );
    assert!(!max_error.is_nan());
    assert!(max_error < 13510798882111488f64);
}
#[test]
fn test_hypot_a() {
    const N: i32 = 0x100000;
    let tmin = -1f64;
    let tmax = 1f64;
    let mut max_error = 0.0_f64;
    let mut xmax = tmin;
    let mut y1max = 0.0;
    let mut y2max = 0.0;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.hypot(1.0);
        let y2 = hypot(x as f64, 1.0) as f64;
        let error = (y1 - y2).abs();
        if error > max_error {
            max_error = error;
            xmax = x;
            y1max = y1;
            y2max = y2;
        }
        if i % (N / 16) == 0 {
            println!(
                " x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
                x,
                y1,
                y2,
                y2 - y1
            );
        }
    }
    println!(
        "!x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
        xmax, y1max, y2max, max_error
    );
    assert!(!max_error.is_nan());
    assert!(max_error < 13510798882111488f64);
}
#[test]
fn test_hypot_b() {
    const N: i32 = 0x100000;
    let tmin = -1f64;
    let tmax = 1f64;
    let mut max_error = 0.0_f64;
    let mut xmax = tmin;
    let mut y1max = 0.0;
    let mut y2max = 0.0;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.hypot(-1.0);
        let y2 = hypot(x as f64, -1.0) as f64;
        let error = (y1 - y2).abs();
        if error > max_error {
            max_error = error;
            xmax = x;
            y1max = y1;
            y2max = y2;
        }
        if i % (N / 16) == 0 {
            println!(
                " x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
                x,
                y1,
                y2,
                y2 - y1
            );
        }
    }
    println!(
        "!x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
        xmax, y1max, y2max, max_error
    );
    assert!(!max_error.is_nan());
    assert!(max_error < 13510798882111488f64);
}
#[test]
fn test_hypot_c() {
    const N: i32 = 0x100000;
    let tmin = -1f64;
    let tmax = 1f64;
    let mut max_error = 0.0_f64;
    let mut xmax = tmin;
    let mut y1max = 0.0;
    let mut y2max = 0.0;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = (1.0_f64).hypot(x);
        let y2 = hypot(1.0, x as f64) as f64;
        let error = (y1 - y2).abs();
        if error > max_error {
            max_error = error;
            xmax = x;
            y1max = y1;
            y2max = y2;
        }
        if i % (N / 16) == 0 {
            println!(
                " x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
                x,
                y1,
                y2,
                y2 - y1
            );
        }
    }
    println!(
        "!x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
        xmax, y1max, y2max, max_error
    );
    assert!(!max_error.is_nan());
    assert!(max_error < 13510798882111488f64);
}
#[test]
fn test_hypot_d() {
    const N: i32 = 0x100000;
    let tmin = -1f64;
    let tmax = 1f64;
    let mut max_error = 0.0_f64;
    let mut xmax = tmin;
    let mut y1max = 0.0;
    let mut y2max = 0.0;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = (-1.0_f64).hypot(x);
        let y2 = hypot(-1.0, x as f64) as f64;
        let error = (y1 - y2).abs();
        if error > max_error {
            max_error = error;
            xmax = x;
            y1max = y1;
            y2max = y2;
        }
        if i % (N / 16) == 0 {
            println!(
                " x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
                x,
                y1,
                y2,
                y2 - y1
            );
        }
    }
    println!(
        "!x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
        xmax, y1max, y2max, max_error
    );
    assert!(!max_error.is_nan());
    assert!(max_error < 13510798882111488f64);
}
#[test]
fn test_sqrt() {
    const N: i32 = 0x100000;
    let tmin = 0.5f64;
    let tmax = 2f64;
    let mut max_error = 0.0_f64;
    let mut xmax = tmin;
    let mut y1max = 0.0;
    let mut y2max = 0.0;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.sqrt();
        let y2 = sqrt(x as f64) as f64;
        let error = (y1 - y2).abs();
        if error > max_error {
            max_error = error;
            xmax = x;
            y1max = y1;
            y2max = y2;
        }
        if i % (N / 16) == 0 {
            println!(
                " x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
                x,
                y1,
                y2,
                y2 - y1
            );
        }
    }
    println!(
        "!x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
        xmax, y1max, y2max, max_error
    );
    assert!(!max_error.is_nan());
    assert!(max_error < 4503599627370496f64);
}
#[test]
fn test_cbrt() {
    const N: i32 = 0x100000;
    let tmin = -2f64;
    let tmax = 2f64;
    let mut max_error = 0.0_f64;
    let mut xmax = tmin;
    let mut y1max = 0.0;
    let mut y2max = 0.0;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.cbrt();
        let y2 = cbrt(x as f64) as f64;
        let error = (y1 - y2).abs();
        if error > max_error {
            max_error = error;
            xmax = x;
            y1max = y1;
            y2max = y2;
        }
        if i % (N / 16) == 0 {
            println!(
                " x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
                x,
                y1,
                y2,
                y2 - y1
            );
        }
    }
    println!(
        "!x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
        xmax, y1max, y2max, max_error
    );
    assert!(!max_error.is_nan());
    assert!(max_error < 4503599627370496f64);
}
#[test]
fn test_recip() {
    const N: i32 = 0x100000;
    let tmin = 0.5f64;
    let tmax = 1.5f64;
    let mut max_error = 0.0_f64;
    let mut xmax = tmin;
    let mut y1max = 0.0;
    let mut y2max = 0.0;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.recip();
        let y2 = recip(x as f64) as f64;
        let error = (y1 - y2).abs();
        if error > max_error {
            max_error = error;
            xmax = x;
            y1max = y1;
            y2max = y2;
        }
        if i % (N / 16) == 0 {
            println!(
                " x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
                x,
                y1,
                y2,
                y2 - y1
            );
        }
    }
    println!(
        "!x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
        xmax, y1max, y2max, max_error
    );
    assert!(!max_error.is_nan());
    assert!(max_error < 9007199254740992f64);
}
#[test]
fn test_recip_n() {
    const N: i32 = 0x100000;
    let tmin = -1.5f64;
    let tmax = -0.5f64;
    let mut max_error = 0.0_f64;
    let mut xmax = tmin;
    let mut y1max = 0.0;
    let mut y2max = 0.0;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.recip();
        let y2 = recip(x as f64) as f64;
        let error = (y1 - y2).abs();
        if error > max_error {
            max_error = error;
            xmax = x;
            y1max = y1;
            y2max = y2;
        }
        if i % (N / 16) == 0 {
            println!(
                " x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
                x,
                y1,
                y2,
                y2 - y1
            );
        }
    }
    println!(
        "!x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
        xmax, y1max, y2max, max_error
    );
    assert!(!max_error.is_nan());
    assert!(max_error < 9007199254740992f64);
}
#[test]
fn test_recip_x() {
    const N: i32 = 0x100000;
    let tmin = 0.5f64;
    let tmax = 1.5f64;
    let mut max_error = 0.0_f64;
    let mut xmax = tmin;
    let mut y1max = 0.0;
    let mut y2max = 0.0;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.recip();
        let y2 = recip_approx(x as f64) as f64;
        let error = (y1 - y2).abs();
        if error > max_error {
            max_error = error;
            xmax = x;
            y1max = y1;
            y2max = y2;
        }
        if i % (N / 16) == 0 {
            println!(
                " x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
                x,
                y1,
                y2,
                y2 - y1
            );
        }
    }
    println!(
        "!x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
        xmax, y1max, y2max, max_error
    );
    assert!(!max_error.is_nan());
    assert!(max_error < 0.1f64);
}
#[test]
fn test_recip_y() {
    const N: i32 = 0x100000;
    let tmin = -1.5f64;
    let tmax = -0.5f64;
    let mut max_error = 0.0_f64;
    let mut xmax = tmin;
    let mut y1max = 0.0;
    let mut y2max = 0.0;
    for i in 0..=N {
        let x = i as f64 * (tmax - tmin) / N as f64 + tmin;
        let y1 = x.recip();
        let y2 = recip_approx(x as f64) as f64;
        let error = (y1 - y2).abs();
        if error > max_error {
            max_error = error;
            xmax = x;
            y1max = y1;
            y2max = y2;
        }
        if i % (N / 16) == 0 {
            println!(
                " x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
                x,
                y1,
                y2,
                y2 - y1
            );
        }
    }
    println!(
        "!x={:25.20} y1={:25.20} y2={:25.20} e={:25.20}",
        xmax, y1max, y2max, max_error
    );
    assert!(!max_error.is_nan());
    assert!(max_error < 0.1f64);
}
