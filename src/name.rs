
use quote::quote;

#[derive(Clone, PartialEq, Hash, Eq)]
pub struct Name {
    pub (crate) inner: syn::Path,
}

impl From<syn::Path> for Name {
    fn from(path: syn::Path) -> Self {
        Self { inner: path }
    }
}

impl From<Name> for syn::Path {
    fn from(path: Name) -> Self {
        path.inner
    }
}

impl AsRef<syn::Path> for Name {
    fn as_ref(&self) -> &syn::Path {
        &self.inner
    }
}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inner = &self.inner;
        write!(f, "{}", quote!(#inner).to_string())
    }
}

impl std::fmt::Debug for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inner = &self.inner;
        write!(f, "{}", quote!(#inner).to_string())
    }
}

#[macro_export]
macro_rules! name {
    ($e : expr) => {
        {
            let path : syn::Path = syn::parse_quote!($e);
            $crate::Name::from(path)
        }
    }
}
