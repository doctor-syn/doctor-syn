
use super::*;

// use quote::quote;
// use syn::punctuated::Punctuated;
// use syn::Token;
use syn::{parse_quote, Expr, Block};

#[test]
fn test_interpreter() {
    let mut interp = interpreter::Interpreter::new();
    interp.set_var(&parse_quote!(x), parse_quote!(0.5));

    // Block.
    let e : Block = parse_quote!({x});
    assert_eq!(interp.block(&e).unwrap(), parse_quote!(0.5));

    // Variable.
    let e : Expr = parse_quote!(x);
    assert_eq!(interp.expr(&e).unwrap(), parse_quote!(0.5));

    // Binops.
    let e : Expr = parse_quote!(1.0 + 1.0);
    assert_eq!(interp.expr(&e).unwrap(), parse_quote!(2.0));

    let e : Expr = parse_quote!(1.0e6 - 1.0);
    assert_eq!(interp.expr(&e).unwrap(), parse_quote!(999999.0));

    // let e : Expr = parse_quote!(10.0 * 10.0);
    // assert_eq!(interp.expr(&e).unwrap(), 100.0);

    // let e : Expr = parse_quote!(10.0 / 2.0);
    // assert_eq!(interp.expr(&e).unwrap(), 5.0);

    // // Maths functions
    // let e : Expr = parse_quote!(10.0.powf(2.0));
    // assert_eq!(interp.expr(&e).unwrap(), 100.0);
    // let e : Expr = parse_quote!(0.0.exp());
    // assert_eq!(interp.expr(&e).unwrap(), 1.0);
    // let e : Expr = parse_quote!(1.0.ln());
    // assert_eq!(interp.expr(&e).unwrap(), 0.0);
    // let e : Expr = parse_quote!(0.0.sin());
    // assert_eq!(interp.expr(&e).unwrap(), 0.0);
    // let e : Expr = parse_quote!(0.0.cos());
    // assert_eq!(interp.expr(&e).unwrap(), 1.0);
    // let e : Expr = parse_quote!(4.0.sqrt());
    // assert_eq!(interp.expr(&e).unwrap(), 2.0);
}
