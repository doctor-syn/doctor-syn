
use crate::Expression;
use quote::quote;

#[derive(Clone, PartialEq)]
pub struct VariableList {
    pub (crate) inner: Vec<(syn::Path, Expression)>,
}

impl From<Vec<(syn::Path, crate::Expression)>> for VariableList {
    fn from(vec: Vec<(syn::Path, crate::Expression)>) -> VariableList {
        VariableList { inner: vec.into_iter().collect() }
    }
}

impl VariableList {
    pub fn find(&self, path: &syn::Path) -> Option<Expression> {
        if let Some((_, e)) = self.inner.iter().find(|(p, _)| p == path) {
            Some(e.clone())
        } else {
            None
        }
    }
}

#[macro_export]
macro_rules! vars {
    ($($i : ident = $e : expr),*) => {
        $crate::VariableList::from(vec![$((syn::parse_quote!($i), $crate::expr!($e))),*])
    }
}

impl std::fmt::Debug for VariableList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "vars!(")?;
        let mut comma = "";
        for (p,e) in self.inner.iter() {
            write!(f, "{}{} = {}", comma, quote!(#p).to_string(), e)?;
            comma = ", ";
        }
        write!(f, ")")
    }
}

#[test]
fn test_vars() {
    use crate::vars;
    let v = vars!(x=1, y=2);
    assert_eq!(format!("{:?}", v), "vars!(x = 1, y = 2)");
}
