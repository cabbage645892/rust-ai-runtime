use std::sync::Arc;

use rust_ai_runtime::inference::{
    context::InferenceContext,
    engine::InferenceEngine,
    models::{MarkovModel, RuleBasedModel},
};

#[test]
fn engine_can_switch_current_model() {
    let mut engine = InferenceEngine::new();

    engine.add_model(Arc::new(
        RuleBasedModel::load("assets/models/rules.json").unwrap(),
    ));

    engine.add_model(Arc::new(
        MarkovModel::load("assets/models/markov_corpus.txt").unwrap(),
    ));

    assert_eq!(engine.current_model_name(), "RuleBasedModel");

    engine.set_model_by_name("MarkovModel").unwrap();

    assert_eq!(engine.current_model_name(), "MarkovModel");
}

#[test]
fn engine_infer_uses_current_model() {
    let mut engine = InferenceEngine::new();

    engine.add_model(Arc::new(
        RuleBasedModel::load("assets/models/rules.json").unwrap(),
    ));

    let context = InferenceContext::new("ownership".to_string(), vec![]);

    let result = engine.infer(&context).unwrap();

    assert!(result.response.contains("Ownership"));
}
