use crate::config::Config;
use proc_macro2::TokenStream;
use std::f64::consts::PI;

pub enum TestType {
    MaxAbs(&'static str, &'static str, f64, f64, usize),
    Histogram(&'static str, &'static str),
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
        name: "iabs",
        deps: &[],
        num_terms: [0, 0],
        gen: Some(crate::auxfuncs::gen_iabs),
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
        name: "LOG2_E",
        deps: &[],
        num_terms: [0, 0],
        gen: Some(crate::auxfuncs::gen_LOG2_E),
        test_specs: &[],
    },
    Function {
        name: "RECIP_LOG2_E",
        deps: &[],
        num_terms: [0, 0],
        gen: Some(crate::auxfuncs::gen_RECIP_LOG2_E),
        test_specs: &[],
    },
    Function {
        name: "RECIP_LOG2_10",
        deps: &[],
        num_terms: [0, 0],
        gen: Some(crate::auxfuncs::gen_RECIP_LOG2_10),
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
        name: "SQRT_RECIP_2PI",
        deps: &[],
        num_terms: [0, 0],
        gen: Some(crate::auxfuncs::gen_SQRT_RECIP_2PI),
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
        name: "exp2",
        deps: &["fty", "ity", "uty"],
        num_terms: [8, 24],
        gen: Some(crate::log_exp::gen_exp2),
        test_specs: &[
            TestSpec {
                test_name: "test_exp2_1",
                ref_expr: "x.exp2()",
                rust_expr: "exp2(x)",
                test: TestType::MaxAbs("-1", "0", 1.0, 1.0, 237),
            },
            TestSpec {
                test_name: "test_exp2_2",
                ref_expr: "x.exp2()",
                rust_expr: "exp2(x)",
                test: TestType::MaxAbs("0", "1", 2.0, 2.0, 237),
            },
        ],
    },
    Function {
        name: "exp",
        deps: &["fty", "exp2", "LOG2_E"],
        num_terms: [16, 24],
        gen: Some(crate::log_exp::gen_exp),
        test_specs: &[
            TestSpec {
                test_name: "test_exp_1",
                ref_expr: "x.exp()",
                rust_expr: "exp(x)",
                test: TestType::MaxAbs("-1", "0", 1.0, 1.0, 237),
            },
            TestSpec {
                test_name: "test_exp_2",
                ref_expr: "x.exp()",
                rust_expr: "exp(x)",
                test: TestType::MaxAbs("0", "1", 4.0, 4.0, 237),
            },
            TestSpec {
                test_name: "test_exp_3",
                ref_expr: "x.exp()",
                rust_expr: "exp(x)",
                test: TestType::MaxAbs("1", "2", 16.0, 16.0, 237),
            },
        ],
    },
    Function {
        name: "exp_m1",
        deps: &["fty", "exp2"],
        num_terms: [16, 24],
        gen: Some(crate::log_exp::gen_exp_m1),
        test_specs: &[
            TestSpec {
                test_name: "test_exp_m1_1",
                ref_expr: "x.exp() - 1",
                rust_expr: "exp_m1(x)",
                test: TestType::MaxAbs("-1", "0", 1.0, 1.0, 237),
            },
            TestSpec {
                test_name: "test_exp_m1_2",
                ref_expr: "x.exp() - 1",
                rust_expr: "exp_m1(x)",
                test: TestType::MaxAbs("0", "1", 4.0, 4.0, 237),
            },
        ],
    },
    Function {
        name: "log2",
        deps: &["fty"],
        num_terms: [16, 24],
        gen: Some(crate::log_exp::gen_log2),
        test_specs: &[
            TestSpec {
                test_name: "test_log2_1",
                ref_expr: "x.log2()",
                rust_expr: "log2(x)",
                test: TestType::MaxAbs("0.7", "1.0", 1.5, 1.5, 237),
            },
            TestSpec {
                test_name: "test_log2_2",
                ref_expr: "x.log2()",
                rust_expr: "log2(x)",
                test: TestType::MaxAbs("1.0", "2", 2.0, 2.0, 237),
            },
        ],
    },
    Function {
        name: "ln_1p",
        deps: &["fty", "RECIP_LOG2_E", "select"],
        num_terms: [16, 24],
        gen: Some(crate::log_exp::gen_ln_1p),
        test_specs: &[
            TestSpec {
                test_name: "test_ln_1p_1",
                ref_expr: "(1+x).ln()",
                rust_expr: "ln_1p(x)",
                test: TestType::MaxAbs("1", "2", 2.0, 2.0, 237),
            },
        ],
    },
    Function {
        name: "ln",
        deps: &["log2", "RECIP_LOG2_E"],
        num_terms: [16, 24],
        gen: Some(crate::log_exp::gen_ln),
        test_specs: &[
            TestSpec {
                test_name: "test_ln_1",
                ref_expr: "x.ln()",
                rust_expr: "ln(x)",
                test: TestType::MaxAbs("0.7", "1.0", 1.5, 1.5, 237),
            },
            TestSpec {
                test_name: "test_ln_2",
                ref_expr: "x.ln()",
                rust_expr: "ln(x)",
                test: TestType::MaxAbs("1.0", "2", 2.0, 2.0, 237),
            },
        ],
    },
    Function {
        name: "log10",
        deps: &["log2", "RECIP_LOG2_10"],
        num_terms: [16, 24],
        gen: Some(crate::log_exp::gen_log10),
        test_specs: &[
            TestSpec {
                test_name: "test_log10_1",
                ref_expr: "x.log10()",
                rust_expr: "log10(x)",
                test: TestType::MaxAbs("0.1", "1.0", 1.5, 1.5, 237),
            },
            TestSpec {
                test_name: "test_log10_2",
                ref_expr: "x.log10()",
                rust_expr: "log10(x)",
                test: TestType::MaxAbs("1.0", "10", 2.0, 2.0, 237),
            },
        ],
    },
    Function {
        name: "log",
        deps: &["fty", "log2"],
        num_terms: [16, 24],
        gen: Some(crate::log_exp::gen_log),
        test_specs: &[
            TestSpec {
                test_name: "test_log_1",
                ref_expr: "x.log10()",
                rust_expr: "log(x, 10.0 as fty)",
                test: TestType::MaxAbs("0.1", "1.0", 1.5, 1.5, 237),
            },
            TestSpec {
                test_name: "test_log_2",
                ref_expr: "x.log10()",
                rust_expr: "log(x, 10.0 as fty)",
                test: TestType::MaxAbs("1.0", "10", 2.0, 2.0, 237),
            },
        ],
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
        deps: &["fty", "log2", "exp2", "select", "recip", "iabs"],
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
        test_specs: &[
            TestSpec {
                test_name: "test_runif",
                ref_expr: "1",
                rust_expr: "runif(i, 0.0, 1.0)",
                test: TestType::Histogram("0.0", "1.0"),
            },
        ],
    },
    Function {
        name: "dnorm",
        deps: &["fty", "LOG2_E", "SQRT_RECIP_2PI", "recip", "exp2"],
        num_terms: [16, 24],
        gen: Some(crate::stats_norm::gen_dnorm),
        test_specs: &[
            TestSpec {
                test_name: "test_dnorm_1",
                ref_expr: "x.dnorm(0, 1)",
                rust_expr: "dnorm(x, 0.0 as fty, 1.0 as fty)",
                test: TestType::MaxAbs("-1", "1", 1.0, 1.0, 237),
            },
        ],
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
        deps: &["fty", "sqrt", "log2"],
        num_terms: [16, 32],
        gen: Some(crate::stats_norm::gen_qnorm),
        test_specs: &[
            TestSpec {
                test_name: "test_qnorm_1",
                ref_expr: "x.qnorm(0, 1)",
                rust_expr: "qnorm(x, 0.0 as fty, 1.0 as fty)",
                test: TestType::MaxAbs("0.0001", "0.999", 1e6, 1e6, 237),
            },
            // TestSpec {
            //     test_name: "test_qnorm_1",
            //     ref_expr: "x.qnorm(0, 1)",
            //     rust_expr: "qnorm(x, 0.0 as fty, 1.0 as fty)",
            //     test: TestType::MaxAbs("0.25", "0.75", 2.0, 2.0, 237),
            // },
            // TestSpec {
            //     test_name: "test_qnorm_2",
            //     ref_expr: "x.qnorm(0, 1)",
            //     rust_expr: "qnorm(x, 0.0 as fty, 1.0 as fty)",
            //     test: TestType::MaxAbs("0.01", "0.25", 2.0, 1.0, 237),
            // },
        ],
    },
    Function {
        name: "rnorm",
        deps: &["fty", "runif", "qnorm"],
        num_terms: [16, 24],
        gen: Some(crate::stats_norm::gen_rnorm),
        test_specs: &[
            TestSpec {
                test_name: "test_rnorm",
                ref_expr: "dnorm(x, 0.0, 1.0)",
                rust_expr: "rnorm(i, 0.0, 1.0)",
                test: TestType::Histogram("-2.0", "2.0"),
            },
        ],
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
                test: TestType::MaxAbs("-PI", "PI", 9.0, 9.0, 1024),
            },
            TestSpec {
                test_name: "test_sini",
                ref_expr: "x.sin()",
                rust_expr: "sin(x)",
                test: TestType::MaxAbs("-1.0", "1.0", 2.0, 2.0, 1024),
            },
            TestSpec {
                test_name: "test_sin2",
                ref_expr: "x.sin()",
                rust_expr: "sin(x)",
                test: TestType::MaxAbs("-PI/2", "PI/2", 3.0, 3.0, 1024),
            },
            TestSpec {
                test_name: "test_sin3",
                ref_expr: "x.sin()",
                rust_expr: "sin(x)",
                test: TestType::MaxAbs("-PI/4", "PI/4", 2.0, 2.0, 1024),
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
        num_terms: [12, 24],
        gen: Some(crate::trig::gen_tan),
        test_specs: &[
            TestSpec {
                test_name: "test_tan",
                ref_expr: "x.tan()",
                rust_expr: "tan(x)",
                test: TestType::MaxAbs("-0.7", "0.7", 2.0, 2.0, 237),
            },
            TestSpec {
                test_name: "test_tan2",
                ref_expr: "x.tan()",
                rust_expr: "tan(x)",
                test: TestType::MaxAbs("-1.1", "1.1", 6.0, 6.0, 237),
            },
        ],
    },
    Function {
        name: "sin_cos",
        deps: &["fty", "sin", "cos"],
        num_terms: [0, 0],
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
        deps: &["sinh", "cosh", "tanh", "invhyperbolic"],
        num_terms: [16, 24],
        gen: None,
        test_specs: &[],
    },
    Function {
        name: "logexp",
        deps: &["exp2", "exp", "exp_m1", "log2", "ln_1p", "ln", "log10", "log", "powf", "powi"],
        num_terms: [16, 24],
        gen: None,
        test_specs: &[],
    },
    Function {
        name: "recip_sqrt",
        deps: &["recip", "sqrt", "cbrt"],
        num_terms: [16, 24],
        gen: None,
        test_specs: &[],
    },
    Function {
        name: "libm",
        deps: &["logexp", "trig", "hyperbolic", "recip_sqrt"],
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


