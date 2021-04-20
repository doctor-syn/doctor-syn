

use crate::visitor::Visitor;
use crate::error::Error;
use crate::VariableList;
use syn::{Expr, ExprPath};

#[derive(Debug)]
pub struct Subst {
    pub (crate) variables: VariableList,
}

impl Visitor for Subst {
    /// eg. "x" or "f64::const::PI"
    fn visit_path(&self, exprpath: &ExprPath) -> Result<Expr, Error> {
        if let Some(res) = self.variables.find(&exprpath.path) {
            // Substitute.
            Ok(res.into())
        } else {
            // Clone.
            Ok(exprpath.clone().into())
        }
    }
}
