use crate::error::Result;
use crate::visitor::Visitor;
// use syn::{Expr, ExprUnary, ExprBinary, UnOp, BinOp};
// use syn::spanned::Spanned;
use crate::name::Name;
// use super::tools::*;

#[derive(Debug)]
pub struct Collect {
    pub(crate) variable: Name,
}

impl Visitor for Collect {}

#[test]
fn collect() -> Result<()> {
    use crate::{expr, name};

    // // Powers
    // assert_eq!(expr!(x * x).collect_terms(name!(x))?, expr!(x.pow(2)));
    // assert_eq!(expr!(x * x * x).collect_terms(name!(x))?, expr!(x.pow(3)));
    // assert_eq!(
    //     expr!(y * x * 3 * x * x).collect_terms(name!(x))?,
    //     expr!((y * 3) * x.pow(3))
    // );

    // // Multiply
    // assert_eq!(expr!(1 * x).collect_terms(name!(x))?, expr!(x));
    // assert_eq!(expr!(x * 2).collect_terms(name!(x))?, expr!(2 * x));
    // assert_eq!(expr!(x * 1 + 2 * x).collect_terms(name!(x))?, expr!(3 * x));
    // assert_eq!(
    //     expr!(x * y + x * 2).collect_terms(name!(x))?,
    //     expr!((y + 2) * x)
    // );
    // assert_eq!(
    //     expr!(
    //         x * x * x
    //             + x * x * 1
    //             + x * 1 * x
    //             + x * 1 * 1
    //             + 1 * x * x
    //             + 1 * x * 1
    //             + 1 * 1 * x
    //             + 1 * 1 * 1
    //     )
    //     .collect_terms(name!(x))?,
    //     expr!(x.pow(3) + 3 * x.pow(2) + 3 * x + 1)
    // );

    Ok(())
}
