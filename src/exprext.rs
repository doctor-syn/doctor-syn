
use syn::{Expr, punctuated::Punctuated, Token};

pub trait ExprExt {
    fn is_lit(&self) -> bool;
    fn as_ident(&self) -> Option<String>;
    fn as_call(&self) -> Option<(&Expr, &Punctuated<Expr, Token![,]>)>;
}

impl ExprExt for Expr {
    /// Return true if this is a literal.
    /// ```
    /// use doctor_syn::{Expression, ExprExt, expr};
    /// assert!(expr!("hello").is_lit());
    /// ```
    fn is_lit(&self) -> bool {
        match self {
            Expr::Lit(_) => true,
            _ => false,
        }
    }

    /// Return Some(name) if this is an identifier.
    /// ```
    /// use doctor_syn::{Expression, ExprExt, expr};
    /// assert_eq!(expr!(hello).as_ident(), Some("hello".to_string()));
    /// ```
    fn as_ident(&self) -> Option<String> {
        match self {
            Expr::Path(syn::ExprPath { qself: None, ref path, .. }) => {
                if let Some(ident) = path.get_ident() {
                    Some(ident.to_string())
                } else {
                    None
                }
            },
            _ => None,
        }
    }

    /// Return Some((expr, args)) if this is a call.
    /// ```
    /// use doctor_syn::{Expression, ExprExt, expr};
    /// assert_eq!(expr!(hello(1)).as_call().and_then(|(expr, _)| expr.as_ident()), Some("hello".to_string()));
    /// ```
    fn as_call(&self) -> Option<(&Expr, &Punctuated<Expr, syn::Token![,]>)> {
        match self {
            Expr::Call(syn::ExprCall { ref func, ref args, .. }) => {
                Some((&func, &args))
            },
            _ => None,
        }
    }
}

