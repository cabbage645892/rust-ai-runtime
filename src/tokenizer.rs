//! tokenizer.rs
//!
//! Tokenizer 模块
//!
//! 功能：
//! 1. 文本编码（Encode）
//! 2. Token 解码（Decode）
//! 3. 自动维护 Vocabulary

use std::collections::HashMap;

/// Tokenizer Trait
pub trait Tokenizer {
    /// 将文本编码为 Token ID
    fn encode(&mut self, text: &str) -> Vec<u32>;

    /// 将 Token ID 解码为文本
    fn decode(&self, tokens: &[u32]) -> String;

    /// 返回词表大小
    fn vocab_size(&self) -> usize;
}

/// 简单的 Word Tokenizer
///
/// 按空格切分文本
///
/// Rust is safe
///
/// ↓
///
/// Rust
/// is
/// safe
pub struct WordTokenizer {
    /// 单词 -> Token ID
    word_to_id: HashMap<String, u32>,

    /// Token ID -> 单词
    id_to_word: HashMap<u32, String>,

    /// 下一个可分配 Token ID
    next_id: u32,
}

impl WordTokenizer {
    /// 创建 Tokenizer
    pub fn new() -> Self {
        Self {
            word_to_id: HashMap::new(),
            id_to_word: HashMap::new(),
            next_id: 1,
        }
    }

    /// 添加新单词
    fn add_word(&mut self, word: &str) -> u32 {
        let id = self.next_id;

        self.word_to_id.insert(word.to_string(), id);

        self.id_to_word.insert(id, word.to_string());

        self.next_id += 1;

        id
    }

    /// 查看词表（调试用）
    pub fn print_vocab(&self) {
        println!();
        println!("========== Vocabulary ==========");

        let mut items: Vec<_> = self.word_to_id.iter().collect();

        items.sort_by_key(|(_, id)| *id);

        for (word, id) in items {
            println!("{:<15} -> {}", word, id);
        }

        println!("================================");
    }
}

impl Default for WordTokenizer {
    fn default() -> Self {
        Self::new()
    }
}

impl Tokenizer for WordTokenizer {
    fn encode(&mut self, text: &str) -> Vec<u32> {
        let mut tokens = Vec::new();

        for word in text.split_whitespace() {
            let id = if let Some(id) = self.word_to_id.get(word) {
                *id
            } else {
                self.add_word(word)
            };

            tokens.push(id);
        }

        tokens
    }

    fn decode(&self, tokens: &[u32]) -> String {
        let mut words = Vec::new();

        for id in tokens {
            if let Some(word) = self.id_to_word.get(id) {
                words.push(word.clone());
            } else {
                words.push("<UNK>".to_string());
            }
        }

        words.join(" ")
    }

    fn vocab_size(&self) -> usize {
        self.word_to_id.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let mut tokenizer = WordTokenizer::new();

        let tokens = tokenizer.encode("Rust is safe");

        assert_eq!(tokens, vec![1, 2, 3]);
    }

    #[test]
    fn test_decode() {
        let mut tokenizer = WordTokenizer::new();

        let tokens = tokenizer.encode("Rust is safe");

        let text = tokenizer.decode(&tokens);

        assert_eq!(text, "Rust is safe");
    }

    #[test]
    fn test_vocab_size() {
        let mut tokenizer = WordTokenizer::new();

        tokenizer.encode("Rust is safe");

        assert_eq!(tokenizer.vocab_size(), 3);
    }

    #[test]
    fn test_duplicate_word() {
        let mut tokenizer = WordTokenizer::new();

        let tokens = tokenizer.encode("Rust Rust Rust");

        assert_eq!(tokens, vec![1, 1, 1]);

        assert_eq!(tokenizer.vocab_size(), 1);
    }

    #[test]
    fn test_empty() {
        let mut tokenizer = WordTokenizer::new();

        let tokens = tokenizer.encode("");

        assert!(tokens.is_empty());
    }

    #[test]
    fn test_unknown_token() {
        let tokenizer = WordTokenizer::new();

        let text = tokenizer.decode(&[999]);

        assert_eq!(text, "<UNK>");
    }
}
