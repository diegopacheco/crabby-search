pub fn tokenize(text: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    for c in text.chars() {
        if c.is_alphanumeric() {
            for lower in c.to_lowercase() {
                current.push(lower);
            }
        } else if !current.is_empty() {
            tokens.push(std::mem::take(&mut current));
        }
    }
    if !current.is_empty() {
        tokens.push(current);
    }
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_and_lowercases() {
        assert_eq!(tokenize("Hello, World!"), vec!["hello", "world"]);
    }

    #[test]
    fn keeps_alphanumeric_runs() {
        assert_eq!(tokenize("rust2021 is fast"), vec!["rust2021", "is", "fast"]);
    }

    #[test]
    fn empty_text_yields_no_tokens() {
        assert!(tokenize("   ,. ").is_empty());
    }
}
