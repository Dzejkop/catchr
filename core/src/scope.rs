use quote::quote;
use syn::export::TokenStream2;

#[derive(Clone, Debug)]
pub struct Scope {
    before: Vec<syn::Stmt>,
    inner: Option<Box<Scope>>,
    after: Vec<syn::Stmt>,
}

impl Scope {
    pub fn empty() -> Self {
        Self {
            before: vec![],
            inner: None,
            after: vec![],
        }
    }

    pub fn new(before: &[syn::Stmt], after: &[syn::Stmt]) -> Self {
        Self {
            before: Vec::from(before),
            inner: None,
            after: Vec::from(after),
        }
    }

    pub fn push_mut(&mut self, before: &[syn::Stmt], after: &[syn::Stmt]) {
        if let Some(inner) = self.inner.as_mut() {
            inner.push_mut(before, after);
        } else {
            self.inner = Some(Box::new(Scope::new(before, after)));
        }
    }

    pub fn quote_with(&self, stmts: &[syn::Stmt]) -> TokenStream2 {
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
