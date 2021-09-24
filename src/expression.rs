use crate::bdmath::*;
use crate::error::{Error, Result};
use crate::transformation::{
    approx::approx, collect::Collect, eval::Eval, expand::Expand, paren::Paren, subst::Subst,
    use_number_type::UseNumberType,
};
use crate::visitor::Visitor;
use crate::{Name, VariableList};
use proc_macro2::Span;
use quote::quote;
use std::convert::{TryFrom, TryInto};
use syn::spanned::Spanned;
use syn::{parse_quote, Expr, ExprLit, Lit};

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

impl From<bool> for Expression {
    fn from(expr: bool) -> Self {
        let e: Expr = if expr {
            parse_quote!(true)
        } else {
            parse_quote!(false)
        };
        Self { inner: e }
    }
}

// impl TryFrom<f64> for Expression {
//     type Error = Error;

//     fn try_from(val: f64) -> Result<Self> {
//         let s = format!("{}", val);
//         let inner: ExprLit = syn::parse_str(s.as_str()).map_err(|_| Error::CouldNotParse(s))?;
//         Ok(Self {
//             inner: inner.into(),
//         })
//     }
// }

// impl TryFrom<f32> for Expression {
//     type Error = Error;

//     fn try_from(val: f32) -> Result<Self> {
//         let s = format!("{}", val);
//         let inner: ExprLit = syn::parse_str(s.as_str()).map_err(|_| Error::CouldNotParse(s))?;
//         Ok(Self {
//             inner: inner.into(),
//         })
//     }
// }

impl From<f64> for Expression {
    fn from(val: f64) -> Self {
        let bd = BigDecimal::from_f64(val).unwrap();
        Expression::from(bd)
    }
}

impl From<f32> for Expression {
    fn from(val: f32) -> Self {
        let bd = BigDecimal::from_f32(val).unwrap();
        Expression::from(bd)
    }
}

impl From<BigDecimal> for Expression {
    fn from(val: BigDecimal) -> Self {
        let s = val.to_string();
        let inner = ExprLit {
            attrs: Vec::new(),
            lit: syn::LitFloat::new(s.as_str(), Span::call_site()).into(),
        }
        .into();
        Self { inner }
    }
}

// impl TryFrom<BigDecimal> for Expression {
//     type Error = Error;

//     fn try_from(val: BigDecimal) -> Result<Self> {
//         let s = format!("{}", val);
//         let inner: ExprLit =
//             syn::parse_str(s.as_str()).map_err(|_| Error::CouldNotParse(Span::call_site()))?;
//         Ok(Self {
//             inner: inner.into(),
//         })
//     }
// }

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

impl TryFrom<&Expression> for BigDecimal {
    type Error = Error;

    fn try_from(expr: &Expression) -> Result<Self> {
        expr_to::<BigDecimal>(&expr.inner)
    }
}

impl TryFrom<Expression> for BigDecimal {
    type Error = Error;

    fn try_from(expr: Expression) -> Result<Self> {
        expr_to::<BigDecimal>(&expr.inner)
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
        std::fmt::Debug::fmt(&self.inner, f)
    }
}

