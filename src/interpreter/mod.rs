//! Very simple interpreter for single expression functions.
//! This may become unneccessary if const functions are upgraded.

use syn::{BinOp, Block, Expr, ExprBinary, ExprLit, ExprMethodCall, ExprPath, Lit, Stmt, Path};
use crate::error::Error;
use syn::spanned::Spanned;
use proc_macro2::Span;

pub fn from_any<T : std::fmt::Debug>(value: T) -> Result<Expr, Error> {
    let s = format!("{:?}", value);
    parse_str(s.as_str())
}

pub fn as_number<T>(expr: &Expr) -> Result<T, Error>
where
    T : std::str::FromStr,
    T::Err : std::fmt::Display
{
    if let Expr::Lit(ref lit) = expr {
        match &lit.lit {
            Lit::Float(f) => f.base10_parse().map_err(|_| Error::TriedToConvertLiteralToNumber(lit.lit.span())),
            Lit::Int(i) => i.base10_parse().map_err(|_| Error::TriedToConvertLiteralToNumber(lit.lit.span())),
            _ => return Err(Error::TriedToConvertLiteralToNumber(lit.lit.span()))
        }
    } else {
        Err(Error::TriedToConvertLiteralToNumber(expr.span()))
    }
}

pub fn parse_str<T : syn::parse::Parse>(s: &str) -> Result<T, Error> {
    syn::parse_str(s).map_err(|_| Error::CouldNotParse(Span::call_site()))
}

#[derive(Debug, Default)]
pub struct Domain {
    pub min: Option<Expr>,
    pub max: Option<Expr>,
    pub terms: Option<Expr>,
}

#[derive(Debug)]
pub struct Variable {
    pub path: Path,
    pub value: Expr,
    pub domain: Domain,
}

impl Variable {
    pub fn new() -> Self {
        Self {
            path: syn::parse_str("x").unwrap(),
            value: syn::parse_str("0").unwrap(),
            domain: Domain { min: None, max: None, terms: None }
        }
    }
}

// A *very* basic Rust interpreter.
// TODO: make this a trait.
// Split this into subst(expr, variables) and approx(expr).
#[derive(Debug, Default)]
pub struct Interpreter {
    variables: Vec<Variable>,
}

// Evaluate in an empty context.
// pub fn eval(expr: &Expr) -> Result<Expr, Error> {
//     let ctxt = Interpreter::new();
//     ctxt.expr(expr)
// }

#[allow(dead_code)]
impl Interpreter {
    pub fn new() -> Self {
        Self::from_variables(Vec::new())
    }

    pub fn from_variables(variables: Vec<Variable>) -> Self {
        Self { variables }
    }

    pub fn method_call(&self, expr: &ExprMethodCall) -> Result<Expr, Error> {
        let reciever = self.expr(&expr.receiver).and_then(|e| as_number::<f64>(&e))?;
        let id = &expr.method;
        let args : Vec<f64> = expr
            .args
            .iter()
            // self.expr(a).map(|e| as_number::<f64>(&e))
            .map(|a| as_number(&a))
            .collect::<Result<Vec<f64>, Error>>()?;
        
        match id.to_string().as_str() {
            "sin" => from_any(reciever.sin()),
            "cos" => from_any(reciever.cos()),
            "exp" => from_any(reciever.exp()),
            "sqrt" => from_any(reciever.sqrt()),
            "ln" => from_any(reciever.ln()),
            "powf" => from_any(reciever.powf(args[0])),
            method => Err(Error::UnsupportedMethod(method.span())),
        }
    }

    /// eg. set_var(Path::parse("x")?);
    pub fn set_var(&mut self, path: &Path, value: Expr) -> Result<(), Error> {
        self.variables.iter_mut().find(|v| &v.path == path).map(|v| v.value = value);
        Ok(())
    }

    /// eg. get_var(Path::parse("x")?);
    pub fn get_var(&self, path: &Path) -> Result<Expr, Error> {
        self.variables.iter().find(|v| &v.path == path)
            .map(|v| v.value.clone())
            .ok_or_else(|| Error::NotFound(path.span()))
    }

    // eg. "1.0"
    pub fn lit(&self, expr: &ExprLit) -> Result<Expr, Error> {
        Ok(expr.clone().into())
    }

    pub fn path(&self, exprpath: &ExprPath) -> Result<Expr, Error> {
        self.get_var(&exprpath.path)
    }

    pub fn stmt(&self, stmt: &Stmt) -> Result<Expr, Error> {
        match stmt {
            Stmt::Expr(expr) => self.expr(expr),
            _ => Err(Error::UnsupportedStatement(stmt.span())),
        }
    }

    pub fn block(&self, block: &Block) -> Result<Expr, Error> {
        if block.stmts.len() != 1 {
            return Err(Error::BlockMustHaveOneStatement(block.span()));
        }
        self.stmt(&block.stmts[0])
    }

    pub fn binary(&self, exprbinary: &ExprBinary) -> Result<Expr, Error> {
        let left = self.expr(&exprbinary.left)?;
        let right = self.expr(&exprbinary.right)?;

        // As a spike, just use f64.
        let left : f64 = as_number(&left)?;
        let right : f64 = as_number(&right)?;
        match exprbinary.op {
            // The `+` operator (addition)
            BinOp::Add(_) => from_any(left + right),
            // The `-` operator (subtraction)
            BinOp::Sub(_) => from_any(left - right),
            // The `*` operator (multiplication)
            BinOp::Mul(_) => from_any(left * right),
            // The `/` operator (division)
            BinOp::Div(_) => from_any(left / right),

            _ => Err(Error::UnsupportedExpr(exprbinary.op.span())),
        }
    }

    // Evaluate simple expressions like (x+1.0).sin()
    pub fn expr(&self, expr: &Expr) -> Result<Expr, Error> {
        use Expr::*;
        match expr {
            // A binary operation: `a + b`, `a * b`.
            Binary(exprbinary) => self.binary(&exprbinary),

            // A method call expression: `x.foo::<T>(a, b)`.
            MethodCall(exprmethodcall) => self.method_call(exprmethodcall),

            // A parenthesized expression: `(a + b)`.
            Paren(exprparen) => self.expr(&exprparen.expr),

            Lit(exprlit) => self.lit(&exprlit),

            Path(exprpath) => self.path(exprpath),

            _ => Err(Error::UnsupportedExpr(expr.span()))
        }
    }
}

