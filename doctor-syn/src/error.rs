use proc_macro2::Span;

#[derive(Debug)]
pub enum Error {
    UnsupportedExpr(Span),
    UnsupportedCodegen(String),
    UndefinedVariable(String),

    // UnsupportedMethod(Span),
    // UnsupportedStatement(Span),
    // UnsuportedClosureArgument(Span),
    // BlockMustHaveOneStatement(Span),
    // BadAttribute(Span),
    NotFound(Span),
    CouldNotConvertToExpression(Span),
    CouldNotConvertFromExpression(Span),
    CouldNotParse(String),
    CouldNotEvaulate(String),
    WrongNumberOfTerms(Span),
    Expected32or64bits,
    Overflow,
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<syn::Error> for Error {
    fn from(_: syn::Error) -> Self {
        Error::CouldNotParse(String::new())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::*;
        match self {
            UnsupportedExpr(span) => write!(f, "UnsupportedExpr {:?}", span),
            UnsupportedCodegen(tokens) => write!(f, "UnsupportedCodegen {}", tokens.to_string()),
            UndefinedVariable(tokens) => write!(f, "UndefinedVariable {}", tokens.to_string()),
            NotFound(span) => write!(f, "NotFound {:?}", span),
            CouldNotConvertToExpression(span) => {
                write!(f, "CouldNotConvertToExpression {:?}", span)
            }
            CouldNotConvertFromExpression(span) => {
                write!(f, "CouldNotConvertFromExpression {:?}", span)
            }
            CouldNotParse(span) => write!(f, "CouldNotParse {:?}", span),
            CouldNotEvaulate(str) => write!(f, "CouldNotEvaulate({})", str),
            WrongNumberOfTerms(span) => write!(f, "WrongNumberOfTerms {:?}", span),
            Overflow => write!(f, "Overflow"),
            Expected32or64bits => write!(f, "Expected 32 or 64 bits"),
        }
    }
}

impl std::error::Error for Error {}
