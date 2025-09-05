use std::collections::HashSet;

use crate::{
    binrw::parser::{attrs, TrySet},
    meta_types::KeywordToken,
};
use syn::{punctuated::Punctuated, Token, WherePredicate};

#[derive(Clone, Debug)]
pub(crate) enum Bound {
    Implicit(HashSet<syn::TypePath>),
    Explicit(Punctuated<WherePredicate, Token![,]>),
}

impl Default for Bound {
    fn default() -> Self {
        Self::Implicit(HashSet::default())
    }
}

impl From<attrs::Bound> for Bound {
    fn from(bound: attrs::Bound) -> Self {
        Self::Explicit(bound.fields)
    }
}

impl<T: Into<Bound> + KeywordToken> TrySet<Bound> for T {
    fn try_set(self, to: &mut Bound) -> syn::Result<()> {
        *to = self.into();
        Ok(())
    }
}
