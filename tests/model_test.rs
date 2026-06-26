use rust_ai_runtime::{
    inference::{
        context::InferenceContext,
        models::{MarkovModel, RuleBasedModel},
        traits::InferenceModel,
    },
    tokenizer::{Tokenizer, WordTokenizer},
};

#[test]
fn rule_based_model_matches_keyword() {
    let model = RuleBasedModel::load("assets/models/rules.json").unwrap();

    let mut tokenizer = WordTokenizer::new();
    let prompt = "hello rust";

    let context = InferenceContext::new(prompt.to_string(), tokenizer.encode(prompt));

    let result = model.infer(&context).unwrap();

    assert!(result.response.to_lowercase().contains("hello") || result.response.contains("Rust"));
}

#[test]
fn rule_based_model_returns_default_for_unknown_input() {
    let model = RuleBasedModel::load("assets/models/rules.json").unwrap();

    let context = InferenceContext::new("zzzzzzzz".to_string(), vec![]);

    let result = model.infer(&context).unwrap();

    assert_eq!(result.response, "Sorry, I don't understand.");
}

#[test]
fn markov_model_loads_corpus() {
    let model = MarkovModel::load("assets/models/markov_corpus.txt").unwrap();

    assert!(model.state_count() > 0);
}
