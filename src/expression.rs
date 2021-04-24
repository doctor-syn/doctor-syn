use crate::error::{Error, Result};
use crate::transformation::approx;
use crate::transformation::Eval;
use crate::transformation::Subst;
use crate::visitor::Visitor;
use crate::Evaluateable;
use crate::{Name, VariableList};
use proc_macro2::Span;
use quote::quote;
use std::convert::{TryFrom, TryInto};
use syn::spanned::Spanned;
use syn::{Expr, ExprLit, Lit};

#[derive(Clone, PartialEq)]
pub struct Expression {
    pub(crate) inner: Expr,
}

impl From<Expr> for Expression {
    fn from(expr: Expr) -> Self {
        Self { inner: expr }
    }
}

impl TryFrom<f64> for Expression {
    type Error = Error;

    fn try_from(val: f64) -> Result<Self> {
        let s = format!("{}", val);
        let inner: ExprLit =
            syn::parse_str(s.as_str()).map_err(|_| Error::CouldNotParse(Span::call_site()))?;
        Ok(Self {
            inner: inner.into(),
        })
    }
}

impl TryFrom<f32> for Expression {
    type Error = Error;

    fn try_from(val: f32) -> Result<Self> {
        let s = format!("{}", val);
        let inner: ExprLit =
            syn::parse_str(s.as_str()).map_err(|_| Error::CouldNotParse(Span::call_site()))?;
        Ok(Self {
            inner: inner.into(),
        })
    }
}

fn expr_to_f64(expr: &syn::Expr) -> Result<f64> {
    if let Expr::Lit(ref lit) = expr {
        match &lit.lit {
            Lit::Float(f) => f
                .base10_parse()
                .map_err(|_| Error::CouldNotConvertFromExpression(lit.lit.span())),
            Lit::Int(i) => i
                .base10_parse()
                .map_err(|_| Error::CouldNotConvertFromExpression(lit.lit.span())),
            _ => return Err(Error::CouldNotConvertFromExpression(lit.lit.span())),
        }
    } else {
        Err(Error::CouldNotConvertFromExpression(expr.span()))
    }
}

fn expr_to_f32(expr: &syn::Expr) -> Result<f32> {
    if let Expr::Lit(ref lit) = expr {
        match &lit.lit {
            Lit::Float(f) => f
                .base10_parse()
                .map_err(|_| Error::CouldNotConvertFromExpression(lit.lit.span())),
            Lit::Int(i) => i
                .base10_parse()
                .map_err(|_| Error::CouldNotConvertFromExpression(lit.lit.span())),
            _ => return Err(Error::CouldNotConvertFromExpression(lit.lit.span())),
        }
    } else {
        Err(Error::CouldNotConvertFromExpression(expr.span()))
    }
}

impl TryFrom<&Expression> for f64 {
    type Error = Error;

    fn try_from(expr: &Expression) -> Result<Self> {
        expr_to_f64(&expr.inner)
    }
}

impl TryFrom<Expression> for f64 {
    type Error = Error;

    fn try_from(expr: Expression) -> Result<Self> {
        expr_to_f64(&expr.inner)
    }
}

impl TryFrom<&Expression> for f32 {
    type Error = Error;

    fn try_from(expr: &Expression) -> Result<Self> {
        expr_to_f32(&expr.inner)
    }
}

impl TryFrom<Expression> for f32 {
    type Error = Error;

    fn try_from(expr: Expression) -> Result<Self> {
        expr_to_f32(&expr.inner)
    }
}

impl From<Expression> for Expr {
    fn from(expr: Expression) -> Self {
        expr.inner
    }
}

impl AsRef<syn::Expr> for Expression {
    fn as_ref(&self) -> &syn::Expr {
        &self.inner
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inner = &self.inner;
        write!(f, "{}", quote!(#inner).to_string())
    }
}

impl std::fmt::Debug for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inner = &self.inner;
        write!(f, "{}", quote!(#inner).to_string())
    }
}

#[macro_export]
macro_rules! expr {
    ($e : expr) => {{
        let synexpr: syn::Expr = syn::parse_quote!($e);
        $crate::Expression::from(synexpr)
    }};
}

impl Expression {
    pub fn span(&self) -> Span {
        self.inner.span()
    }

    /// Generate a number from a literal expression.
    /// ```
    /// use doctor_syn::{Expression, expr};
    /// assert!(expr!("hello").is_lit());
    /// ```
    pub fn is_lit(&self) -> bool {
        match self.inner {
            Expr::Lit(_) => true,
            _ => false,
        }
    }

    /// Substitute the occurance of certain variables with an expression.
    /// ```
    /// use doctor_syn::{expr, vars};
    ///
    /// assert_eq!(expr!(x + 1).subst(vars!(x=1)).unwrap(), expr!(1 + 1));
    /// ```
    pub fn subst(&self, variables: VariableList) -> Result<Expression> {
        Ok(Subst { variables }.visit_expr(&self.inner)?.into())
    }

    /// Eval constant expressions into a single expression using a particular floating point type.
    /// ```
    /// use doctor_syn::{expr};
    ///
    /// assert_eq!(expr!(1 + 1).eval_float::<f64>().unwrap(), 2.0);
    /// ```
    pub fn eval_float<T: Evaluateable>(&self) -> Result<T> {
        let expr: Expr = Eval::<T> {
            datatype: std::marker::PhantomData,
        }
        .visit_expr(&self.inner)?;
        Ok(Expression::from(expr).try_into()?)
    }

    /// Return a polynomial approximation of a single variable expression.
    /// The polynomial is in a canonical form t[k] . mul_add( x, t[k-1]) ...  . mul_add( x, t[0])
    /// This is the most accurate and highest throughput form on most processors.
    ///
    /// ```
    /// use doctor_syn::{expr, name};
    /// use std::f64::consts::PI;
    ///
    /// assert_eq!(expr!(x).approx(4, 0.0, 1.0, name!(x)).unwrap(), expr!(1f64 . mul_add (x , 0f64)));
    /// ```
    pub fn approx<T: Evaluateable>(
        &self,
        num_terms: usize,
        xmin: T,
        xmax: T,
        variable: Name,
    ) -> Result<Expression> {
        approx(self, num_terms, xmin, xmax, variable)
    }
}
