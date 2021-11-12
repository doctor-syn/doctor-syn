use quote::quote;
use std::io::Write;
use syn::parse_quote;
use syn::Stmt;

mod config;
mod auxfuncs;
mod hyperbolic;
mod inv_trig;
mod log_exp;
mod recip_sqrt;
mod test;
mod trig;

use auxfuncs::*;
use doctor_syn::codegen::c;
use hyperbolic::*;
use inv_trig::*;
use log_exp::*;
use recip_sqrt::*;
use trig::*;
use config::Config;

const C_SCALAR_HEADER : &'static str = r#"
#include<math.h>

typedef double f64;
typedef long long i64;
typedef unsigned long long u64;
typedef long long bool;

#define REP(X) {X, X, X, X, X, X, X, X}
#define REINTERP(from, F, T) union { F f; T t; } u; u.f = from; return u.t;

inline f64 f64_mul_add(f64 a, f64 b, f64 c) {
    return a * b + c;
}

inline f64 f64_select(bool a, f64 b, f64 c) {
    return a ? b : c;
}

inline f64 f64_round(f64 a) {
    return round(a);
}

inline f64 f64_f(double f) {
    return (f64)f;
}

inline u64 f64_mkuty(long long v) {
    return (u64)v;
}

inline f64 f64_mkfty(long long v) {
    REINTERP(v, long long, double)
}

inline u64 f64_reinterpret_fty_uty(f64 f) {
    REINTERP(f, f64, u64)
}

inline f64 f64_reinterpret_uty_fty(u64 f) {
    REINTERP(f, u64, f64)
}

const f64 PI = M_PI;
const f64 LOG2_E = M_LOG2E;
const f64 LOG2_10 = M_LN10 / M_LN2;
const f64 MIN_POSITIVE = 2.2250738585072014E-308;
"#;

const C_VECTOR_HEADER : &'static str = r#"
#include<math.h>

typedef double f64 __attribute__ ((vector_size (64)));
typedef long long i64 __attribute__ ((vector_size (64)));
typedef unsigned long long u64 __attribute__ ((vector_size (64)));
typedef int bool __attribute__ ((vector_size (64)));

#define REP(X) {X, X, X, X, X, X, X, X}
#define REINTERP(f, F, T) { union { f: F, t: T } u; u.f = f; u.t }

inline f64 f64_mul_add(f64 a, f64 b, f64 c) {
    return a * b + c;
}

inline f64 f64_select(bool a, f64 b, f64 c) {
    return (f64)(((i64)b & (i64)a) | ((i64)c & ~(i64)a));
}

inline f64 f64_round(f64 a) {
    return (f64){ round(a[0]), round(a[1]), round(a[2]), round(a[3]), round(a[4]), round(a[5]), round(a[6]), round(a[7]) };
}

inline i64 f64_reinterpret_fi(f64 f) {
    return REINTERP(f, f64, i64)
  }
  
inline i64 f64_reinterpret_if(i64 f) {
    return REINTERP(f, i64, f64)
}

inline f64 f64_f(double v) {
    return (f64)REP(v);
}

inline i64 f64_i(long long v) {
    return (i64)REP(v);
}
    
inline i64 f64_u(long long v) {
    return (i64)REP(v);
}
    
inline f64 f64_uf(unsigned long v) {
  double x = REINTERP(v, unsigned long, double);
  return (f64)REP(x);
}

inline f64 f64_cvt_if(i64 v) {
    return __builtin_convertvector(v, f64);
}

const f64 PI = REP(M_PI);
const f64 LOG2_E = REP(M_LOG2E);
const f64 LOG2_10 = REP(M_LN10 / M_LN2);
"#;

