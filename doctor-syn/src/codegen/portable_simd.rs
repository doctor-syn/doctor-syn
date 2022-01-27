//! Convert a file of functions into a set of methods
//! suitable for rust-lang/portable-simd

use quote::format_ident;
use quote::ToTokens;
use syn::punctuated::Punctuated;
use syn::visit_mut::{visit_expr_mut, visit_ident_mut, VisitMut};
use syn::{parse_quote, Expr, Ident, Item, ItemConst, Token, Visibility};

pub struct Options {
    pub num_bits: usize,
}

impl Default for Options {
    fn default() -> Self {
        Options { num_bits: 32 }
    }
}

pub struct SimdVisitor {
    options: Options,
    consts: Vec<ItemConst>,
    idents_used: Vec<Ident>,
}

impl SimdVisitor {
    fn new(options: Options) -> Self {
        let consts = Vec::new();
        let idents_used = Vec::new();
        SimdVisitor {
            options,
            consts,
            idents_used,
        }
    }
}

fn deblock1(block: &syn::Block) -> &Expr {
    if block.stmts.len() == 1 {
        if let syn::Stmt::Expr(expr) = &block.stmts[0] {
            return expr;
        }
    }
    unreachable!();
}

fn deblock2(expr: &Expr) -> &Expr {
    if let syn::Expr::Block(block) = expr {
        if block.block.stmts.len() == 1 {
            if let syn::Stmt::Expr(expr) = &block.block.stmts[0] {
                return expr;
            }
        }
    }
    expr
}

impl VisitMut for SimdVisitor {
    fn visit_ident_mut(&mut self, i: &mut Ident) {
        match i {
            i if i == "arg" => *i = Ident::new("self", i.span()),
            i if i == "fty" => *i = Ident::new("Self", i.span()),
            _ => {
                self.idents_used.push(i.clone());
                visit_ident_mut(self, i);
            }
        }
    }

    fn visit_signature_mut(&mut self, sig: &mut syn::Signature) {
        // Patch the first argument to self
        let arg0 = sig.inputs.iter_mut().next().unwrap();
        *arg0 = parse_quote! {self};
        sig.output = parse_quote! {-> Self};
    }

