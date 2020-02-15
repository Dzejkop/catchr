use itertools::Itertools;

pub fn escape_name(input: impl AsRef<str>) -> String {
    if input.as_ref().is_empty() {
        return "empty".to_string();
    }

    let s: String = input
        .as_ref()
        .to_ascii_lowercase()
        .chars()
        .map(|c| match c {
            c if c.is_ascii_alphabetic() || c.is_numeric() => c,
            _ => '_',
        })
        .dedup_by(|a, b| *a == '_' && a == b)
        .collect();

    s.trim_end_matches('_').to_string()
}

pub fn extract_literal_string(lit: syn::Lit) -> Option<String> {
    match lit {
        syn::Lit::Str(s) => Some(s.value()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escapes_strings() {
        assert_eq!("hello_world", &escape_name("Hello World!"));
        assert_eq!(
            "my_struct_foo_1_should_equal_2",
            &escape_name("my_struct.foo(1) should equal 2")
        );
        assert_eq!(
            "here_we_go",
            &escape_name("Here!@#%$#^@#We!$!#%$^&^*Go!!!!!")
        );
        assert_eq!("empty", &escape_name(""))
    }
}
