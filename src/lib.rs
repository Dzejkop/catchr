// #![feature(type_alias_enum_variants)]

extern crate proc_macro;

use proc_macro::TokenStream;

use proc_macro2::{Ident, Span};
use quote::quote;
use syn::export::TokenStream2;
use syn::parse::{self, Parse, ParseStream};
use syn::parse_macro_input;

mod kw {
    syn::custom_keyword!(when);
    syn::custom_keyword!(then);
    syn::custom_keyword!(given);
    syn::custom_keyword!(case);
}

#[derive(Clone)]
struct Scope {
    before: Vec<syn::Stmt>,
    inner: Option<Box<Scope>>,
    after: Vec<syn::Stmt>,
}

fn extract_literal_string(lit: syn::Lit) -> String {
    match lit {
        syn::Lit::Str(s) => {
            let mut s = s.value().to_lowercase();

            s = s.replace(" ", "_");
            s = s.replace(",", "");
            s = s.replace("!", "");

            s
        }
        _ => "".to_string(),
    }
}

impl Scope {
    pub fn new(before: &[syn::Stmt], after: &[syn::Stmt]) -> Self {
        Self {
            before: Vec::from(before),
            inner: None,
            after: Vec::from(after),
        }
    }

    pub fn push(&self, before: &[syn::Stmt], after: &[syn::Stmt]) -> Self {
        let mut new_scope = self.clone();

        if let Some(inner) = new_scope.inner.as_mut() {
            new_scope = inner.push(before, after);
        } else {
            new_scope.inner = Some(Box::new(Scope::new(before, after)));
        }

        new_scope
    }

    fn quote_with(&self, stmts: &[syn::Stmt]) -> TokenStream2 {
        let Scope {
            before,
            inner,
            after,
        } = &self;

        if let Some(inner) = inner.as_ref() {
            let inner = inner.quote_with(stmts);

            quote! {
                #(#before)*
                {
                    #inner
                }
                #(#after)*
            }
        } else {
            quote! {
                #(#before)*
                {
                    #(#stmts)*
                }
                #(#after)*
            }
        }
    }
}

#[derive(Debug, Clone)]
enum CatchrItem {
    Sep(NamedCatchr),
    Stmt(syn::Stmt),
}

impl CatchrItem {
    fn is_stmt(&self) -> bool {
        match self {
            Self::Stmt(_) => true,
            _ => false,
        }
    }

