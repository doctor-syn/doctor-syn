use crate::error::Result;
use crate::visitor::Visitor;
use bigdecimal::*;
use syn::{parse_quote, Expr, ExprLit, Lit};

#[derive(Debug)]
pub struct UseNumberType<'a> {
    pub(crate) number_type: &'a str,
    pub(crate) num_bits: usize,
}

impl<'a> Visitor for UseNumberType<'a> {
    // eg. "1.0"
    fn visit_lit(&self, expr: &ExprLit) -> Result<Expr> {
        match &expr.lit {
            Lit::Float(lit) => {
                let bd: BigDecimal = lit.base10_digits().parse().unwrap();
                let float32 = bd.to_f32().unwrap();
                let float64 = bd.to_f64().unwrap();
                let bits32 = bd.to_f32().unwrap().to_bits();
                let bits64 = bd.to_f64().unwrap().to_bits();
                let e: Expr = match (self.number_type, self.num_bits) {
                    ("decimal", 32) => parse_quote!(#float32),
                    ("decimal", 64) => parse_quote!(#float64),
                    ("hex", 32) => parse_quote!(f32::from_bits(#bits32)),
                    ("hex", 64) => parse_quote!(f32::from_bits(#bits64)),
                    _ => expr.clone().into(),
                };
                Ok(e)
            }
            Lit::Int(lit) => {
                let bd: BigDecimal = lit.base10_digits().parse().unwrap();
                let float32 = bd.to_f32().unwrap();
                let float64 = bd.to_f64().unwrap();
                let bits32 = bd.to_f32().unwrap().to_bits();
                let bits64 = bd.to_f64().unwrap().to_bits();
                let e: Expr = match (self.number_type, self.num_bits) {
                    ("decimal", 32) => parse_quote!(#float32),
                    ("decimal", 64) => parse_quote!(#float64),
                    ("hex", 32) => parse_quote!(f32::from_bits(#bits32)),
                    ("hex", 64) => parse_quote!(f32::from_bits(#bits64)),
                    _ => expr.clone().into(),
                };
                Ok(e)
            }
            _ => Ok(expr.clone().into()),
        }
    }
}
