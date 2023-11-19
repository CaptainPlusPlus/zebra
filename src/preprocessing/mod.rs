pub fn tokenize(text: &str) -> Vec<String> {
    text.split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

pub fn normalize(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_simple_sentence() {
        let text = "Zebras are several species of African equids.";
        let tokens = tokenize(text);
        assert_eq!(tokens, vec!["Zebras", "are", "several", "species", "of", "African", "equids."]);
    }

    #[test]
    fn normalize_mixed_case_sentence() {
        let text = "Zebra stripes are Unique to Each Individual.";
        let normalized_text = normalize(text);
        assert_eq!(normalized_text, "zebra stripes are unique to each individual");
    }

    #[test]
    fn normalize_sentence_with_numbers_and_punctuation() {
        let text = "In 2023, the zebra population increased by 5%.";
        let normalized_text = normalize(text);
        assert_eq!(normalized_text, "in 2023 the zebra population increased by 5");
    }

    #[test]
    fn tokenize_sentence_with_punctuation() {
        let text = "Zebras' stripes may serve as camouflage.";
        let tokens = tokenize(text);
        assert_eq!(tokens, vec!["Zebras'", "stripes", "may", "serve", "as", "camouflage."]);
    }

    #[test]
    fn normalize_empty_string() {
        let text = "";
        let normalized_text = normalize(text);
        assert_eq!(normalized_text, "");
    }
}