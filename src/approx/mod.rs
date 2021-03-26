//! Mathematical function generation.
#![allow(dead_code)]

use super::interpreter::{Domain, Interpreter, Variable};
use super::polynomial::Polynomial;

use proc_macro::TokenStream;

use proc_macro2::Span;
use syn::{parse_quote, ItemFn, Lit, Meta, NestedMeta, Stmt};

use quote::{quote, quote_spanned};

struct Args {
    xmin: f64,
    xmax: f64,
    terms: usize,
}

fn error_stream<T: AsRef<str>>(msg: T) -> TokenStream {
    let msg = msg.as_ref();
    return TokenStream::from(quote! { compile_error!(#msg); });
}

fn error_stream_spanned<T: AsRef<str>>(span: Span, msg: T) -> TokenStream {
    let msg = msg.as_ref();
    return TokenStream::from(quote_spanned! {span=> compile_error!(#msg); });
}

fn parse_args(args: Vec<NestedMeta>) -> Result<Args, TokenStream> {
    let mut res = Args {
        xmin: 0.0,
        xmax: 1.0,
        terms: 6,
    };
    for meta in &args {
        match meta {
            NestedMeta::Meta(Meta::NameValue(nv)) => match nv.path.get_ident() {
                None => return Err(error_stream("Expected identifier.")),
                Some(ident) => match (ident.to_string().as_str(), &nv.lit) {
                    ("xmin", Lit::Float(f)) => {
                        let span = ident.span();
                        res.xmin = f
                            .base10_parse()
                            .map_err(|_| error_stream_spanned(span, "parse error"))?
                    }
                    ("xmax", Lit::Float(f)) => {
                        let span = ident.span();
                        res.xmax = f
                            .base10_parse()
                            .map_err(|_| error_stream_spanned(span, "parse error"))?
                    }
                    ("terms", Lit::Int(f)) => {
                        let span = ident.span();
                        res.terms = f
                            .base10_parse()
                            .map_err(|_| error_stream_spanned(span, "parse error"))?
                    }
                    (id, _) => {
                        return Err(error_stream(format!(
                            "Unknown option or wrong type {}.",
                            id
                        )))
                    }
                },
            },
            _ => {
                return Err(error_stream(
                    "Expected #[extendr_approx(xmin=f, xmax=f, terms=n)",
                ))
            }
        }
    }

    if res.xmin >= res.xmax || res.terms > 30 {
        return Err(error_stream("Expected xmin >= xmax and <= 30 terms."));
    }

    Ok(res)
}

// Only accept f64.
fn type_is_ok(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Path(typepath) => {
            if typepath.path.is_ident("f64") {
                return true;
            }
        }
        _ => (),
    }
    false
}

// Only accept x: f64
fn arg_is_ok(arg: &syn::FnArg) -> bool {
    if let syn::FnArg::Typed(syn::PatType { pat, ty, .. }) = arg {
        if let syn::Pat::Ident(syn::PatIdent { ref ident, .. }) = **pat {
            if type_is_ok(ty) {
                if ident == "x" {
                    return true;
                }
            }
        }
    }
    false
}

fn generate_function(args: Args, func: ItemFn) -> TokenStream {
    match &func.sig.output {
        syn::ReturnType::Type(_, ref ty) if type_is_ok(ty) => (),
        _ => return error_stream("Function must return f64."),
    }

    for arg in &func.sig.inputs {
        if !arg_is_ok(arg) {
            return error_stream("Arg must be x: f64.");
        }
    }

    use std::f64::consts::PI;
    let a = (args.xmax + args.xmin) * 0.5;
    let b = PI / (args.terms - 1) as f64;
    let c = (args.xmax - args.xmin) * 0.5;
    let mut xvalues = Vec::new();
    let mut yvalues = Vec::new();
    for i in 0..args.terms {
        // *almost* Chebyshev nodes.
        let x = a - c * (i as f64 * b).cos();
        let y = Interpreter::new_x(x).block(&func.block).unwrap();
        println!("{} {}", x, y);
        xvalues.push(x);
        yvalues.push(y);
    }

    let poly = Polynomial::from_points(xvalues.as_slice(), yvalues.as_slice());
    let k = args.terms;
    let terms = poly.terms();
    let func_sig = func.sig;
    let mut stmts: Vec<Stmt> = Vec::new();
    let tk = terms[k - 1];
    stmts.push(parse_quote!(let y = #tk;));
    for i in (0..k - 1).rev() {
        let ti = terms[i];
        stmts.push(parse_quote!(let y = y.mul_add(x, #ti);));
    }
    println!(
        "{}",
        TokenStream::from(quote!(
            #func_sig {
                #( #stmts )*
                y
            }
        ))
    );
    return TokenStream::from(quote!(
        #func_sig {
            #( #stmts )*
            y
        }
    ));
}

fn parse_domain(attr: &syn::Attribute, domain: &mut Domain) -> Result<(), TokenStream> {
    let meta = attr.parse_meta().map_err(|e| {
        error_stream_spanned(
            attr.pound_token.span,
            format!("{}", e).as_str(),
        )
    })?;

    // println!("meta={:#?}", meta);

    match meta {
        syn::Meta::List(list) => {
            for nm in &list.nested {
                if let syn::NestedMeta::Meta(syn::Meta::NameValue(nv)) = nm {
                    let ident = if let Some(ident) = nv.path.get_ident() {
                        Ok(ident)
                    } else {
                        Err(error_stream_spanned(
                            attr.pound_token.span,
                            "Expected name=value.",
                        ))
                    }?;

                    let val: f64 = match &nv.lit {
                        syn::Lit::Float(f) => f.base10_parse().map_err(|e| error_stream_spanned(nv.lit.span(), "expected number"))?,
                        syn::Lit::Int(f) => f.base10_parse().map_err(|e| error_stream_spanned(nv.lit.span(), "expected number"))?,
                        _ => Err(error_stream_spanned(nv.lit.span(), "expected number"))?,
                    };

                    println!("{} {}", ident, val);
                    match ident.to_string().as_str() {
                        "min" => {
                            domain.min = Some(val);
                        }
                        "max" => {
                            domain.max = Some(val);
                        }
                        "terms" => {
                            domain.terms = Some(val as usize);
                        }
                        _ => {
                            return Err(error_stream_spanned(
                                ident.span(),
                                "expected #[domain(min=x, max=y, terms=z)]"));
                        }
                    }
                } else {
                    return Err(error_stream_spanned(
                        attr.pound_token.span,
                        "expected #[domain(min=x, max=y, terms=z)]"));
                }
            }
        }
        _ => return Err(error_stream_spanned(
            attr.pound_token.span,
            "expected #[domain(min=x, max=y, terms=z)]")),
    }


    // for nv in nvpairs {
    // }

    // for arg in &meta {
    //     let nv = if let Meta::NameValue(nv) = meta {
    //         Ok(nv)
    //     } else {
    //         Err(error_stream_spanned(
    //             attr.pound_token.span,
    //             "Expected name=value.",
    //         ))
    //     }?;

    //     let ident = if let Some(ident) = nv.path.get_ident() {
    //         Ok(ident)
    //     } else {
    //         Err(error_stream_spanned(
    //             attr.pound_token.span,
    //             "Expected name=value.",
    //         ))
    //     }?;

    //     let val: f64 = match nv.lit {
    //         syn::Lit::Float(f) => f.base10_parse(),
    //         syn::Lit::Int(f) => f.base10_parse(),
    //         _ => error_stream_spanned(nv.lit.span(), "expected number")?,
    //     };

    //     match ident.to_string().as_str() {
    //         "min" => {
    //             domain.min = val;
    //         }
    //         "max" => {
    //             domain.max = val;
    //         }
    //         "terms" => {
    //             domain.terms = val as usize;
    //         }
    //     }
    // }
    Err(error_stream( "zzz"))

    // let nv = match meta {
    //     Meta::NameValue(nv) => nv.path.get_ident().or_else(
    //         Err(error_stream("Expected identifier.")

    //     {
    //         None => Err(error_stream("Expected identifier."),
    //         Some(ident) => match (ident.to_string().as_str(), &nv.lit) {
    //             ("min", Lit::Float(f)) => {
    //                 let span = ident.span();
    //                 domain.min = f
    //                     .base10_parse()
    //                     .map_err(|_| error_stream_spanned(span, "parse error"))?
    //             }
    //             ("max", Lit::Float(f)) => {
    //                 let span = ident.span();
    //                 domain.max = f
    //                     .base10_parse()
    //                     .map_err(|_| error_stream_spanned(span, "parse error"))?
    //             }
    //             ("terms", Lit::Int(f)) => {
    //                 let span = ident.span();
    //                 domain.terms = f
    //                     .base10_parse()
    //                     .map_err(|_| error_stream_spanned(span, "parse error"))?
    //             }
    //             (id, _) => {
    //                 return Err(error_stream(format!(
    //                     "Unknown option or wrong type {}.",
    //                     id
    //                 )))
    //             }
    //         },
    //     _ =>
    // }
}

pub fn do_approx(clos: syn::ExprClosure) -> Result<TokenStream, TokenStream> {
    println!("{:#?}", clos);

    let variables: Vec<Variable> = Vec::new();
    for arg in &clos.inputs {
        match arg {
            syn::Pat::Ident(ref id) => {
                let mut var = Variable::default();
                for attr in &id.attrs {
                    if attr.path.is_ident("domain") {
                        parse_domain(attr, &mut var.domain)?;
                    } else {
                        return Err(error_stream_spanned(
                            attr.pound_token.span,
                            "expected domain attribute",
                        ))
                    }
                }
                // pub attrs: Vec<Attribute>,
                // pub by_ref: Option<Token![ref]>,
                // pub mutability: Option<Token![mut]>,
                // pub ident: Ident,
                // pub subpat: Option<(Token![@], Box<Pat>)>,
            }
            _ => return Err(error_stream_spanned(clos.or1_token.span, "expected variable name")),
        }
    }

    // match parse_args(args) {
    //     Ok(args) => generate_function(args, func),
    //     Err(e) => e,
    // }
    Err(error_stream_spanned(clos.or1_token.span, "ouch"))
}
