use proc_macro2::Span;
use syn::parse::{self, Parse, ParseStream};

mod kw {
    syn::custom_keyword!(when);
    syn::custom_keyword!(then);
    syn::custom_keyword!(given);
    syn::custom_keyword!(case);
    syn::custom_keyword!(section);
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SectionKeyword {
    When,
    Then,
    Given,
    Case,
    Section,
}

impl SectionKeyword {
    pub fn to_name(&self) -> String {
        match self {
            Self::When => "when".to_string(),
            Self::Then => "then".to_string(),
            Self::Given => "given".to_string(),
            Self::Case => "case".to_string(),
            Self::Section => "section".to_string(),
        }
    }
}

impl SectionKeyword {
    pub fn peek(i: ParseStream) -> bool {
        let lk = i.lookahead1();

        let mut test = false;

        test |= lk.peek(kw::when);
        test |= lk.peek(kw::given);
        test |= lk.peek(kw::section);
        test |= lk.peek(kw::then);
        test |= lk.peek(kw::case);

        test
    }
}

impl Parse for SectionKeyword {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let lk = input.lookahead1();

        if lk.peek(kw::when) {
            input.parse::<kw::when>()?;

            Ok(Self::When)
        } else if lk.peek(kw::then) {
            input.parse::<kw::then>()?;

            Ok(Self::Then)
        } else if lk.peek(kw::given) {
            input.parse::<kw::given>()?;

            Ok(Self::Given)
        } else if lk.peek(kw::case) {
            input.parse::<kw::case>()?;

            Ok(Self::Case)
        } else if lk.peek(kw::section) {
            input.parse::<kw::section>()?;

            Ok(Self::Section)
        } else {
            Err(parse::Error::new(
                Span::call_site(),
                "Invalid section keyword",
            ))
        }
    }
}
