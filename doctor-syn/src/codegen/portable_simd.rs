//! Convert a file of functions into a set of methods
//! suitable for rust-lang/portable-simd

use quote::{quote, format_ident, ToTokens};
use syn::punctuated::Punctuated;
use syn::visit_mut::{visit_expr_mut, visit_ident_mut, VisitMut, visit_signature_mut, visit_local_mut};
use syn::{parse_quote, Expr, Ident, Item, ItemConst, Token, Visibility};

pub struct Options {
    pub num_bits: usize,
}

impl Default for Options {
    fn default() -> Self {
        Options { num_bits: 32 }
    }
}

#[allow(dead_code)]
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
        self.idents_used.push(i.clone());
        visit_ident_mut(self, i);
        match i {
            i if i == "fty" => *i = Ident::new("Self", i.span()),
            // i if i == "uty" => *i = Ident::new("Self::UintType", i.span()),
            // i if i == "ity" => *i = Ident::new("Self::IntType", i.span()),
            _ => {
                self.idents_used.push(i.clone());
                visit_ident_mut(self, i);
            }
        }
    }

    fn visit_signature_mut(&mut self, sig: &mut syn::Signature) {
        visit_signature_mut(self, sig);
        // Patch the first argument to self
        let arg0 = sig.inputs.iter_mut().next().unwrap();
        *arg0 = parse_quote! {self};
        sig.output = parse_quote! {-> Self};
    }

    fn visit_type_path_mut(&mut self, type_path: &mut syn::TypePath) {
        match type_path.to_token_stream().to_string().as_str() {
            "uty" => *type_path = parse_quote!(Self::UintType),
            "ity" => *type_path = parse_quote!(Self::IntType),
            "fty" => *type_path = parse_quote!(Self),
            _ => {},
        }
    }

    // Convert `lit as f32` etc. to splats.
    fn visit_expr_mut(&mut self, expr: &mut syn::Expr) {
        // println!("{} {:?}", expr.to_token_stream(), expr);
        visit_expr_mut(self, expr);

        match &*expr {
            syn::Expr::Cast(cast) => {
                *expr = convert_cast(cast, self.options.num_bits)
            }
            syn::Expr::Binary(binary) => {
                *expr = convert_binary(binary)
            }
            syn::Expr::Call(call) => {
                *expr = convert_call(call);
            }
            syn::Expr::Lit(syn::ExprLit { lit, ..} ) => {
                match lit {
                    syn::Lit::Float(f) => {
                        *expr = convert_lit_float(f);
                    }
                    syn::Lit::Int(f) => {
                        *expr = convert_lit_int(f);
                    }
                    _ => {}
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
            syn::Expr::Path(exprpath) => {
                // println!("HERE: {}", exprpath.to_token_stream().to_string().as_str())
                match exprpath.to_token_stream().to_string().as_str() {
                    "f32 :: NAN" => *expr = parse_quote! { Self::splat(f32::NAN) },
                    "f64 :: NAN" => *expr = parse_quote! { Self::splat(f64::NAN) },
                    "f32 :: INFINITY" => *expr = parse_quote! { Self::splat(f32::INFINITY) },
                    "f64 :: INFINITY" => *expr = parse_quote! { Self::splat(f64::INFINITY) },
                    "f32 :: MIN_POSITIVE" => *expr = parse_quote! { Self::splat(f32::MIN_POSITIVE) },
                    "f64 :: MIN_POSITIVE" => *expr = parse_quote! { Self::splat(f64::MIN_POSITIVE) },
                    _ => (),
                }
            },
            _ => {}
        }
    }

    fn visit_local_mut(&mut self, local: &mut syn::Local) {
        visit_local_mut(self, local);
        if let syn::Pat::Type(pat_type) = &local.pat {
            local.pat = *pat_type.pat.clone();
        }
    }
}

fn convert_cast(cast: &syn::ExprCast, num_bits: usize) -> Expr {
    let expr = &*cast.expr;
    let ty = &*cast.ty;
    let (uty, ity, fty) = if num_bits == 32 {
        (quote!(u32), quote!(i32), quote!(f32))
    } else {
        (quote!(u64), quote!(i64), quote!(f64))
    };
    match ty.to_token_stream().to_string().as_str() {
        "uty" | "Self :: UintType" => parse_quote!{ #expr.cast::<#uty>() },
        "ity" | "Self :: IntType" => parse_quote!{ #expr.cast::<#ity>() },
        "fty" | "Self" => parse_quote!{ #expr.cast::<#fty>() },
        _ => cast.clone().into()
    }
}

fn convert_binary(binary: &syn::ExprBinary) -> Expr {
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
        parse_quote! { (#lhs).#id(#rhs) }
    } else {
        binary.clone().into()
    }
}

fn convert_call(call: &syn::ExprCall) -> Expr {
    let mut args = call.args.iter();
    let arg0 = args.next().unwrap();
    match call.func.to_token_stream().to_string().as_str() {
        "sin" | "cos" | "tan" | "exp2" | "powf" | "log2" | "ln" => {
            let func = &*call.func;
            let rest = args.cloned().collect::<Punctuated<Expr, Token![,]>>();
            parse_quote! { (#arg0).#func(#rest) }
        }

        _ => call.clone().into()
    }
}

fn convert_lit_float(f: &syn::LitFloat) -> Expr {
    match f.suffix() {
        "u32" | "u64" => {
            parse_quote! { UintType::splat(#f) }
        }
        "i32" | "i64" => {
            parse_quote! { IntType::splat(#f) }
        }
        "f32" | "f64" | _ => {
            parse_quote! { Self::splat(#f) }
        }
    }
}

fn convert_lit_int(f: &syn::LitInt) -> Expr {
    match f.suffix() {
        "u32" | "u64" => {
            parse_quote! { Self::UintType::splat(#f) }
        }
        "i32" | "i64" => {
            parse_quote! { Self::IntType::splat(#f) }
        }
        "f32" | "f64" | _ => {
            parse_quote! { Self::splat(#f) }
        }
    }
}

// fn convert_path(f: &syn::Path) -> Expr {
//     // match f.to_token_stream().to_string().as_str() {
//     //     "uty" => {
//     //         *expr = parse_quote! { Self::UintType }
//     //     }
//     //     "ity" => {
//     //         *expr = parse_quote! { Self::IntType }
//     //     }
//     //     "fty" | _ => {
//     //         *expr = parse_quote! { Self }
//     //     }
//     // }
//     f.into()
// }

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
                visit_signature_mut(&mut cv, &mut new_sig);
                let first_arg = item.sig.inputs.iter().next().unwrap();
                let pat = &**match first_arg {
                    syn::FnArg::Typed(syn::PatType{pat, ..}) => pat,
                    _ => unreachable!("expected typed arg"),
                };
                let ident = match pat {
                    syn::Pat::Ident(id) => &id.ident,
                    _ => unreachable!("expected identifier"),
                };
                cv.visit_signature_mut(&mut new_sig);
                let mut new_block = (*item.block).clone();
                new_block.stmts.insert(0, parse_quote! {let #ident = self;});
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
        #![doc("This code is automatically generated, do not edit.")]

        use super ::StdLibm ;
        use super ::StdFloat ;
        use super ::simd::{
          LaneCount ,Simd ,SupportedLaneCount 
        };

        impl<const N: usize> StdLibm for Simd<f32, N>
        where
            LaneCount<N>: SupportedLaneCount,
        {
            type IntType = Simd<i32, N>;
            type UintType = Simd<u32, N>;

            #(#methods)*
        }
    }
}

#[test]
fn test() {
    use quote::ToTokens;

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
