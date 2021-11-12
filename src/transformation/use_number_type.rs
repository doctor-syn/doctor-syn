use crate::error::Result;
use crate::visitor::Visitor;
use bigdecimal::*;
use syn::{parse_quote, Expr, ExprLit, Lit};

#[derive(Debug)]
pub struct UseNumberType<'a> {
    pub(crate) number_type: &'a str,
}

impl<'a> Visitor for UseNumberType<'a> {
    // eg. "1.0"
    fn visit_lit(&self, expr: &ExprLit) -> Result<Expr> {
        match &expr.lit {
            Lit::Float(litfloat) => {
                let bd: BigDecimal = litfloat.base10_digits().parse().unwrap();
                let bits32 = bd.to_f32().unwrap().to_bits();
                let bits64 = bd.to_f64().unwrap().to_bits();
                let e: Expr = match self.number_type {
                    "f32_hex" => parse_quote!(mkfty(#bits32)),
                    "f32_simd" => parse_quote!(mkfty(#bits32)),
                    "f64_hex" => parse_quote!(mkfty(#bits64)),
                    "f64_simd" => parse_quote!(mkfty(#bits64)),
                    _ => expr.clone().into(),
                };
                Ok(e)
            }
            Lit::Int(litint) => {
                let bd: BigDecimal = litint.base10_digits().parse().unwrap();
                let bits32 = bd.to_f32().unwrap().to_bits();
                let bits64 = bd.to_f64().unwrap().to_bits();
                let e: Expr = match self.number_type {
                    "f32_hex" => parse_quote!(mkfty(#bits32)),
                    "f32_simd" => parse_quote!(mkfty(#bits32)),
                    "f64_hex" => parse_quote!(mkfty(#bits64)),
                    "f64_simd" => parse_quote!(mkfty(#bits64)),
                    _ => expr.clone().into(),
                };
                Ok(e)
            }
            _ => Ok(expr.clone().into()),
        }
    }
}