macro_rules! make_op {
    ($fname: ident, $op: tt) => {
        fn $fname(lhs: Expression, rhs: Expression ) -> Expression {
            if lhs.is_numeric() && rhs.is_numeric() {
                let lhs : BigDecimal = lhs.try_into().unwrap();
                let rhs : BigDecimal = rhs.try_into().unwrap();
                (lhs $op rhs).try_into().unwrap()
            } else {
                let lhs = lhs.as_ref();
                let rhs = rhs.as_ref();
                let res : Expr = parse_quote!((#lhs) + (#rhs));
                res.into()
            }
        }
    }
}

make_op!(add, +);
make_op!(sub, -);
make_op!(mul, *);
make_op!(div, /);

impl std::ops::Add for Expression {
    type Output = Self;

    /// Adding expressions uses BigDecimal if they are numeric
    /// or (lhs) + (rhs) otherwise.
    ///
    /// ```
    /// use doctor_syn::{Expression, expr};
    /// assert_eq!(expr!(1.234) + expr!(2), expr!(3.234));
    /// ```
    fn add(self, rhs: Self) -> Self::Output {
        add(self, rhs)
    }
}

impl std::ops::Mul for Expression {
    type Output = Self;

    /// Subtracting expressions uses BigDecimal if they are numeric
    /// or (lhs) - (rhs) otherwise.
    ///
    /// ```
    /// use doctor_syn::{Expression, expr};
    /// assert_eq!(expr!(3.234) - expr!(2), expr!(1.234));
    /// ```
    fn mul(self, rhs: Self) -> Self::Output {
        mul(self, rhs)
    }
}

impl std::ops::Div for Expression {
    type Output = Self;

    /// Adding expressions uses BigDecimal if they are numeric
    /// or (lhs) + (rhs) otherwise.
    ///
    /// ```
    /// use doctor_syn::{Expression, expr};
    /// assert_eq!(expr!(1.234) + expr!(2), expr!(3.234));
    /// ```
    fn div(self, rhs: Self) -> Self::Output {
        div(self, rhs)
    }
}

impl std::ops::Sub for Expression {
    type Output = Self;

    /// Subtracting expressions uses BigDecimal if they are numeric
    /// or (lhs) - (rhs) otherwise.
    ///
    /// ```
    /// use doctor_syn::{Expression, expr};
    /// assert_eq!(expr!(3.234) - expr!(2), expr!(1.234));
    /// ```
    fn sub(self, rhs: Self) -> Self::Output {
        sub(self, rhs)
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
        let inner: Expr = syn::parse_str(s).map_err(|_| Error::CouldNotParse(s.to_owned()))?;
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

    /// Return true if this is a literal.
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

    /// Return true if this is a numeric literal.
    /// ```
    /// use doctor_syn::{Expression, expr};
    /// assert!(expr!(1).is_numeric());
    /// assert!(expr!(1.0).is_numeric());
    /// ```
    pub fn is_numeric(&self) -> bool {
        match self.inner {
            Expr::Lit(ExprLit {
                lit: Lit::Int(_), ..
            }) => true,
            Expr::Lit(ExprLit {
                lit: Lit::Float(_), ..
            }) => true,
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

    /// Eval constant expressions into a single expression.
    /// ```
    /// use doctor_syn::{expr};
    ///
    /// assert_eq!(expr!(-(1)).eval(20).unwrap(), expr!(0 - 1).eval(20).unwrap());
    /// assert_eq!(expr!(1 + 1).eval(20).unwrap(), expr!(2 + 0).eval(20).unwrap());
    /// assert_eq!(expr!(1 - 1).eval(20).unwrap(), expr!(0 + 0).eval(20).unwrap());
    /// assert_eq!(expr!(2 * 2).eval(20).unwrap(), expr!(4 + 0).eval(20).unwrap());
    /// assert_eq!(expr!(100 / 10).eval(20).unwrap(), expr!(10 + 0).eval(20).unwrap());
    /// assert!(expr!(x + 1).eval(20).is_err());
    /// ```
    pub fn eval(&self, num_digits: i64) -> Result<Expression> {
        let expr: Expr = Eval { num_digits }.visit_expr(&self.inner)?;
        Ok(Expression::from(expr))
    }

    /// Return a polynomial approximation of a single variable expression.
    /// The polynomial is in a canonical form t[k] . mul_add( x, t[k-1]) ...  . mul_add( x, t[0])
    /// This is the most accurate and highest throughput form on most processors.
    ///
    /// ```ignore
    /// use doctor_syn::{expr, name, Parity};
    ///
    /// let e = expr!(x).approx(2, 0.0, 1.0, name!(x), Parity::Neither, num_digits_for(num_bits)).unwrap();
    /// let expected = expr!(1f64 . mul_add (x , 0f64));
    /// assert_eq!(e, expected));
    /// ```
    pub fn approx(
        &self,
        num_terms: usize,
        xmin: f64,
        xmax: f64,
        variable: Name,
        parity: Parity,
        num_digits: i64,
    ) -> Result<Expression> {
        approx(self, num_terms, xmin, xmax, variable, parity, num_digits)
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
    /// assert_eq!(expr!(1.0f64).use_number_type("f32_hex").unwrap(), expr!(f32 :: from_bits (1065353216u32)));
    /// ```
    pub fn use_number_type(&self, number_type: &str) -> Result<Expression> {
        Ok(UseNumberType { number_type }
            .visit_expr(&self.inner)?
            .into())
    }
}
