/// Converts ASCII letters and digits into a lowercase, hyphen-separated slug.
///
/// Runs of all other characters, including non-ASCII characters, separate
/// words. Separators at either end are omitted; input without ASCII letters
/// or digits produces an empty string.
pub fn slugify(input: &str) -> String {
    let mut result = String::new();
    let mut separator_pending = false;

    for ch in input.chars() {
        if ch.is_ascii_alphanumeric() {
            if separator_pending && !result.is_empty() {
                result.push('-');
            }
            result.push(ch.to_ascii_lowercase());
            separator_pending = false;
        } else {
            separator_pending = true;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::slugify;

    /// Empty and separator-only inputs have no slug to return.
    #[test]
    fn empty_and_separator_only() {
        assert_eq!(slugify(""), "");
        assert_eq!(slugify(" ! \t--"), "");
    }

    /// Existing slugs stay intact, including their digits.
    #[test]
    fn already_slugged() {
        assert_eq!(slugify("hello-world-123"), "hello-world-123");
    }

    /// ASCII letters are normalized regardless of their original case.
    #[test]
    fn mixed_case() {
        assert_eq!(slugify("HeLLo WoRLD"), "hello-world");
    }

    /// A run of different separators produces only one hyphen.
    #[test]
    fn punctuation_and_whitespace() {
        assert_eq!(slugify("one, !\t  two"), "one-two");
    }

    /// Separators outside the first and last word do not create hyphens.
    #[test]
    fn leading_and_trailing_separators() {
        assert_eq!(slugify(" --hello world!! "), "hello-world");
    }

    /// Non-ASCII characters separate words rather than being transliterated.
    #[test]
    fn non_ascii_characters() {
        assert_eq!(slugify("café世界TEST"), "caf-test");
        assert_eq!(slugify("世界"), "");
    }
}
