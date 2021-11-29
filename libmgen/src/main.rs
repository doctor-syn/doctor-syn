//!    Libmgen - generates maths libraries for many targets.
//!    Copyright (C) 2021 Andy Thomason
//!
//!    This program is free software: you can redistribute it and/or modify
//!    it under the terms of the GNU General Public License as published by
//!    the Free Software Foundation, either version 3 of the License, or
//!    (at your option) any later version.
//!
//!    This program is distributed in the hope that it will be useful,
//!    but WITHOUT ANY WARRANTY; without even the implied warranty of
//!    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//!    GNU General Public License for more details.
//!
//!    You should have received a copy of the GNU General Public License
//!    along with this program.  If not, see <https://www.gnu.org/licenses/>.
//! 
//! 
//! Note that generated code is not covered by any license but is also
//! WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
//! 
//! It is up to the user to test that results match the requirements of your
//! project.

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use proc_macro2::{TokenStream};

use std::io::Write;

mod auxfuncs;
mod config;
mod functions;
mod hyperbolic;
mod inv_trig;
mod log_exp;
mod recip_sqrt;
mod stats_random;
mod stats_norm;
mod test;
mod trig;

use config::Config;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "libmgen",
    about = "Generate maths and stats functions in many languages."
)]
struct Opt {
    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,

    /// Generate tests.
    #[structopt(long)]
    generate_tests: bool,

    /// Output file, stdout if not present
    #[structopt(short, long, parse(from_os_str))]
    output: Option<PathBuf>,

    /// List of functions and groups of functions to generate
    /// as a comma-separated list. Use "help" for a list.
    ///
    /// Examples: sin,cos,ln,exp
    #[structopt(short, long, default_value = "")]
    functions: String,

    /// Number of floating point bits: 32 or 64.
    #[structopt(long, default_value = "64")]
    num_bits: usize,

    /// Number format - hex or decimal.
    #[structopt(long, default_value = "f64")]
    number_type: String,

    /// Target language. C, C++, rust, fortran.
    #[structopt(long, default_value = "rust")]
    language: String,

    /// Function prefix
    #[structopt(long, default_value = "")]
    function_prefix: String,
}

/*
const RUST_SCALAR_HEADER : &'static str = r#"
fn select(a: bool, b: fty, c: fty) -> fty {
    if a { b } else { c }
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
"#;
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
    let (stats_norm, stats_norm_tests) = gen_stats_norm(&config);
    let (aux, aux_tests) = gen_aux(&config);

    let functions: Vec<Stmt> = parse_quote!(
        #trig
        #inv_trig
        #log_exp
        #hyperbolic
        #recip_sqrt
        #stats_norm
        #aux
    );

    let tests: Vec<Stmt> = parse_quote!(
        #trig_tests
        #inv_trig_tests
        #log_exp_tests
        #hyperbolic_tests
        #recip_sqrt_tests
        #stats_norm_tests
        #aux_tests
    );

    match config.language() {
        "rust_scalar" => {
            writeln!(file, "type fty = f64;")?;
            writeln!(file, "type ity = i64;")?;
            writeln!(file, "type uty = u64;")?;
            writeln!(file, "")?;
            writeln!(file, "use std::f64::consts::PI;")?;
            writeln!(file, "use std::f64::consts::LOG2_E;")?;
            writeln!(file, "use std::f64::consts::LOG2_10;")?;
            writeln!(file, "")?;
            file.write_all(RUST_SCALAR_HEADER.as_bytes())?;

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

fn generate() {
    // let val = doctor_syn::expr!((2.16065388452409390396).cos()).eval(60).unwrap();
    // let bd : doctor_syn::bigdecimal::BigDecimal = val.into();
    //let val = doctor_syn::expr!(123456789.123456789123456789123456789123456789).eval(60).unwrap();
    //let bd : bigdecimal::BigDecimal = "123456789.123456789123456789123456789123456789".parse().unwrap();
    //let val = doctor_syn::expr!(123456789.12345678912let#fty3456789123456789123456789);
    //let val = doctor_syn::Expression::from(bd);
    // println!("val={}", val);
    // println!("bd={}", bd);

    // generate_libm("tests/libm32.rs", 32, "f32_hex", "rust_scalar").unwrap();
    // generate_libm("tests/libm64.rs", 64, "f64_hex", "rust_scalar").unwrap();

    if true {
        let mut config = Config::new(64, "f64", "rust_scalar", false, "");
        config.add_function("exp");
        config.add_function("qnorm");

        generate_libm("tests/libm64.rs", &config).unwrap();
    }

    if false {
        let mut config = Config::new(64, "f64", "c_scalar", false, "f64_");
        config.add_function("ln");

        generate_libm("tests/libm64_scalar.c", &config).unwrap();
    }

    if false {
        let mut config = Config::new(64, "f64_hex", "c_vector", false, "f64x8_");
        config.add_function("ln");

        generate_libm("tests/libm64_vector.c", &config).unwrap();
    }
}
*/

fn main() {
    let opt = Opt::from_args();
    if opt.debug {
        println!("opt={:?}", opt);
    }

    if opt.functions.is_empty() {
        eprintln!("re-run with -f or -h");
        return;
    }

    if opt.functions == "help" {
        for f in functions::FUNCTIONS {
            println!("{}", f.name);
        }
        return;
    }

    let names = opt
        .functions
        .split(',')
        .map(str::to_string)
        .collect::<Vec<_>>();
    let funcs = functions::get_functions_and_deps(&names);

    let config: Config = Config::new(
        opt.num_bits,
        &opt.number_type,
        &opt.language,
        opt.generate_tests,
        &opt.function_prefix,
    );

    let mut tokens = TokenStream::new();

    for f in funcs.iter() {
        if let Some(gen) = f.gen {
            let num_terms = if config.num_bits() == 32 {
                f.num_terms[0]
            } else {
                f.num_terms[1]
            };
            tokens.extend(gen(num_terms, &config));
        }
    }

    if opt.generate_tests {
        tokens.extend(crate::auxfuncs::gen_test_function(0, &config));
        for f in funcs {
            for t in f.test_specs {
                tokens.extend(crate::test::gen_test(t, &config));
    
            }
        }
    }

    let text = doctor_syn::codegen::rust::format_token_stream(tokens);

    if let Some(path) = &opt.output {
        std::fs::write(path, text.as_bytes()).unwrap();
    } else {
        std::io::stdout().write_all(text.as_bytes()).unwrap();
    }
}
