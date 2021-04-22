use crate::error::{Error, Result};
use crate::transformation::Eval;
use crate::transformation::Subst;
use crate::visitor::Visitor;
use crate::{VariableList, Name};
use proc_macro2::Span;
use quote::quote;
use std::convert::{TryFrom, TryInto};
use syn::spanned::Spanned;
use syn::{Expr, Lit, ExprUnary, UnOp};
use num_traits::Float;

#[derive(Debug, Clone, PartialEq)]
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
        let inner = syn::parse_str(s.as_str()).map_err(|_| Error::CouldNotParse(Span::call_site()))?;
        Ok(Self { inner })
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
    } else if let Expr::Unary(ExprUnary { op: UnOp::Neg(_), ref expr, .. }) = expr {
        Ok(-expr_to_f64(expr)?)
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

#[macro_export]
macro_rules! expr {
    ($e : expr) => {{
        let synexpr: syn::Expr = syn::parse_quote!($e);
        $crate::Expression::from(synexpr)
    }};
}

impl Expression {
    // /// Generate an expression from a number.
    // ///
    // /// ```
    // /// use doctor_syn::{Expression, expr};
    // /// let e = Expression::from_number(1).unwrap();
    // /// assert!(e.is_lit());
    // /// assert_eq!(e, expr!(1));
    // /// ```
    // pub fn from_number<T: std::fmt::Display>(value: T) -> Result<Self> {
    //     let s = format!("{}", value);
    //     Ok(Self {
    //         inner: parse_str(s.as_str()).map_err(|_| Error::CouldNotParse(Span::call_site()))?,
    //     })
    // }

    // /// Generate a number from a literal expression.
    // /// ```
    // /// use doctor_syn::{Expression, expr};
    // /// let one : u32 = expr!(1).as_number().unwrap();
    // /// assert_eq!(one, 1);
    // /// ```
    // pub fn as_number<T>(&self) -> Result<T>
    // where
    //     T: std::str::FromStr,
    //     T::Err: std::fmt::Display,
    // {
    //     if let Expr::Lit(ref lit) = self.inner {
    //         match &lit.lit {
    //             Lit::Float(f) => f
    //                 .base10_parse()
    //                 .map_err(|_| Error::CouldNotConvert(lit.lit.span())),
    //             Lit::Int(i) => i
    //                 .base10_parse()
    //                 .map_err(|_| Error::CouldNotConvert(lit.lit.span())),
    //             _ => return Err(Error::CouldNotConvert(lit.lit.span())),
    //         }
    //     } else {
    //         Err(Error::CouldNotConvert(self.inner.span()))
    //     }
    // }

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
        println!("subst");
        Ok(Subst { variables }.visit_expr(&self.inner)?.into())
    }

    /// Eval constant expressions into a single expression using a particular floating point type.
    /// ```
    /// use doctor_syn::{expr};
    ///
    /// assert_eq!(expr!(1 + 1).eval_float::<f64>().unwrap(), 2.0);
    /// ```
    pub fn eval_float<T: TryFrom<Expression, Error=Error> + TryInto<Expression, Error=Error> + Float>(&self) -> Result<T> {
        let expr : Expr = Eval::<T> {
            datatype: std::marker::PhantomData,
        }
        .visit_expr(&self.inner)?;
        Ok(Expression::from(expr).try_into()?)
    }

    /// Return an approximation of a single variable expression.
    /// ```
    /// use doctor_syn::{expr, name};
    /// use std::f64::consts::PI;
    ///
    /// assert_eq!(expr!(x.sin()).approx(2, 0.0, PI*2.0, name!(x)).unwrap(), expr!(2.0));
    /// ```
    pub fn approx(
        &self,
        num_terms: u32,
        xmin: f64,
        xmax: f64,
        variable: Name,
    ) -> Result<Expression> {
        use std::f64::consts::PI;
        let a = (xmax + xmin) * 0.5;
        let b = PI / (num_terms - 1) as f64;
        let c = (xmax - xmin) * 0.5;
        let mut xvalues = Vec::new();
        let mut yvalues = Vec::new();
        for i in 0..num_terms {
            // *almost* Chebyshev nodes.
            let x = a - c * (i as f64 * b).cos();
            let mut vars = VariableList::new();
            vars.add_var(variable.clone(), x.try_into()?);
            let subst = self.subst(vars)?;
            println!("subst={}", subst);
            let y : f64 = subst.eval_float()?;
            xvalues.push(x);
            yvalues.push(y);
        }
        println!("{:?}", xvalues);
        println!("{:?}", yvalues);
        Ok(expr!(1))
    }
}
