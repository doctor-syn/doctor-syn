use crate::config::Config;
use proc_macro2::TokenStream;
use std::f64::consts::PI;

pub enum TestType {
    MaxAbs(&'static str, &'static str, f64, f64, usize)
}

pub struct TestSpec {
    pub test_name: &'static str,
    pub ref_expr: &'static str,
    pub rust_expr: &'static str,
    pub test: TestType,
}

pub struct Function {
    pub name: &'static str,
    pub deps: &'static [&'static str],
    pub num_terms: [usize; 2],
    pub gen: Option<fn(num_terms: usize, config: &Config) -> TokenStream>,
    pub test_specs: &'static [TestSpec],
}

pub fn get_functions_and_deps(names: &Vec<String>) -> Vec<&Function> {
    let mut res: Vec<&Function> = Vec::new();
    let mut stack = names.clone();
    while !stack.is_empty() {
        let name = stack.pop().unwrap();
        if let Some(f) = FUNCTIONS.iter().find(|f| f.name == name) {
            if !res.iter().any(|f| f.name == name) {
                // New function, push it.
                res.push(f);

                // Add its dependencies.
                stack.extend(f.deps.iter().map(|d| d.to_string()));
            }
        } else {
            eprintln!("function {} not found", name);
        }
    }
    res.sort_by(|a, b| (*a as *const Function as usize).cmp(&(*b as *const Function as usize)));
    res
}

