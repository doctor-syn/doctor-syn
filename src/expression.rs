use crate::error::{Error, Result};
use crate::transformation::{
    approx::approx, collect::Collect, eval::Eval, expand::Expand, paren::Paren, subst::Subst,
    use_suffix::UseSuffix,
};
use crate::visitor::Visitor;
use crate::Evaluateable;
use crate::{Name, VariableList};
use proc_macro2::Span;
use quote::quote;
use std::convert::{TryFrom, TryInto};
use syn::spanned::Spanned;
use syn::{Expr, ExprLit, Lit};

pub enum Parity {
    Odd,
    Even,
    Neither,
}

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

fn expr_to<N>(expr: &syn::Expr) -> Result<N>
where
    N: std::str::FromStr,
    N::Err: std::fmt::Display,
{
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
        expr_to::<f64>(&expr.inner)
    }
}

impl TryFrom<Expression> for f64 {
    type Error = Error;

    fn try_from(expr: Expression) -> Result<Self> {
        expr_to::<f64>(&expr.inner)
    }
}

impl TryFrom<&Expression> for f32 {
    type Error = Error;

    fn try_from(expr: &Expression) -> Result<Self> {
        expr_to::<f32>(&expr.inner)
    }
}

impl TryFrom<Expression> for f32 {
    type Error = Error;

    fn try_from(expr: Expression) -> Result<Self> {
        expr_to::<f32>(&expr.inner)
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
        // write!(f, "{:?}", quote!(#inner))
    }
}

#[macro_export]
macro_rules! expr {
    ($e : expr) => {{
        let synexpr: $crate::syn::Expr = $crate::syn::parse_quote!($e);
        $crate::Expression::from(synexpr)
    }};
}

impl std::str::FromStr for Expression {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let inner: Expr = syn::parse_str(s).map_err(|_| Error::CouldNotParse(Span::call_site()))?;
        Ok(Self { inner })
    }
}

impl Expression {
    pub fn span(&self) -> Span {
        self.inner.span()
    }

    pub fn into_inner(self) -> Expr {
        self.inner
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
    /// assert_eq!(expr!(1 + 1).eval::<f64>().unwrap(), 2.0);
    /// ```
    pub fn eval<T: Evaluateable>(&self) -> Result<T> {
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
    /// ```ignore
    /// use doctor_syn::{expr, name, Parity};
    ///
    /// let e = expr!(x).approx(2, 0.0, 1.0, name!(x), Parity::Neither).unwrap();
    /// let expected = expr!(1f64 . mul_add (x , 0f64));
    /// assert_eq!(e, expected));
    /// ```
    pub fn approx<T: Evaluateable>(
        &self,
        num_terms: usize,
        xmin: T,
        xmax: T,
        variable: Name,
        parity: Parity,
    ) -> Result<Expression> {
        approx(self, num_terms, xmin, xmax, variable, parity)
    }

    /// Expand an expression.
    ///
    /// ```
    /// use doctor_syn::{expr, Result};
    /// || -> Result<()> {
    ///    // Unary
    ///    assert_eq!(expr!(- - x).expand()?, expr!(x));
    ///    assert_eq!(expr!(-(x+1)).expand()?, expr!(-x-1));
    ///
    ///    // Binary add/sub
    ///    assert_eq!(expr!((x+1)+(x+1)).expand()?, expr!(x + 1 + x + 1));
    ///    assert_eq!(expr!((x+1)+((x+1)+(x+1))).expand()?, expr!(x + 1 + x + 1 + x + 1));
    ///    assert_eq!(expr!((x+1)-(x+1)).expand()?, expr!(x + 1 - x - 1));
    ///    assert_eq!(expr!((x+1)-((x+1)-(x+1))).expand()?, expr!(x + 1 - x - 1 + x + 1));
    ///    assert_eq!(expr!((x+1)-((x+1)-(-x+1))).expand()?, expr!(x + 1 - x - 1 - x + 1));
    ///
    ///    // Binary mul
    ///    assert_eq!(expr!(x*x).expand()?, expr!(x * x));
    ///    assert_eq!(expr!(x*(x+1)).expand()?, expr!(x * x + x * 1));
    ///    assert_eq!(expr!((x+1)*x).expand()?, expr!(x * x + 1 * x));
    ///    assert_eq!(expr!((x+1)*(x+1)).expand()?, expr!(x * x + x * 1 + 1 * x + 1 * 1));
    ///    assert_eq!(expr!((x+1)*(x+1)*(x+1)).expand()?, expr!(x * x * x + x * x * 1 + x * 1 * x + x * 1 * 1 + 1 * x * x + 1 * x * 1 + 1 * 1 * x + 1 * 1 * 1));
    ///    Ok(())
    /// }();
    /// ```
    pub fn expand(&self) -> Result<Expression> {
        Ok(Expand {}.visit_expr(&self.inner)?.into())
    }

    /// Collect terms assuming commutativity.
    ///
    /// ```
    /// use doctor_syn::{expr, Result};
    /// || -> Result<()> {
    ///    
    ///    Ok(())
    /// }();
    /// ```
    pub fn collect_terms(&self, variable: Name) -> Result<Expression> {
        Ok(Collect { variable }.visit_expr(&self.inner)?.into())
    }

    /// Parenthesise operators of operators.
    ///
    /// ```
    /// use doctor_syn::{expr};
    ///
    /// assert_eq!(expr!(-x).paren().unwrap(), expr!(-x));
    /// assert_eq!(expr!(x+1).paren().unwrap(), expr!(x+1));
    /// assert_eq!(expr!(x+1+1).paren().unwrap(), expr!((x+1)+1));
    /// assert_eq!(expr!(x+1+1+1).paren().unwrap(), expr!(((x+1)+1)+1));
    /// assert_eq!(expr!(2*x+y).paren().unwrap(), expr!((2*x)+y));
    /// ```
    pub fn paren(&self) -> Result<Expression> {
        Ok(Paren {}.visit_expr(&self.inner)?.into())
    }

    /// Change the suffix of floatng point numbers.
    ///
    /// ```
    /// use doctor_syn::{expr};
    ///
    /// assert_eq!(expr!(1.0f64).use_suffix(Some("f32".to_string())).unwrap(), expr!(1.0_f32));
    /// ```
    pub fn use_suffix(&self, float_suffix: Option<String>) -> Result<Expression> {
        Ok(UseSuffix { float_suffix }.visit_expr(&self.inner)?.into())
    }
}
