use crate::error::{Error, Result};
use proc_macro2::Span;
use syn::spanned::Spanned;
use syn::{parse_str, Expr, Lit};
use quote::quote;
use crate::visitor::Visitor;
use crate::transformation::Subst;
use crate::transformation::Fold;
use crate::VariableList;

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub (crate) inner: Expr,
}

impl From<Expr> for Expression {
    fn from(expr: Expr) -> Self {
        Self { inner: expr }
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
    ($e : expr) => {
        {
            let synexpr : syn::Expr = syn::parse_quote!($e);
            $crate::Expression::from(synexpr)
        }
    }
}



impl Expression {
    /// Generate an expression from a number.
    ///
    /// ```
    /// use doctor_syn::{Expression, expr};
    /// let e = Expression::from_number(1).unwrap();
    /// assert!(e.is_lit());
    /// assert_eq!(e, expr!(1));
    /// ```
    pub fn from_number<T: std::fmt::Debug>(value: T) -> Result<Self> {
        let s = format!("{:?}", value);
        Ok(Self {
            inner: parse_str(s.as_str()).map_err(|_| Error::CouldNotParse(Span::call_site()))?,
        })
    }

    /// Generate a number from a literal expression.
    /// ```
    /// use doctor_syn::{Expression, expr};
    /// let one : u32 = expr!(1).as_number().unwrap();
    /// assert_eq!(one, 1);
    /// ```
    pub fn as_number<T>(&self) -> Result<T>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Display,
    {
        if let Expr::Lit(ref lit) = self.inner {
            match &lit.lit {
                Lit::Float(f) => f
                    .base10_parse()
                    .map_err(|_| Error::CouldNotConvert(lit.lit.span())),
                Lit::Int(i) => i
                    .base10_parse()
                    .map_err(|_| Error::CouldNotConvert(lit.lit.span())),
                _ => return Err(Error::CouldNotConvert(lit.lit.span())),
            }
        } else {
            Err(Error::CouldNotConvert(self.inner.span()))
        }
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

    /// Fold constant expressions into a single expression.
    /// ```
    /// use doctor_syn::{expr};
    /// 
    /// assert_eq!(expr!(1 + 1).fold(10).unwrap(), expr!(2.0));
    /// ```
    pub fn fold(&self, max_digits: u32) -> Result<Expression> {
        Ok(Fold { max_digits }.visit_expr(&self.inner)?.into())
    }
}

