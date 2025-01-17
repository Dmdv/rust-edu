use std::collections::HashMap;

fn hash_map() {
    let mut letters = HashMap::new();
    for ch in "some_text".chars() {
        let counter: &mut u32 = letters.entry(ch).or_default();
        *counter += 1;
    }
}

fn count_words(test: &str) -> HashMap<&str, u32> {
    let mut counts = HashMap::new();

    for w in test.split_whitespace() {
        let word = w.trim_matches(|c: char| !c.is_alphabetic());

        let count = counts.entry(w).or_default();
        *count += 1;
    }

    counts
}

fn count_words_fold(test: &str) -> HashMap<&str, u32> {
    test.split_whitespace()
        .map(|w| w.trim_matches(|c: char| !c.is_alphabetic()))
        .fold(HashMap::new(), |mut counts, word| {
            *counts.entry(word).or_insert(0) += 1;
            counts
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_words_with_numbers() {
        let input = "word123 another456 word123";
        let result = count_words(input);

        let mut expected = HashMap::new();
        expected.insert("word123", 2);
        expected.insert("another456", 1);

        assert_eq!(result, expected);
    }

    fn test_count_words_with_numbers_fold() {
        let input = "word123 another456 word123";
        let result = count_words_fold(input);

        let mut expected = HashMap::new();
        expected.insert("word123", 2);
        expected.insert("another456", 1);

        assert_eq!(result, expected);
    }
}
