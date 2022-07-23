use crate::catchr_mode::CatchrMode;
use crate::section::Section;

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SectionItem {
    Sep(Section),
    Stmt(syn::Stmt),
}

impl SectionItem {
    pub fn with_mode(self, test_attribute: CatchrMode) -> Self {
        match self {
            SectionItem::Sep(section) => {
                SectionItem::Sep(section.with_mode(test_attribute))
            }
            stmt => stmt,
        }
    }

    pub fn is_stmt(&self) -> bool {
        matches!(self, Self::Stmt(_))
    }

    pub fn stmt(&self) -> Option<syn::Stmt> {
        match self {
            Self::Stmt(inner) => Some(inner.clone()),
            _ => None,
        }
    }
}
