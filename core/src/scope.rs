use quote::quote;
use syn::export::TokenStream2;

#[derive(Clone)]
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

    pub fn push(&self, before: &[syn::Stmt], after: &[syn::Stmt]) -> Self {
        let mut new_scope = self.clone();

        if let Some(inner) = new_scope.inner.as_mut() {
            new_scope = inner.push(before, after);
        } else {
            new_scope.inner = Some(Box::new(Scope::new(before, after)));
        }

        new_scope
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
