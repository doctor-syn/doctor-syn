type fty = f64;
type ity = i64;
type uty = u64;

use std::f64::consts::PI;
use std::f64::consts::LOG2_E;
use std::f64::consts::LOG2_10;

fn select(a: bool, b: fty, c: fty) -> fty {
    if a { b } else { c }
}

fn iabs(i: ity) -> ity {
    i.abs()
}

const fn fu(u: uty) -> fty {
    std::f64::from_bits(u)
}

const fn f(f: fty) -> ity {
    f
}

const fn from_bits(u: uty) -> fty {
    std::f64::from_bits(u)
}

const fn to_bits(f: fty) -> uty {
    std::f64::to_bits(f)
}
fn exp2 (arg : f64) -> f64 { let r : f64 = arg . round () ; let mul : f64 = f64 :: from_bits ((r . mul_add (0x0010000000000000_u64 as f64 , 0x3ff0000000000000_u64 as f64)) as u64) ; let x : f64 = arg - r ; (mkfty (4549839347750377909u64)) . mul_add (x , mkfty (4563827094295188139u64)) . mul_add (x , mkfty (4576698039041613846u64)) . mul_add (x , mkfty (4588159642448921967u64)) . mul_add (x , mkfty (4597823092488205992u64)) . mul_add (x , mkfty (4604418534717280147u64)) . mul_add (x , mkfty (4607182418800017408u64)) * mul }fn exp (arg : f64) -> f64 { exp2 (arg * LOG2_E) }fn negate_on_odd (x : f64 , y : f64) -> f64 { let sign_bit : u64 = (((x as i64) & 1) << 63i32) as u64 ; f64 :: from_bits (sign_bit ^ y . to_bits ()) }fn recip_approx (x : f64) -> f64 { let y : f64 = f64 :: from_bits ((((x . abs () . to_bits () as f64) . mul_add (- 1.0 , 0x3ff0000000000000_u64 as f64 * 2.0))) as u64) ; (y - 0.08) . copysign (x) }fn sqrt_approx (x : f64) -> f64 { let y : f64 = f64 :: from_bits ((((x . abs () . to_bits () as f64) . mul_add (0.5 , 0x3ff0000000000000_u64 as f64 * 0.5))) as u64) ; y - 0.08 }fn cbrt_approx (x : f64) -> f64 { let y : f64 = f64 :: from_bits ((((x . abs () . to_bits () as f64) . mul_add (1.0 / 3.0 , 0x3ff0000000000000_u64 as f64 * 2.0 / 3.0))) as u64) ; (y - 0.08) . copysign (x) }