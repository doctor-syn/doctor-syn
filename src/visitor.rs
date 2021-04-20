
use syn::{Expr, ExprBinary, ExprLit, ExprMethodCall, ExprPath, punctuated::Punctuated, Token};
use crate::error::Error;
use syn::spanned::Spanned;
// use proc_macro2::Span;

/// A visitor trait for a subset of expressions.
/// Default behaviour is to clone the expression.
pub trait Visitor {
    fn visit_method_call(&self, expr: &ExprMethodCall) -> Result<Expr, Error> {
        let receiver = self.visit_expr(&expr.receiver)?;
        let args : Punctuated<Expr, Token![,]> = expr.args.iter().map(|a| self.visit_expr(a)).collect::<Result<Punctuated<Expr, Token![,]>, Error>>()?;
        Ok(ExprMethodCall {
            attrs: expr.attrs.clone(),
            receiver: Box::new(receiver),
            dot_token: expr.dot_token.clone(),
            method: expr.method.clone(),
            turbofish: expr.turbofish.clone(),
            paren_token: expr.paren_token.clone(),
            args: args,
        }.into())
    }

    // eg. "1.0"
    fn visit_lit(&self, expr: &ExprLit) -> Result<Expr, Error> {
        Ok(expr.clone().into())
    }

    /// eg. "x" or "f64::const::PI"
    fn visit_path(&self, exprpath: &ExprPath) -> Result<Expr, Error> {
        Ok(exprpath.clone().into())
    }

    /// eg. "let x = 1;" "fn x();" "x * 2" or "x * 2;"
    // fn visit_stmt(&self, stmt: &Stmt) -> Result<Stmt, Error> {
    //     match stmt {
    //         Stmt::Local(local) => Ok(local.clone().into()),
    //         Stmt::Item(item) => Ok(item.clone().into()),
    //         Stmt::Expr(expr) => Ok(self.visit_expr(expr).into()),
    //         Stmt::Semi(expr, semi) => Ok(Stmt::Semi(self.visit_expr(expr), semi)),
    //     }
    // }

    // fn visit_block(&self, block: &Block) -> Result<Expr, Error> {
    //     let stmts = block.stmts.iter()
    //         .map(|stmt| self.visit_stmt(stmt))
    //         .collect::<Result<Vec<Stmt>>>()?;
    //     Ok(Block {
    //         brace_token: block.brace_token,
    //         stmts
    //     }.into())
    // }

    fn visit_binary(&self, exprbinary: &ExprBinary) -> Result<Expr, Error> {
        let left = self.visit_expr(&exprbinary.left)?;
        let right = self.visit_expr(&exprbinary.right)?;
        Ok(ExprBinary {
            attrs: exprbinary.attrs.clone(),
            left: Box::new(left),
            op: exprbinary.op.clone(),
            right: Box::new(right),
        }.into())
    }

    // Evaluate simple expressions like (x+1.0).sin()
    fn visit_expr(&self, expr: &Expr) -> Result<Expr, Error> {
        use Expr::*;
        match expr {
            // A binary operation: `a + b`, `a * b`.
            Binary(exprbinary) => self.visit_binary(&exprbinary),

            // A method call expression: `x.foo::<T>(a, b)`.
            MethodCall(exprmethodcall) => self.visit_method_call(exprmethodcall),

            // A parenthesized expression: `(a + b)`.
            Paren(exprparen) => self.visit_expr(&exprparen.expr),

            Lit(exprlit) => self.visit_lit(&exprlit),

            Path(exprpath) => self.visit_path(exprpath),

            _ => Err(Error::UnsupportedExpr(expr.span()))
        }
    }
}
