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

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;
    use syn::parse_quote;

    fn assert_eq_string(exp: impl ToString, act: impl ToString) {
        assert_eq!(exp.to_string(), act.to_string());
    }

    #[test]
    fn quote_empty_scope() {
        let scope = Scope::empty();

        let act = scope.quote_with(&[]);

        assert_eq_string(
            quote!(
                {}
            ),
            act
        );
    }

    #[test]
    fn quote_empty_scope_with_items() {
        let scope = Scope::empty();

        let act = scope.quote_with(&[
            parse_quote!(let x = 1;),
            parse_quote!(assert_eq!(x, 1);)
        ]);

        assert_eq_string(
            quote!(
                {
                    let x = 1;
                    assert_eq!(x, 1);
                }
            ),
            act
        );
    }

    #[test]
    fn quote_non_empty_scope() {
        let scope = Scope::new(
            &[
                parse_quote!(let x = 1;)
            ],
            &[
                parse_quote!(assert_eq!(x, 1);)
            ]
        );

        let act = scope.quote_with(&[]);

        assert_eq_string(
            quote!(
                let x = 1;
                {

                }
                assert_eq!(x, 1);
            ),
            act
        );
    }

    #[test]
    fn quote_non_empty_scope_with_items() {
        let scope = Scope::new(
            &[
                parse_quote!(let x = 1;)
            ],
            &[
                parse_quote!(assert_eq!(x, 1);)
            ]
        );

        let act = scope.quote_with(&[
            parse_quote!(assert!(true);)
        ]);

        assert_eq_string(
            quote!(
                let x = 1;
                {
                    assert!(true);
                }
                assert_eq!(x, 1);
            ),
            act
        );
    }

    #[test]
    fn push_empty_scope() {
        let mut scope = Scope::empty();
        scope.push_mut(&[], &[]);

        let act = scope.quote_with(&[]);

        assert_eq_string(
            quote!(
                {{}}
            ),
            act
        );
    }

    fn push_non_empty_scope() {
        let mut scope = Scope::new(
            &[
                parse_quote!(let x = 1;)
            ],
            &[
                parse_quote!(assert_eq!(x, 1);)
            ]
        );

        scope.push_mut(
            &[
                parse_quote!(assert!(true);)
            ],
            &[
                parse_quote!(assert!(false);)
            ]
        );

        let act = scope.quote_with(&[
            parse_quote!(assert!(true);)
        ]);

        assert_eq_string(
            quote!(
                let x = 1;
                {
                    assert!(true)
                    {
                        assert!(true);
                    }
                    assert!(false)
                }
                assert_eq!(x, 1);
            ),
            act
        );
    }
}
