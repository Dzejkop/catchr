use crate::Section;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::parse::{self, Parse, ParseStream};
use syn::token::{Brace, Mod};
use syn::{AttrStyle, Attribute, Ident, Item, Visibility};

#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum CatchrModItem {
    Section(Section),
    Item(Item),
}

impl ToTokens for CatchrModItem {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            CatchrModItem::Item(item) => item.to_tokens(tokens),
            CatchrModItem::Section(section) => section.to_tokens(tokens),
        }
    }
}

impl Parse for CatchrModItem {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let result = if Section::peek(input) {
            CatchrModItem::Section(input.parse::<Section>()?)
        } else {
            CatchrModItem::Item(input.parse::<Item>()?)
        };

        Ok(result)
    }
}

#[derive(Debug, Clone)]
pub struct CatchrMod {
    attrs: Vec<Attribute>,
    vis: Visibility,
    mod_token: Mod,
    ident: Ident,
    content: (Brace, Vec<CatchrModItem>),
}

impl ToTokens for CatchrMod {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let CatchrMod {
            vis,
            content,
            attrs,
            mod_token,
            ident,
        } = &self;

        let content = &content.1;

        let outer_attrs =
            attrs.iter().filter(|attr| attr.style == AttrStyle::Outer);

        let inner_attrs = attrs.iter().filter(|attr| match attr.style {
            AttrStyle::Inner(_) => true,
            _ => false,
        });

        let q = quote! {
            #(#outer_attrs)*
            #[allow(unused)]
            #vis #mod_token #ident {
                #(#inner_attrs)*
                #(#content)*
            }
        };

        tokens.append_all(q);
    }
}

impl Parse for CatchrMod {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let mut attrs = Attribute::parse_outer(input)?;

        let vis = Visibility::parse(input)?;

        let mod_token = Mod::parse(input)?;

        let ident = Ident::parse(input)?;

        let content;
        let brace = syn::braced!(content in input);

        let inner_attrs = Attribute::parse_inner(&content)?;

        attrs.extend(inner_attrs);

        let mut items = vec![];

        loop {
            if content.is_empty() {
                break;
            }
            let item = content.parse::<CatchrModItem>()?;

            items.push(item);
        }

        Ok(Self {
            attrs,
            vis,
            mod_token,
            ident,
            content: (brace, items),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_quote() {
        let s = r#"
            #[hello]
            mod whatever {
                #![goodbye]
                use super::*;

                when "whatever" {
                    let x = 1;
                    then "hello" {
                        assert_eq!(x, 1);
                    }
                }
            }"#;

        let catchr_mod = syn::parse_str::<CatchrMod>(s).unwrap();

        let act = catchr_mod.to_token_stream();

        let exp = quote!(
            #[hello]
            #[allow(unused)]
            mod whatever {
                #![goodbye]
                use super::*;

                mod when_whatever {
                    use super::*;

                    #[test]
                    fn then_hello() {
                        {
                            let x = 1;
                            {
                                assert_eq!(x, 1);
                            }
                        }
                    }
                }
            }
        );

        assert_eq!(exp.to_string(), act.to_string());
    }
}
