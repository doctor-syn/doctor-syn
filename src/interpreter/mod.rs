//! Very simple interpreter for single expression functions.
//! This may become unneccessary if const functions are upgraded.

use quote::quote;
// use syn::punctuated::Punctuated;
// use syn::Token;
use syn::{BinOp, Block, Expr, ExprBinary, ExprLit, ExprMethodCall, ExprPath, Lit, Stmt};

#[derive(Debug)]
pub enum Error {
    UnsupportedExpr(String),
    UnsupportedMethod(String),
    UnsupportedLiteral(String),
    UnsupportedVariable(String),
    UnsupportedStatement(String),
    BlockMustHaveOneStatement(String),
}

type Value = f64;

#[derive(Debug, Default)]
pub struct Domain {
    pub min: Option<Value>,
    pub max: Option<Value>,
    pub terms: Option<usize>,
}

#[derive(Debug, Default)]
pub struct Variable {
    pub name: String,
    pub value: Value,
    pub domain: Domain,
}

// A *very* basic Rust interpreter.
// It has only one variable, "x".
#[derive(Debug, Default)]
pub struct Interpreter {
    x: f64,
    variables: Vec<Variable>,
}

fn unsupported(expr: &Expr) -> Result<f64, Error> {
    Err(Error::UnsupportedExpr(quote!(#expr).to_string()))
}

impl Interpreter {
    pub fn new_x(x: f64) -> Self {
        Self { x, ..Self::default() }
    }

    pub fn method_call(&self, expr: &ExprMethodCall) -> Result<f64, Error> {
        let reciever = self.expr(&expr.receiver)?;
        let id = &expr.method;
        let args = expr
            .args
            .iter()
            .map(|a| self.expr(a))
            .collect::<Result<Vec<_>, Error>>()?;
        match id.to_string().as_str() {
            "sin" => Ok(reciever.sin()),
            "cos" => Ok(reciever.cos()),
            "exp" => Ok(reciever.exp()),
            "sqrt" => Ok(reciever.sqrt()),
            "ln" => Ok(reciever.ln()),
            "powf" => Ok(reciever.powf(args[0])),
            method => Err(Error::UnsupportedMethod(method.to_string())),
        }
    }

    // eg. "1.0"
    pub fn lit(&self, expr: &ExprLit) -> Result<f64, Error> {
        match &expr.lit {
            Lit::Float(litfloat) => {
                if let Ok(res) = litfloat.base10_parse() {
                    Ok(res)
                } else {
                    Err(Error::UnsupportedLiteral(quote!(#expr).to_string()))
                }
            }
            lit => Err(Error::UnsupportedLiteral(quote!(#lit).to_string())),
        }
    }

    pub fn path(&self, exprpath: &ExprPath) -> Result<f64, Error> {
        if let Some(id) = exprpath.path.get_ident() {
            if id.to_string() == "x" {
                return Ok(self.x);
            }
        }
        Err(Error::UnsupportedVariable(quote!(#exprpath).to_string()))
    }

    pub fn stmt(&self, stmt: &Stmt) -> Result<f64, Error> {
        match stmt {
            // A local (let) binding.
            //Local(local),

            // An item definition.
            //Item(item),

            // Expr without trailing semicolon.
            Stmt::Expr(expr) => self.expr(expr),

            // Expression with trailing semicolon.
            //Semi(expr, _),
            _ => Err(Error::UnsupportedStatement(quote!(#stmt).to_string())),
        }
    }

    pub fn block(&self, block: &Block) -> Result<f64, Error> {
        if block.stmts.len() != 1 {
            return Err(Error::BlockMustHaveOneStatement(quote!(#block).to_string()));
        }
        self.stmt(&block.stmts[0])
    }

    pub fn binary(&self, exprbinary: &ExprBinary) -> Result<f64, Error> {
        let left = self.expr(&exprbinary.left)?;
        let right = self.expr(&exprbinary.right)?;
        match exprbinary.op {
            // The `+` operator (addition)
            BinOp::Add(_) => Ok(left + right),
            // The `-` operator (subtraction)
            BinOp::Sub(_) => Ok(left - right),
            // The `*` operator (multiplication)
            BinOp::Mul(_) => Ok(left * right),
            // The `/` operator (division)
            BinOp::Div(_) => Ok(left / right),

            _ => Err(Error::UnsupportedExpr(quote!(#exprbinary).to_string())),
        }
    }

    // expruate simple expressions like (x+1.0).sin()
    pub fn expr(&self, expr: &Expr) -> Result<f64, Error> {
        use Expr::*;
        match expr {
            // A binary operation: `a + b`, `a * b`.
            Binary(exprbinary) => self.binary(&exprbinary),

            // A method call expression: `x.foo::<T>(a, b)`.
            MethodCall(exprmethodcall) => self.method_call(exprmethodcall),

            // A parenthesized expression: `(a + b)`.
            Paren(exprparen) => self.expr(&exprparen.expr),

            Lit(exprlit) => self.lit(&exprlit),

            // A path like `std::mem::replace` possibly containing generic
            // parameters and a qualified self-type.
            //
            // A plain identifier like `x` is a path of length 1.
            Path(exprpath) => self.path(exprpath),

            _ => unsupported(&expr),
        }
    }
}