    fn stmt(&self) -> Option<syn::Stmt> {
        match self {
            Self::Stmt(inner) => Some(inner.clone()),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
struct NamedCatchr {
    name: String,
    catchr: Catchr,
}

impl NamedCatchr {
    fn quote_inner(&self, scope: Scope) -> TokenStream2 {
        if self.catchr.is_top_level() {
            let my_stmts: Vec<_> = self
                .catchr
                .items
                .iter()
                .filter_map(|i| i.stmt().clone())
                .collect();

            let name = self.name.clone();
            let name = Ident::new(&name, Span::call_site());

            let inner = scope.quote_with(&my_stmts);

            return quote! {
                #[test]
                fn #name() {
                    #inner
                }
            };
        }

        let mut stream = vec![];

        for (idx, item) in self.catchr.items.iter().enumerate() {
            if let CatchrItem::Sep(catchr) = item {
                let sb = self.catchr.get_stmts_before(idx);
                let sa = self.catchr.get_stmts_after(idx);

                let new_scope = scope.push(&sb, &sa);
                let inner = catchr.quote_inner(new_scope);

                stream.push(inner);
            }
        }

        let name = self.name.clone();
        let name = Ident::new(&name, Span::call_site());

        quote! {
            mod #name {
                use super::*;

                #(#stream)*
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Catchr {
    items: Vec<CatchrItem>,
}

impl Catchr {
    fn push_stmt(&mut self, stmt: syn::Stmt) {
        self.items.push(CatchrItem::Stmt(stmt));
    }

    fn push_when(&mut self, item: NamedCatchr) {
        self.items.push(CatchrItem::Sep(item));
    }

    fn push_then(&mut self, item: NamedCatchr) {
        self.items.push(CatchrItem::Sep(item));
    }

    fn is_top_level(&self) -> bool {
        self.items.iter().all(|item| item.is_stmt())
    }

    fn get_stmts_before(&self, idx: usize) -> Vec<syn::Stmt> {
        self.items
            .iter()
            .take(idx)
            .filter_map(|i| i.stmt().clone())
            .collect()
    }

    fn get_stmts_after(&self, idx: usize) -> Vec<syn::Stmt> {
        self.items
            .iter()
            .skip(idx + 1)
            .filter_map(|i| i.stmt().clone())
            .collect()
    }

    fn quote_inner(&self, scope: Scope) -> TokenStream2 {
        let mut stream = vec![];

        for (idx, item) in self.items.iter().enumerate() {
            if let CatchrItem::Sep(catchr) = item {
                let sb = self.get_stmts_before(idx);
                let sa = self.get_stmts_after(idx);

                let new_scope = scope.push(&sb, &sa);
                let inner = catchr.quote_inner(new_scope);

                stream.push(quote! {
                    mod catchr_tests {
                        use super::*;

                        #inner
                    }
                });
            }
        }

        quote! {
            #(#stream)*
        }
    }

    fn quote(&self) -> TokenStream2 {
        let empty_scope = Scope::new(&[], &[]);
        self.quote_inner(empty_scope)
    }
}

fn parse_braced_content(input: ParseStream) -> parse::Result<Catchr> {
    let content;
    syn::braced!(content in input);

    content.parse::<Catchr>()
}

impl Parse for Catchr {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let mut catchr = Catchr { items: vec![] };

        loop {
            let lk = input.lookahead1();
            if lk.peek(kw::when) {
                input.parse::<kw::when>()?;
                let when = input.parse::<syn::Lit>()?;
                let when = extract_literal_string(when);
                let when = format!("when_{}", when);

                let inner_catchr = parse_braced_content(input)?;

                let inner_catchr = NamedCatchr {
                    name: when,
                    catchr: inner_catchr,
                };

                catchr.push_when(inner_catchr);
            } else if lk.peek(kw::then) {
                input.parse::<kw::then>()?;
                let then = input.parse::<syn::Lit>()?;

                let then = extract_literal_string(then);

                let then = format!("then_{}", then);

                let inner_catchr = parse_braced_content(input)?;

                let inner_catchr = NamedCatchr {
                    name: then,
                    catchr: inner_catchr,
                };

                catchr.push_then(inner_catchr);
            } else if lk.peek(kw::case) {
                input.parse::<kw::case>()?;
                let then = input.parse::<syn::Lit>()?;

                let then = extract_literal_string(then);

                let then = format!("case_{}", then);

                let inner_catchr = parse_braced_content(input)?;

                let inner_catchr = NamedCatchr {
                    name: then,
                    catchr: inner_catchr,
                };

                catchr.push_then(inner_catchr);
            } else if lk.peek(kw::given) {
                input.parse::<kw::given>()?;
                let then = input.parse::<syn::Lit>()?;

                let then = extract_literal_string(then);

                let then = format!("given_{}", then);

                let inner_catchr = parse_braced_content(input)?;

                let inner_catchr = NamedCatchr {
                    name: then,
                    catchr: inner_catchr,
                };

                catchr.push_then(inner_catchr);
            } else if input.is_empty() {
                break;
            } else {
                let next = input.parse::<syn::Stmt>()?;
                catchr.push_stmt(next);
            }
        }

        Ok(catchr)
    }
}

#[proc_macro]
pub fn catchr(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Catchr);

    let output = input.quote();

    let output = quote! {
        #[allow(unused)]
        #output
    };

    output.into()
}
