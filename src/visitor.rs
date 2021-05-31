use crate::error::{Error, Result};
use quote::ToTokens;
use syn::spanned::Spanned;
use syn::{
    punctuated::Punctuated, Expr, ExprBinary, ExprField, ExprLit, ExprMethodCall, ExprParen,
    ExprPath, ExprUnary, Token,
};

const TRACING : bool = false;
// use proc_macro2::Span;

/// A visitor trait for a subset of expressions.
/// Default behaviour is to clone the expression.
pub trait Visitor {
    fn visit_method_call(&self, expr: &ExprMethodCall) -> Result<Expr> {
        let receiver = self.visit_expr(&expr.receiver)?;
        let args: Punctuated<Expr, Token![,]> = expr
            .args
            .iter()
            .map(|a| self.visit_expr(a))
            .collect::<Result<Punctuated<Expr, Token![,]>>>()?;
        Ok(ExprMethodCall {
            attrs: expr.attrs.clone(),
            receiver: Box::new(receiver),
            dot_token: expr.dot_token.clone(),
            method: expr.method.clone(),
            turbofish: expr.turbofish.clone(),
            paren_token: expr.paren_token.clone(),
            args: args,
        }
        .into())
    }

    // eg. "1.0"
    fn visit_lit(&self, expr: &ExprLit) -> Result<Expr> {
        Ok(expr.clone().into())
    }

    /// eg. "x" or "f64::const::PI"
    fn visit_path(&self, exprpath: &ExprPath) -> Result<Expr> {
        Ok(exprpath.clone().into())
    }

    /// eg. "x.y"
    fn visit_field(&self, exprfield: &ExprField) -> Result<Expr> {
        let base = self.visit_expr(&exprfield.base)?;
        Ok(ExprField {
            attrs: exprfield.attrs.clone(),
            base: Box::new(base),
            dot_token: exprfield.dot_token,
            member: exprfield.member.clone(),
        }
        .into())
    }

    /// eg. "(x)"
    fn visit_paren(&self, exprparen: &ExprParen) -> Result<Expr> {
        let expr = self.visit_expr(&exprparen.expr)?;
        Ok(ExprParen {
            attrs: exprparen.attrs.clone(),
            paren_token: exprparen.paren_token.clone(),
            expr: Box::new(expr),
        }
        .into())
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

    // fn visit_block(&self, block: &Block) -> Result<Expr> {
    //     let stmts = block.stmts.iter()
    //         .map(|stmt| self.visit_stmt(stmt))
    //         .collect::<Result<Vec<Stmt>>>()?;
    //     Ok(Block {
    //         brace_token: block.brace_token,
    //         stmts
    //     }.into())
    // }

    fn visit_binary(&self, exprbinary: &ExprBinary) -> Result<Expr> {
        let left = self.visit_expr(&exprbinary.left)?;
        let right = self.visit_expr(&exprbinary.right)?;
        Ok(ExprBinary {
            attrs: exprbinary.attrs.clone(),
            left: Box::new(left),
            op: exprbinary.op.clone(),
            right: Box::new(right),
        }
        .into())
    }

    fn visit_unary(&self, exprunary: &ExprUnary) -> Result<Expr> {
        let expr = self.visit_expr(&exprunary.expr)?;
        Ok(ExprUnary {
            attrs: exprunary.attrs.clone(),
            op: exprunary.op,
            expr: Box::new(expr),
        }
        .into())
    }

    // Visit a generalised expression.
    fn visit_expr(&self, expr: &Expr) -> Result<Expr> {
        if TRACING {
            println!("visit_expr: {}", expr.to_token_stream());
            let expr = self.visit_expr_core(&expr).unwrap();
            println!("..visit_expr: {}", expr.to_token_stream());
            Ok(expr)
        } else {
            self.visit_expr_core(&expr)
        }
    }

    fn visit_expr_core(&self, expr: &Expr) -> Result<Expr> {
        use Expr::*;
        match expr {
            Unary(exprunary) => self.visit_unary(exprunary),
            Binary(exprbinary) => self.visit_binary(&exprbinary),
            MethodCall(exprmethodcall) => self.visit_method_call(exprmethodcall),
            Paren(exprparen) => self.visit_paren(&exprparen),
            Lit(exprlit) => self.visit_lit(&exprlit),
            Path(exprpath) => self.visit_path(exprpath),
            Field(exprfield) => self.visit_field(exprfield),
            _ => Err(Error::UnsupportedExpr(expr.span())),
        }
    }
}