pub static FUNCTIONS: &[Function] = &[
    Function {
        name: "fty",
        deps: &[],
        num_terms: [0, 0],
        gen: Some(crate::auxfuncs::gen_fty),
        test_specs: &[],
    },
    Function {
        name: "uty",
        deps: &[],
        num_terms: [0, 0],
        gen: Some(crate::auxfuncs::gen_uty),
        test_specs: &[],
    },
    Function {
        name: "ity",
        deps: &[],
        num_terms: [0, 0],
        gen: Some(crate::auxfuncs::gen_ity),
        test_specs: &[],
    },
    Function {
        name: "select",
        deps: &[],
        num_terms: [0, 0],
        gen: Some(crate::auxfuncs::gen_select),
        test_specs: &[],
    },
    Function {
        name: "PI",
        deps: &[],
        num_terms: [0, 0],
        gen: Some(crate::auxfuncs::gen_PI),
        test_specs: &[],
    },
    Function {
        name: "RECIP_2PI",
        deps: &[],
        num_terms: [0, 0],
        gen: Some(crate::auxfuncs::gen_RECIP_2PI),
        test_specs: &[],
    },
    Function {
        name: "negate_on_odd",
        deps: &["fty"],
        num_terms: [0, 0],
        gen: Some(crate::auxfuncs::gen_negate_on_odd),
        test_specs: &[],
    },
    Function {
        name: "recip_approx",
        deps: &["fty"],
        num_terms: [0, 0],
        gen: Some(crate::auxfuncs::gen_recip_approx),
        test_specs: &[],
    },
    Function {
        name: "sqrt_approx",
        deps: &["fty"],
        num_terms: [0, 0],
        gen: Some(crate::auxfuncs::gen_sqrt_approx),
        test_specs: &[],
    },
    Function {
        name: "cbrt_approx",
        deps: &["fty"],
        num_terms: [0, 0],
        gen: Some(crate::auxfuncs::gen_cbrt_approx),
        test_specs: &[],
    },
    Function {
        name: "nextafter",
        deps: &["fty"],
        num_terms: [0, 0],
        gen: Some(crate::auxfuncs::gen_nextafter),
        test_specs: &[TestSpec {
            test_name: "test_nextafter",
            ref_expr: "x",
            rust_expr: "nextafter(x)",
            test: TestType::MaxAbs("0", "1", 1.0, 1.0, 237),
        }],
    },
    Function {
        name: "sinh",
        deps: &["fty"],
        num_terms: [16, 24],
        gen: Some(crate::hyperbolic::gen_sinh),
        test_specs: &[],
    },
    Function {
        name: "cosh",
        deps: &["fty"],
        num_terms: [16, 24],
        gen: Some(crate::hyperbolic::gen_cosh),
        test_specs: &[],
    },
    Function {
        name: "tanh",
        deps: &["fty"],
        num_terms: [16, 24],
        gen: Some(crate::hyperbolic::gen_tanh),
        test_specs: &[],
    },
    Function {
        name: "asinh",
        deps: &["fty"],
        num_terms: [16, 24],
        gen: Some(crate::hyperbolic::gen_asinh),
        test_specs: &[],
    },
    Function {
        name: "acosh",
        deps: &["fty"],
        num_terms: [16, 24],
        gen: Some(crate::hyperbolic::gen_acosh),
        test_specs: &[],
    },
    Function {
        name: "atanh",
        deps: &["fty"],
        num_terms: [16, 24],
        gen: Some(crate::hyperbolic::gen_atanh),
        test_specs: &[],
    },
    Function {
        name: "atan2",
        deps: &["fty"],
        num_terms: [16, 24],
        gen: Some(crate::inv_trig::gen_atan2),
        test_specs: &[],
    },
    Function {
        name: "asin",
        deps: &["fty", "select"],
        num_terms: [16, 24],
        gen: Some(crate::inv_trig::gen_asin),
        test_specs: &[],
    },
    Function {
        name: "acos",
        deps: &["fty", "select"],
        num_terms: [16, 24],
        gen: Some(crate::inv_trig::gen_acos),
        test_specs: &[],
    },
    Function {
        name: "atan",
        deps: &["fty", "select"],
        num_terms: [16, 24],
        gen: Some(crate::inv_trig::gen_atan),
        test_specs: &[],
    },
    Function {
        name: "atan2",
        deps: &["fty", "select"],
        num_terms: [16, 24],
        gen: Some(crate::inv_trig::gen_atan2),
        test_specs: &[],
    },
    Function {
        name: "exp",
        deps: &["fty", "exp2"],
        num_terms: [16, 24],
        gen: Some(crate::log_exp::gen_exp),
        test_specs: &[],
    },
    Function {
        name: "exp2",
        deps: &["fty"],
        num_terms: [16, 24],
        gen: Some(crate::log_exp::gen_exp2),
        test_specs: &[],
    },
    Function {
        name: "exp_m1",
        deps: &["fty", "exp2"],
        num_terms: [16, 24],
        gen: Some(crate::log_exp::gen_exp_m1),
        test_specs: &[],
    },
    Function {
        name: "ln_1p",
        deps: &["fty"],
        num_terms: [16, 24],
        gen: Some(crate::log_exp::gen_ln_1p),
        test_specs: &[],
    },
    Function {
        name: "ln",
        deps: &["fty", "log2"],
        num_terms: [16, 24],
        gen: Some(crate::log_exp::gen_ln),
        test_specs: &[],
    },
    Function {
        name: "log10",
        deps: &["fty", "log2"],
        num_terms: [16, 24],
        gen: Some(crate::log_exp::gen_log10),
        test_specs: &[],
    },
    Function {
        name: "log2",
        deps: &["fty"],
        num_terms: [16, 24],
        gen: Some(crate::log_exp::gen_log2),
        test_specs: &[],
    },
    Function {
        name: "log",
        deps: &["fty", "log2"],
        num_terms: [16, 24],
        gen: Some(crate::log_exp::gen_log),
        test_specs: &[],
    },
    Function {
        name: "powf",
        deps: &["fty", "log2", "exp2"],
        num_terms: [16, 24],
        gen: Some(crate::log_exp::gen_powf),
        test_specs: &[],
    },
    Function {
        name: "powi",
        deps: &["fty", "log2", "exp2"],
        num_terms: [16, 24],
        gen: Some(crate::log_exp::gen_powi),
        test_specs: &[],
    },
    Function {
        name: "sqrt",
        deps: &["fty", "sqrt_approx"],
        num_terms: [16, 24],
        gen: Some(crate::recip_sqrt::gen_sqrt),
        test_specs: &[],
    },
    Function {
        name: "cbrt",
        deps: &["fty", "cbrt_approx"],
        num_terms: [16, 24],
        gen: Some(crate::recip_sqrt::gen_cbrt),
        test_specs: &[],
    },
    Function {
        name: "recip",
        deps: &["fty", "recip_approx"],
        num_terms: [16, 24],
        gen: Some(crate::recip_sqrt::gen_recip),
        test_specs: &[],
    },
    Function {
        name: "hypot",
        deps: &["fty"],
        num_terms: [16, 24],
        gen: Some(crate::recip_sqrt::gen_hypot),
        test_specs: &[],
    },
    Function {
        name: "runif",
        deps: &["fty"],
        num_terms: [0, 0],
        gen: Some(crate::stats_random::gen_runif),
        test_specs: &[],
    },
    Function {
        name: "dnorm",
        deps: &["fty"],
        num_terms: [16, 24],
        gen: Some(crate::stats_norm::gen_dnorm),
        test_specs: &[],
    },
    Function {
        name: "pnorm",
        deps: &["fty"],
        num_terms: [16, 24],
        gen: Some(crate::stats_norm::gen_pnorm),
        test_specs: &[],
    },
    Function {
        name: "qnorm",
        deps: &["fty"],
        num_terms: [16, 24],
        gen: Some(crate::stats_norm::gen_qnorm),
        test_specs: &[],
    },
    Function {
        name: "rnorm",
        deps: &["fty", "runif", "qnorm"],
        num_terms: [16, 24],
        gen: Some(crate::stats_norm::gen_rnorm),
        test_specs: &[],
    },
    Function {
        name: "sin",
        deps: &["fty", "RECIP_2PI"],
        num_terms: [12, 24],
        gen: Some(crate::trig::gen_sin),
        test_specs: &[
            TestSpec {
                test_name: "test_sin",
                ref_expr: "x.sin()",
                rust_expr: "sin(x)",
                test: TestType::MaxAbs("-PI", "PI", 6.0, 6.0, 237),
            },
            TestSpec {
                test_name: "test_sin2",
                ref_expr: "x.sin()",
                rust_expr: "sin(x)",
                test: TestType::MaxAbs("-PI/2", "PI/2", 2.0, 2.0, 237),
            },
            TestSpec {
                test_name: "test_sin3",
                ref_expr: "x.sin()",
                rust_expr: "sin(x)",
                test: TestType::MaxAbs("-PI/4", "PI/4", 1.0, 1.0, 237),
            },
        ],
    },
    Function {
        name: "cos",
        deps: &["fty", "RECIP_2PI"],
        num_terms: [13, 25],
        gen: Some(crate::trig::gen_cos),
        test_specs: &[
            TestSpec {
                test_name: "test_cos",
                ref_expr: "x.cos()",
                rust_expr: "cos(x)",
                test: TestType::MaxAbs("-PI", "PI", 6.0, 8.0, 237),
            },
            TestSpec {
                test_name: "test_cos2",
                ref_expr: "x.cos()",
                rust_expr: "cos(x)",
                test: TestType::MaxAbs("-PI/2", "PI/2", 2.0, 2.0, 237),
            },
            TestSpec {
                test_name: "test_cos3",
                ref_expr: "x.cos()",
                rust_expr: "cos(x)",
                test: TestType::MaxAbs("-PI/4", "PI/4", 1.0, 1.0, 237),
            },
        ],
    },
    Function {
        name: "tan",
        deps: &["fty", "PI", "select"],
        num_terms: [16, 24],
        gen: Some(crate::trig::gen_tan),
        test_specs: &[],
    },
    Function {
        name: "sin_cos",
        deps: &["fty", "sin", "cos"],
        num_terms: [16, 24],
        gen: Some(crate::trig::gen_sin_cos),
        test_specs: &[],
    },
    Function {
        name: "invtrig",
        deps: &["fty", "asin", "acos", "atan"],
        num_terms: [16, 24],
        gen: None,
        test_specs: &[],
    },
    Function {
        name: "trig",
        deps: &["fty", "sin", "cos", "tan", "invtrig"],
        num_terms: [16, 24],
        gen: None,
        test_specs: &[],
    },
    Function {
        name: "invhyperbolic",
        deps: &["fty", "asinh", "acosh", "atanh"],
        num_terms: [16, 24],
        gen: None,
        test_specs: &[],
    },
    Function {
        name: "hyperbolic",
        deps: &["fty", "sinh", "cosh", "tanh", "invhyperbolic"],
        num_terms: [16, 24],
        gen: None,
        test_specs: &[],
    },
    Function {
        name: "logexp",
        deps: &["fty", "exp", "ln", "exp2", "log2"],
        num_terms: [16, 24],
        gen: None,
        test_specs: &[],
    },
    Function {
        name: "recip_sqrt",
        deps: &["fty", "recip", "sqrt", "cbrt"],
        num_terms: [16, 24],
        gen: None,
        test_specs: &[],
    },
    Function {
        name: "libm",
        deps: &["fty", "logexp", "trig", "hyperbolic", "recip_sqrt"],
        num_terms: [16, 24],
        gen: None,
        test_specs: &[],
    },
    Function {
        name: "normal",
        deps: &["fty", "rnorm", "dnorm", "pnorm", "qnorm"],
        num_terms: [16, 24],
        gen: None,
        test_specs: &[],
    },
    Function {
        name: "stats",
        deps: &["fty", "normal"],
        num_terms: [16, 24],
        gen: None,
        test_specs: &[],
    },
    Function {
        name: "all",
        deps: &["fty", "stats", "libm"],
        num_terms: [16, 24],
        gen: None,
        test_specs: &[],
    },
];
