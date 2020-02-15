use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::parse::{self, Parse, ParseStream};

use crate::scope::Scope;
use crate::section_body::SectionBody;
use crate::section_item::SectionItem;
use crate::section_keyword::SectionKeyword;
use crate::utils;
use utils::extract_literal_string;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Section {
    section_kind: SectionKeyword,
    name: String,
    body: SectionBody,
}

impl Section {
    pub fn new(
        section_kind: SectionKeyword,
        name: impl ToString,
        body: SectionBody,
    ) -> Self {
        Self {
            section_kind,
            name: name.to_string(),
            body,
        }
    }

    fn quote_name(&self) -> Ident {
        let name = utils::escape_name(&self.name);
        let kind = self.section_kind.to_name();

        let name = if kind.is_empty() {
            name
        } else {
            format!("{}_{}", kind, name)
        };

        Ident::new(&name, Span::call_site())
    }

    pub fn quote_inner(&self, scope: Scope) -> TokenStream {
        let mut token_stream = TokenStream::default();

        self.to_tokens_inner(scope, &mut token_stream);

        token_stream
    }

    pub fn peek(input: ParseStream) -> bool {
        SectionKeyword::peek(input)
    }

    fn to_tokens_inner(&self, scope: Scope, tokens: &mut TokenStream) {
        if self.body.is_top_level() {
            let my_stmts: Vec<_> =
                self.body.items().iter().filter_map(|i| i.stmt()).collect();

            let name = self.quote_name();

            let inner = scope.quote_with(&my_stmts);

            tokens.append_all(quote! {
                #[test]
                fn #name() {
                    #inner
                }
            });

            return;
        }

        let mut stream = vec![];

        for (idx, item) in self.body.items().iter().enumerate() {
            if let SectionItem::Sep(section) = item {
                let sb = self.body.get_stmts_before(idx);
                let sa = self.body.get_stmts_after(idx);

                let new_scope = scope.push(&sb, &sa);
                let inner = section.quote_inner(new_scope);

                stream.push(inner);
            }
        }

        let name = self.quote_name();

        tokens.append_all(quote! {
            mod #name {
                use super::*;

                #(#stream)*
            }
        });
    }
}

impl ToTokens for Section {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let scope = Scope::empty();

        self.to_tokens_inner(scope, tokens);
    }
}

impl Parse for Section {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let section_keyword: SectionKeyword = input.parse()?;
        let name: syn::Lit = input.parse()?;
        let name = extract_literal_string(name).ok_or_else(|| {
            parse::Error::new(Span::call_site(), "Invalid section literal")
        })?;

        let content;
        syn::braced!(content in input);
        let inner_body = content.parse::<SectionBody>()?;

        Ok(Section::new(section_keyword, name, inner_body))
    }
}
