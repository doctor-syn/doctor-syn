use crate::{Expression, Name};

#[derive(Clone, PartialEq)]
pub struct VariableList {
    pub(crate) inner: Vec<(Name, Expression)>,
}

impl From<Vec<(Name, Expression)>> for VariableList {
    fn from(vec: Vec<(Name, Expression)>) -> VariableList {
        VariableList {
            inner: vec.into_iter().collect(),
        }
    }
}

impl VariableList {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn add_var(&mut self, name: Name, value: Expression) -> usize {
        let res = self.inner.len();
        self.inner.push((name, value));
        res
    }

    pub fn find(&self, name: &Name) -> Option<Expression> {
        if let Some((_, e)) = self.inner.iter().find(|(p, _)| p == name) {
            Some(e.clone())
        } else {
            None
        }
    }
}

#[macro_export]
macro_rules! vars {
    ($($i : ident = $e : expr),*) => {
        $crate::VariableList::from(::std::vec![$(($crate::name!($i), $crate::expr!($e))),*])
    }
}

impl std::fmt::Display for VariableList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "vars!(")?;
        let mut comma = "";
        for (p, e) in self.inner.iter() {
            write!(f, "{}{} = {}", comma, p, e)?;
            comma = ", ";
        }
        write!(f, ")")
    }
}

impl std::fmt::Debug for VariableList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "vars!(")?;
        let mut comma = "";
        for (p, e) in self.inner.iter() {
            write!(f, "{}{} = {}", comma, p, e)?;
            comma = ", ";
        }
        write!(f, ")")
    }
}

#[test]
fn test_vars() {
    use crate::vars;
    let v = vars!(x = 1, y = 2);
    assert_eq!(format!("{:?}", v), "vars!(x = 1, y = 2)");
}
