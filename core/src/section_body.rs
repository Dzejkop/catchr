use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};
use syn::parse::{self, Parse, ParseStream};

use crate::catchr_mode::CatchrMode;
use crate::scope::Scope;
use crate::section::Section;
use crate::section_item::SectionItem;
use crate::section_keyword::SectionKeyword;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SectionBody {
    items: Vec<SectionItem>,
}

impl SectionBody {
    pub fn with_mode(mut self, test_attribute: CatchrMode) -> Self {
        self.items = self
            .items
            .into_iter()
            .map(|item| item.with_mode(test_attribute))
            .collect();

        self
    }

    pub fn empty() -> Self {
        Self { items: vec![] }
    }

    pub fn new(items: Vec<SectionItem>) -> Self {
        Self { items }
    }

    fn push_stmt(&mut self, stmt: syn::Stmt) {
        self.items.push(SectionItem::Stmt(stmt));
    }

    fn push_section(&mut self, item: Section) {
        self.items.push(SectionItem::Sep(item));
    }

    pub fn is_top_level(&self) -> bool {
        self.items.iter().all(|item| item.is_stmt())
    }

    pub fn get_stmts_before(&self, idx: usize) -> Vec<syn::Stmt> {
        self.items
            .iter()
            .take(idx)
            .filter_map(|i| i.stmt())
            .collect()
    }

    pub fn get_stmts_after(&self, idx: usize) -> Vec<syn::Stmt> {
        self.items
            .iter()
            .skip(idx + 1)
            .filter_map(|i| i.stmt())
            .collect()
    }

    pub fn items(&self) -> &[SectionItem] {
        &self.items
    }

    pub fn to_tokens_inner(&self, mut scope: Scope, tokens: &mut TokenStream) {
        let mut stream = vec![];

        for (idx, item) in self.items.iter().enumerate() {
            if let SectionItem::Sep(section) = item {
                let sb = self.get_stmts_before(idx);
                let sa = self.get_stmts_after(idx);

                scope.push_mut(&sb, &sa);
                let inner = section.quote_inner(scope.clone());

                stream.push(quote! {
                    mod catchr_scenarios {
                        use super::*;

                        #inner
                    }
                });
            }
        }

        tokens.append_all(stream);
    }
}

impl Parse for SectionBody {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let mut body = SectionBody { items: vec![] };

        loop {
            if SectionKeyword::peek(input) {
                let inner_section = input.parse()?;

                body.push_section(inner_section);
            } else if input.is_empty() {
                break;
            } else {
                let next = input.parse::<syn::Stmt>()?;
                body.push_stmt(next);
            }
        }

        Ok(body)
    }
}
