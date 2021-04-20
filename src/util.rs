
use syn::{Expr, Lit};
use syn::spanned::Spanned;
use crate::error::Error;
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
            Lit::Float(f) => f.base10_parse().map_err(|_| Error::CouldNotConvert(lit.lit.span())),
            Lit::Int(i) => i.base10_parse().map_err(|_| Error::CouldNotConvert(lit.lit.span())),
            _ => return Err(Error::CouldNotConvert(lit.lit.span()))
        }
    } else {
        Err(Error::CouldNotConvert(expr.span()))
    }
}

pub fn parse_str<T : syn::parse::Parse>(s: &str) -> Result<T, Error> {
    syn::parse_str(s).map_err(|_| Error::CouldNotParse(Span::call_site()))
}
