use crate::error::Result;
use crate::visitor::Visitor;
use syn::{Expr, ExprLit, LitFloat, LitInt, Lit};
use syn::spanned::Spanned;

#[derive(Debug)]
pub struct UseSuffix {
    pub(crate) float_suffix: Option<String>,
}

impl Visitor for UseSuffix {
    // eg. "1.0"
    fn visit_lit(&self, expr: &ExprLit) -> Result<Expr> {
        match &expr.lit {
            Lit::Float(litfloat) => {
                if let Some(float_suffix) = self.float_suffix.as_ref() {
                    let repr = format!("{}_{}", litfloat.base10_digits(), float_suffix);
                    let lit : Lit = LitFloat::new(repr.as_str(), expr.span()).into();
                    let attrs = Vec::new();
                    Ok(Expr::Lit(ExprLit { attrs, lit}))
                } else {
                    Ok(expr.clone().into())
                }
            }
            Lit::Int(litint) => {
                if let Some(float_suffix) = self.float_suffix.as_ref() {
                    let repr = format!("{}_{}", litint.base10_digits(), float_suffix);
                    let lit : Lit = LitInt::new(repr.as_str(), expr.span()).into();
                    let attrs = Vec::new();
                    Ok(Expr::Lit(ExprLit { attrs, lit}))
                } else {
                    Ok(expr.clone().into())
                }
            }
            _ => Ok(expr.clone().into())
        }
    }
}
