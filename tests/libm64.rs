type fty = f64;
type ity = i64;
type uty = u64;

use std::f64::consts::LOG2_10;
use std::f64::consts::LOG2_E;
use std::f64::consts::PI;

fn select(a: bool, b: fty, c: fty) -> fty {
    if a {
        b
    } else {
        c
    }
}

fn iabs(i: ity) -> ity {
    i.abs()
}

fn f(f: fty) -> fty {
    f
}

fn from_bits(u: uty) -> fty {
    fty::from_bits(u)
}

fn to_bits(f: fty) -> uty {
    fty::to_bits(f)
}
fn exp2(arg: fty) -> fty {
    let r: fty = arg.round();
    let mul: fty = fty::from_bits(
        (r.mul_add(0x0010000000000000_u64 as fty, 0x3ff0000000000000_u64 as fty)) as uty,
    );
    let x: fty = arg - r;
    (from_bits(4549839347750377909u64))
        .mul_add(x, from_bits(4563827094295188139u64))
        .mul_add(x, from_bits(4576698039041613846u64))
        .mul_add(x, from_bits(4588159642448921967u64))
        .mul_add(x, from_bits(4597823092488205992u64))
        .mul_add(x, from_bits(4604418534717280147u64))
        .mul_add(x, from_bits(4607182418800017408u64))
        * mul
}
fn exp(arg: fty) -> fty {
    exp2(arg * LOG2_E)
}
