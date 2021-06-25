use proc_macro2::Span;
use crate::Expression;

#[derive(Debug)]
pub enum Error {
    UnsupportedExpr(Span),
    // UnsupportedMethod(Span),
    // UnsupportedStatement(Span),
    // UnsuportedClosureArgument(Span),
    // BlockMustHaveOneStatement(Span),
    // BadAttribute(Span),
    NotFound(Span),
    CouldNotConvertToExpression(Span),
    CouldNotConvertFromExpression(Span),
    CouldNotParse(Span),
    CouldNotEvaulate(Expression),
    WrongNumberOfTerms(Span),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<syn::Error> for Error {
    fn from(_: syn::Error) -> Self {
        Error::CouldNotParse(Span::call_site())
    }
}
