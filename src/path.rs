
use quote::quote;

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct Path {
    pub (crate) inner: syn::Path,
}

impl From<syn::Path> for Path {
    fn from(path: syn::Path) -> Self {
        Self { inner: path }
    }
}

impl From<Path> for syn::Path {
    fn from(path: Path) -> Self {
        path.inner
    }
}

impl AsRef<syn::Path> for Path {
    fn as_ref(&self) -> &syn::Path {
        &self.inner
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inner = &self.inner;
        write!(f, "{}", quote!(#inner).to_string())
    }
}

#[macro_export]
macro_rules! path {
    ($e : expr) => {
        $crate::Path { inner: syn::parse_quote!($e), }
    }
}
