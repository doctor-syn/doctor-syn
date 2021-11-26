use crate::config::Config;
use proc_macro2::TokenStream;

pub struct Function {
    pub name: &'static str,
    pub deps: &'static [&'static str],
    pub num_terms: [usize; 2],
    pub gen: Option<fn(num_terms: usize, config: &Config) -> TokenStream>,
    pub gen_test: Option<fn(config: &Config) -> TokenStream>,
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
    res
}

pub static FUNCTIONS: &[Function] = &[
    Function {
        name: "negate_on_odd",
        deps: &[],
        num_terms: [16, 24],
        gen: None,
        gen_test: None,
    },
    Function {
        name: "recip_approx",
        deps: &[],
        num_terms: [16, 24],
        gen: None,
        gen_test: None,
    },
    Function {
        name: "sqrt_approx",
        deps: &[],
        num_terms: [16, 24],
        gen: None,
        gen_test: None,
    },
    Function {
        name: "cbrt_approx",
        deps: &[],
        num_terms: [16, 24],
        gen: None,
        gen_test: None,
    },
    Function {
        name: "sinh",
        deps: &[],
        num_terms: [16, 24],
        gen: None,
        gen_test: None,
    },
    Function {
        name: "cosh",
        deps: &[],
        num_terms: [16, 24],
        gen: None,
        gen_test: None,
    },
    Function {
        name: "tanh",
        deps: &[],
        num_terms: [16, 24],
        gen: None,
        gen_test: None,
    },
    Function {
        name: "asinh",
        deps: &[],
        num_terms: [16, 24],
        gen: None,
        gen_test: None,
    },
    Function {
        name: "acosh",
        deps: &[],
        num_terms: [16, 24],
        gen: None,
        gen_test: None,
    },
    Function {
        name: "atanh",
        deps: &[],
        num_terms: [16, 24],
        gen: None,
        gen_test: None,
    },
    Function {
        name: "atan2",
        deps: &[],
        num_terms: [16, 24],
        gen: None,
        gen_test: None,
    },
    Function {
        name: "asin",
        deps: &[],
        num_terms: [16, 24],
        gen: None,
        gen_test: None,
    },
    Function {
        name: "acos",
        deps: &[],
        num_terms: [16, 24],
        gen: None,
        gen_test: None,
    },
    Function {
        name: "atan",
        deps: &[],
        num_terms: [16, 24],
        gen: None,
        gen_test: None,
    },
    Function {
        name: "exp",
        deps: &["exp2"],
        num_terms: [16, 24],
        gen: None,
        gen_test: None,
    },
    Function {
        name: "exp2",
        deps: &[],
        num_terms: [16, 24],
        gen: None,
        gen_test: None,
    },
    Function {
        name: "exp_m1",
        deps: &["exp2"],
        num_terms: [16, 24],
        gen: None,
        gen_test: None,
    },
    Function {
        name: "ln_1p",
        deps: &[],
        num_terms: [16, 24],
        gen: None,
        gen_test: None,
    },
    Function {
        name: "ln",
        deps: &["log2"],
        num_terms: [16, 24],
        gen: None,
        gen_test: None,
    },
    Function {
        name: "log10",
        deps: &["log2"],
        num_terms: [16, 24],
        gen: None,
        gen_test: None,
    },
    Function {
        name: "log2",
        deps: &[],
        num_terms: [16, 24],
        gen: None,
        gen_test: None,
    },
    Function {
        name: "log",
        deps: &["log2"],
        num_terms: [16, 24],
        gen: None,
        gen_test: None,
    },
    Function {
        name: "powf",
        deps: &["log2", "exp2"],
        num_terms: [16, 24],
        gen: None,
        gen_test: None,
    },
    Function {
        name: "powi",
        deps: &["log2", "exp2"],
        num_terms: [16, 24],
        gen: None,
        gen_test: None,
    },
    Function {
        name: "sqrt",
        deps: &[],
        num_terms: [16, 24],
        gen: None,
        gen_test: None,
    },
    Function {
        name: "cbrt",
        deps: &[],
        num_terms: [16, 24],
        gen: None,
        gen_test: None,
    },
    Function {
        name: "recip",
        deps: &[],
        num_terms: [16, 24],
        gen: None,
        gen_test: None,
    },
    Function {
        name: "hypot",
        deps: &[],
        num_terms: [16, 24],
        gen: None,
        gen_test: None,
    },
    Function {
        name: "qnorm",
        deps: &[],
        num_terms: [16, 24],
        gen: None,
        gen_test: None,
    },
    Function {
        name: "sin",
        deps: &[],
        num_terms: [16, 24],
        gen: Some(crate::trig::gen_single_pass_sin),
        gen_test: None,
    },
    Function {
        name: "cos",
        deps: &[],
        num_terms: [17, 25],
        gen: Some(crate::trig::gen_single_pass_cos),
        gen_test: None,
    },
    Function {
        name: "tan",
        deps: &[],
        num_terms: [16, 24],
        gen: None,
        gen_test: None,
    },
    Function {
        name: "sin_cos",
        deps: &["sin", "cos"],
        num_terms: [16, 24],
        gen: None,
        gen_test: None,
    },
    Function {
        name: "invtrig",
        deps: &["asin", "acos", "atan"],
        num_terms: [16, 24],
        gen: None,
        gen_test: None,
    },
    Function {
        name: "trig",
        deps: &["sin", "cos", "tan", "invtrig"],
        num_terms: [16, 24],
        gen: None,
        gen_test: None,
    },
    Function {
        name: "invhyperbolic",
        deps: &["asinh", "acosh", "atanh"],
        num_terms: [16, 24],
        gen: None,
        gen_test: None,
    },
    Function {
        name: "hyperbolic",
        deps: &["sinh", "cosh", "tanh", "invhyperbolic"],
        num_terms: [16, 24],
        gen: None,
        gen_test: None,
    },
    Function {
        name: "logexp",
        deps: &["exp", "ln", "exp2", "log2"],
        num_terms: [16, 24],
        gen: None,
        gen_test: None,
    },
    Function {
        name: "recip_sqrt",
        deps: &["recip", "sqrt", "cbrt"],
        num_terms: [16, 24],
        gen: None,
        gen_test: None,
    },
    Function {
        name: "libm",
        deps: &["logexp", "trig", "hyperbolic", "recip_sqrt"],
        num_terms: [16, 24],
        gen: None,
        gen_test: None,
    },
    Function {
        name: "normal",
        deps: &["rnorm", "dnorm", "pnorm", "qnorm"],
        num_terms: [16, 24],
        gen: None,
        gen_test: None,
    },
    Function {
        name: "stats",
        deps: &["normal"],
        num_terms: [16, 24],
        gen: None,
        gen_test: None,
    },
    Function {
        name: "all",
        deps: &["stats", "libm"],
        num_terms: [16, 24],
        gen: None,
        gen_test: None,
    },
];