    // Convert `lit as f32` etc. to splats.
    fn visit_expr_mut(&mut self, expr: &mut syn::Expr) {
        visit_expr_mut(self, expr);

        match expr {
            syn::Expr::Cast(cast) => {
                *expr = (&*cast.expr).clone();
                return;
                // if let syn::Expr::Lit(lit) = &*cast.expr {
                //     *expr = parse_quote! { Self::splat(-#lit) };
                //     return;
                // }
                // if let syn::Expr::Unary(unary) = &*cast.expr {
                //     if let syn::UnOp::Neg(_) = &unary.op {
                //         if let syn::Expr::Lit(lit) = &*unary.expr {
                //             *expr = parse_quote! { Self::splat(-#lit) };
                //             return;
                //         }
                //     }
                // }
            }
            syn::Expr::Binary(binary) => {
                use syn::BinOp::*;
                if let Some(op) = match binary.op {
                    Eq(_) => Some("lanes_eq"),
                    Lt(_) => Some("lanes_lt"),
                    Le(_) => Some("lanes_le"),
                    Ne(_) => Some("lanes_ne"),
                    Ge(_) => Some("lanes_ge"),
                    Gt(_) => Some("lanes_gt"),
                    _ => None,
                } {
                    let id = format_ident!("{}", op);
                    let lhs = binary.left.clone();
                    let rhs = binary.right.clone();
                    *expr = parse_quote! { (#lhs).#id(#rhs) };
                    return;
                }
            }
            syn::Expr::Call(call) => {
                if call.func.to_token_stream().to_string() == "select" {
                    let mut args = call.args.iter();
                    let arg0 = args.next().unwrap();
                    let rest = args.cloned().collect::<Punctuated<Expr, Token![,]>>();
                    *expr = parse_quote! { (#arg0).select(#rest) };
                    return;
                }
            }
            syn::Expr::Lit(syn::ExprLit { lit, ..} ) => {
                if let syn::Lit::Float(f) = lit {
                    match self.options.num_bits {
                        32 => {
                            let rounded: f32 = f.base10_parse().unwrap();
                            *expr = parse_quote! { Self::splat(#rounded) };
                        }
                        64 => {
                            let rounded: f64 = f.base10_parse().unwrap();
                            *expr = parse_quote! { Self::splat(#rounded) };
                        }
                        _ => {}
                    }
                }
            },
            syn::Expr::If(exprif) => {
                let cond = &*exprif.cond;
                let then_branch = deblock1(&exprif.then_branch);
                if let Some((_, else_branch)) = &exprif.else_branch {
                    let else_branch = deblock2(&*else_branch);
                    *expr = parse_quote! { (#cond).select(#then_branch, #else_branch) };
                }
            },
            _ => {}
        }
        // if let syn::Expr::
        // if let syn::Lit::Float(f) = lit {
        //     match self.options.num_bits {
        //         32 => {
        //             let rounded: f32 = f.base10_parse().unwrap();
        //             *lit = parse_quote! { Self::splat(#rounded) };
        //         }
        //         64 => {
        //             let rounded: f64 = f.base10_parse().unwrap();
        //             *lit = parse_quote! { Self::splat(#rounded) };
        //         }
        //         _ => {}
        //     }
        // }
    }
}

pub fn to_simd(file: &syn::File, options: Options) -> syn::File {
    let mut methods = Vec::new();

    let mut cv = SimdVisitor::new(options);

    for it in &file.items {
        cv.idents_used.clear();
        use Item::*;
        match it {
            Const(item) => {
                cv.consts.push(item.clone());
            }
            Fn(item) => {
                let mut new_sig = item.sig.clone();
                cv.visit_signature_mut(&mut new_sig);
                let mut new_block = (*item.block).clone();
                cv.visit_block_mut(&mut new_block);
                for c in cv.consts.clone() {
                    let ident = &c.ident;
                    if cv.idents_used.contains(ident) {
                        let mut expr = (&*c.expr).clone();
                        cv.visit_expr_mut(&mut expr);
                        let new_stmt = parse_quote! {let #ident = #expr;};
                        new_block.stmts.insert(0, new_stmt);
                    }
                }
                let new_method = syn::ImplItemMethod {
                    attrs: vec![parse_quote!(#[inline])],
                    vis: Visibility::Inherited,
                    defaultness: None,
                    sig: new_sig,
                    block: new_block,
                };
                methods.push(new_method);
            }
            Type(_item) => {}
            _ => {}
        }
    }

    parse_quote! {
        #![allow(non_snake_case)]

        use super ::StdLibm ;
        use super ::StdFloat ;
        use super ::simd::{
          LaneCount ,Simd ,SupportedLaneCount 
        };

        impl<const N: usize> StdLibm for Simd<f32, N>
        where
            LaneCount<N>: SupportedLaneCount,
        {
           #(#methods)*
        }
    }
}

#[test]
fn test() {
    let code: syn::File = parse_quote! {
        #[allow (non_camel_case_types)]
        type fty =f64 ;

        const RECIP_2PI :fty =0.1591549430918953357688837633725143620345;

        pub fn sin (arg : fty)->fty {
          let scaled :fty =arg *RECIP_2PI ;
          let x :fty =scaled -scaled .round ();
          (- 0.0000795978135564681619446994463825844449 as f64).mul_add (x * x , 0.0011251039233483632093906670512638370694 as f64).mul_add (x * x , - 0.0120293093815837587083929079549229916291 as f64).mul_add (x * x , 0.1042285941703196255136347927732329854039 as f64).mul_add (x * x , - 0.7181222077484850721256518873820977766730 as f64).mul_add (x * x , 3.8199525744232107661057457125514942650559 as f64).mul_add (x * x , - 15.0946425760590780811041894566331393312522 as f64).mul_add (x * x , 42.0586939448620164904399014801571486455824 as f64).mul_add (x * x , - 76.7058597530604003747392381683185548195638 as f64).mul_add (x * x , 81.6052492760750400501824780624242989421231 as f64).mul_add (x * x , - 41.3417022403997601538762424010531941752307 as f64).mul_add (x * x , 6.2831853071795864768497321650524941104931 as f64)*x
        }
    };

    let options = Options::default();
    let file = to_simd(&code, options);

    println!(
        "res\n{}",
        super::rust::format_token_stream(file.to_token_stream())
    );
}
