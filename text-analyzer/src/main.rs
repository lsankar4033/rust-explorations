// Text Analyzer Project
//
// Goal: Parse a text file and output statistics
//
// Core Features:
// - Total word count
// - Unique word count
// - Top 10 most frequent words
// - Average word length
// - Longest word
//
// Bonus Features:
// - Filter out common words ("the", "a", etc.)
// - Case-insensitive counting
// - Handle punctuation properly

use std::fs;

fn main() {
    let file_path = "text-analyzer/sample.txt";

    let content: String = fs::read_to_string(file_path).expect("Failed to read file");
    let text_stats = analyze_text(&content);

    println!("total words: {}", text_stats.total_words);
    println!("unique words: {}", text_stats.unique_words);
    println!("top words: {:?}", text_stats.top_words);
    println!("average word length: {}", text_stats.avg_word_length);
    println!("longest word: {}", text_stats.longest_word);
}

struct TextStats {
    total_words: usize,
    unique_words: usize,
    top_words: Vec<String>,
    avg_word_length: usize,
    longest_word: String,
}

fn analyze_text(content: &str) -> TextStats {
    let words: Vec<&str> = content.split_whitespace().collect();

    let total_words = words.len();
    let unique_words = words.iter().collect::<std::collections::HashSet<_>>().len();
    let top_words = top_words(&words, 10);
    let avg_word_length = words.iter().map(|word| word.len()).sum::<usize>() / total_words;
    let longest_word = words
        .iter()
        .max_by_key(|word| word.len())
        .unwrap()
        .to_string();

    TextStats {
        total_words,
        unique_words,
        top_words,
        avg_word_length,
        longest_word,
    }
}

fn top_words(words: &[&str], n: usize) -> Vec<String> {
    let mut map = std::collections::HashMap::new();
    for word in words {
        *map.entry(word).or_insert(0) += 1;
    }
    let mut top_words = map.into_iter().collect::<Vec<_>>();
    top_words.sort_by(|a, b| b.1.cmp(&a.1));
    top_words
        .into_iter()
        .map(|(word, _)| word.to_string())
        .take(n)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_words() {
        let text = "a";
        let stats = analyze_text(text);
        assert_eq!(stats.total_words, 1);

        let text = "a b";
        let stats = analyze_text(text);
        assert_eq!(stats.total_words, 2);
    }

    #[test]
    fn test_unique_words() {
        let text = "a a";
        let stats = analyze_text(text);
        assert_eq!(stats.unique_words, 1);

        let text = "a b";
        let stats = analyze_text(text);
        assert_eq!(stats.unique_words, 2);
    }

    #[test]
    fn test_top_words() {
        let words = vec!["a", "b", "c", "a", "b", "a"];
        let top_2_words = top_words(&words, 2);
        assert_eq!(top_2_words, vec!["a".to_string(), "b".to_string()]);

        let top_3_words = top_words(&words, 3);
        assert_eq!(
            top_3_words,
            vec!["a".to_string(), "b".to_string(), "c".to_string()]
        );
    }

    #[test]
    fn test_avg_word_length() {
        let content = "a bb ccc";
        let stats = analyze_text(content);
        assert_eq!(stats.avg_word_length, 2);
    }

    #[test]
    fn test_longest_word() {
        let content = "a bb ccc";
        let stats = analyze_text(content);
        assert_eq!(stats.longest_word, "ccc");
    }
}
