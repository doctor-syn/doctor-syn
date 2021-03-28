
use proc_macro2::Span;

#[derive(Debug)]
pub enum Error {
    UnsupportedExpr(Span),
    UnsupportedMethod(Span),
    UnsupportedStatement(Span),
    UnsuportedClosureArgument(Span),
    BlockMustHaveOneStatement(Span),
    BadAttribute(Span),
    NotFound(Span),
    TriedToConvertLiteralToNumber(Span),
    CouldNotParse(Span),
}

