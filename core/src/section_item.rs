use syn;

use crate::section::Section;

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SectionItem {
    Sep(Section),
    Stmt(syn::Stmt),
}

impl SectionItem {
    pub fn is_stmt(&self) -> bool {
        match self {
            Self::Stmt(_) => true,
            _ => false,
        }
    }

    pub fn stmt(&self) -> Option<syn::Stmt> {
        match self {
            Self::Stmt(inner) => Some(inner.clone()),
            _ => None,
        }
    }
}
