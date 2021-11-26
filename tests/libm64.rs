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
    (0.000154697319774755481135 as fty)
        .mul_add(x, 0.001341000096610015706912 as fty)
        .mul_add(x, 0.009618030782505153797811 as fty)
        .mul_add(x, 0.055502973141993247574765 as fty)
        .mul_add(x, 0.240226510841172935738043 as fty)
        .mul_add(x, 0.693147225395011086525471 as fty)
        .mul_add(x, 1.000000000000000000000000 as fty)
        * mul
}
fn exp(arg: fty) -> fty {
    exp2(arg * LOG2_E)
}
fn qnorm(arg: fty) -> fty {
    let scaled: fty = arg - 0.5;
    let x = scaled;
    let recip: fty = 1.0 / (x * x - 0.5 * 0.5);
    let y: fty = (177186111.131545818686411653000483 as fty)
        .mul_add(x * x, -219058235.58919835 as fty)
        .mul_add(x * x, 117054121.857504129646289572504640 as fty)
        .mul_add(x * x, -35345955.68660036 as fty)
        .mul_add(x * x, 6623473.609141078534685775398250 as fty)
        .mul_add(x * x, -796318.1973069897 as fty)
        .mul_add(x * x, 61391.409088151006196662227193 as fty)
        .mul_add(x * x, -2938.7971360761 as fty)
        .mul_add(x * x, 83.911295471202339471921364 as fty)
        .mul_add(x * x, 0.012702493639562371692090 as fty)
        .mul_add(x * x, 1.856861340488065073103038 as fty)
        .mul_add(x * x, -0.626662948075053 as fty)
        * x;
    #[doc("Re-assembly.")]
    y * recip
}