fn generate_libm(
    path: &str,
    config: &Config,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut file = std::fs::File::create(path)?;

    let (trig, trig_tests) = gen_single_pass_trig(&config);
    let (inv_trig, inv_trig_tests) = gen_inv_trig(&config);
    let (log_exp, log_exp_tests) = gen_log_exp(&config);
    let (hyperbolic, hyperbolic_tests) = gen_hyperbolic(&config);
    let (recip_sqrt, recip_sqrt_tests) = gen_recip_sqrt(&config);
    let (aux, aux_tests) = gen_aux(&config);

    let functions: Vec<Stmt> = parse_quote!(
        #trig
        #inv_trig
        #log_exp
        #hyperbolic
        #recip_sqrt
        #aux
    );

    let tests: Vec<Stmt> = parse_quote!(
        #trig_tests
        #inv_trig_tests
        #log_exp_tests
        #hyperbolic_tests
        #recip_sqrt_tests
        #aux_tests
    );

    match config.language() {
        "rust" => {
            writeln!(file, "type fty = f64;")?;
            writeln!(file, "type ity = i64;")?;
            writeln!(file, "type uty = u64;")?;
            writeln!(file, "")?;
            file.write_all(b"use std::f64::consts::PI;\n")?;
            file.write_all(b"use std::f64::consts::LOG2_E;\n")?;
            file.write_all(b"use std::f64::consts::LOG2_10;\n")?;
            file.write_all(b"\n")?;
            file.write_all(b"fn select(a: bool, b: fty, c: fty) -> fty {\n")?;
            file.write_all(b"    if a { b } else { c }\n")?;
            file.write_all(b"}\n")?;
            file.write_all(b"\n")?;
            file.write_all(b"fn iabs(i: ity) -> ity {\n")?;
            file.write_all(b"    i.abs()\n")?;
            file.write_all(b"}\n")?;
            file.write_all(b"\n")?;
            file.write_all(b"const fn fu(u: uty) -> fty {\n")?;
            file.write_all(b"    std::f64::from_bits(u)\n")?;
            file.write_all(b"}\n")?;
            file.write_all(b"\n")?;
            file.write_all(b"const fn f(f: fty) -> ity {\n")?;
            file.write_all(b"    f\n")?;
            file.write_all(b"}\n")?;
            file.write_all(b"\n")?;
            file.write_all(b"const fn from_bits(u: uty) -> fty {\n")?;
            file.write_all(b"    std::f64::from_bits(u)\n")?;
            file.write_all(b"}\n")?;
            file.write_all(b"\n")?;
            file.write_all(b"const fn to_bits(f: fty) -> uty {\n")?;
            file.write_all(b"    std::f64::to_bits(f)\n")?;
            file.write_all(b"}\n")?;

            for stmt in functions.iter().chain(tests.iter()) {
                let tokens = quote!(#stmt);
                file.write_all(tokens.to_string().as_bytes())?;
            }
        }
        "c_scalar" => {
            file.write_all(C_SCALAR_HEADER.as_bytes())?;

            for stmt in functions.iter().chain(tests.iter()) {
                if let Stmt::Item(item) = stmt {
                    use c::AsC;
                    let context = c::Context::new(config.prefix());
                    let code = item.as_c(&context)?;
                    file.write_all(code.as_bytes())?;
                }
            }
        }
        "c_vector" => {
            file.write_all(b"#define USE_F64\n\n")?;
            file.write_all(C_VECTOR_HEADER.as_bytes())?;

            for stmt in functions.iter().chain(tests.iter()) {
                if let Stmt::Item(item) = stmt {
                    use c::AsC;
                    let context = c::Context::new(config.prefix());
                    let code = item.as_c(&context)?;
                    file.write_all(code.as_bytes())?;
                }
            }
        }
        language => panic!("language {} not supported", language),
    }

    Ok(())
}

fn main() {
    // let val = doctor_syn::expr!((2.16065388452409390396).cos()).eval(60).unwrap();
    // let bd : doctor_syn::bigdecimal::BigDecimal = val.into();
    //let val = doctor_syn::expr!(123456789.123456789123456789123456789123456789).eval(60).unwrap();
    //let bd : bigdecimal::BigDecimal = "123456789.123456789123456789123456789123456789".parse().unwrap();
    //let val = doctor_syn::expr!(123456789.12345678912let#fty3456789123456789123456789);
    //let val = doctor_syn::Expression::from(bd);
    // println!("val={}", val);
    // println!("bd={}", bd);

    // generate_libm("tests/libm32.rs", 32, "f32_hex", "rust").unwrap();
    // generate_libm("tests/libm64.rs", 64, "f64_hex", "rust").unwrap();

    if true {
        let mut config = Config::new(64, "f64_hex", "rust", false, "");
        config.add_function("exp");
        config.add_function("qnorm");
    
        generate_libm("tests/libm32.rs", &config).unwrap();
    }

    if false {
        let mut config = Config::new(64, "f64_hex", "c_scalar", false, "f64_");
        config.add_function("ln");
    
        generate_libm("tests/libm64_scalar.c", &config).unwrap();
    }

    if false {
        let mut config = Config::new(64, "f64_hex", "c_vector", false, "f64x8_");
        config.add_function("ln");
    
        generate_libm("tests/libm64_vector.c", &config).unwrap();
    }
}
