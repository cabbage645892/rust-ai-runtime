use rust_ai_runtime::tokenizer::{Tokenizer, WordTokenizer};

#[test]
fn encode_decode_roundtrip() {
    let mut tokenizer = WordTokenizer::new();

    let tokens = tokenizer.encode("Rust is safe");

    assert_eq!(tokens, vec![1, 2, 3]);
    assert_eq!(tokenizer.decode(&tokens), "Rust is safe");
}

#[test]
fn repeated_words_share_same_token_id() {
    let mut tokenizer = WordTokenizer::new();

    let tokens = tokenizer.encode("Rust Rust Rust");

    assert_eq!(tokens, vec![1, 1, 1]);
    assert_eq!(tokenizer.vocab_size(), 1);
}
