use catchr_core::{Section, SectionBody, SectionItem, SectionKeyword};

fn syn_parse<T: syn::parse::Parse>(s: impl AsRef<str>) -> T {
    syn::parse_str(s.as_ref()).unwrap()
}

#[test]
fn empty_when_section() {
    let raw = r#"
        when "Hello!" {

        }
    "#;

    let section = syn::parse_str::<Section>(raw).unwrap();

    assert_eq!(
        section,
        Section::new(
            SectionKeyword::When,
            "Hello!".to_string(),
            SectionBody::empty(),
        )
    );
}

#[test]
fn nested_one() {
    let raw = r#"
        when "Hello!" {
            then "Whatever" {
                assert!(false);
            }
        }
    "#;

    let section = syn::parse_str::<Section>(raw).unwrap();

    assert_eq!(
        section,
        Section::new(
            SectionKeyword::When,
            "Hello!".to_string(),
            SectionBody::new(vec![SectionItem::Sep(Section::new(
                SectionKeyword::Then,
                "Whatever".to_string(),
                SectionBody::new(vec![SectionItem::Stmt(syn_parse(
                    "assert!(false);"
                ))]),
            ))]),
        )
    );
}

// TODO: More tests!
