mod shared_test;

use grants_integrity::{
    ApplicationTemplate, AttributeScoreTemplate, EvaluationTemplate, NumberRange, ScoreTemplate,
    TimePeriod,
};
use shared_test::*;

use hdk::prelude::*;
use holochain::{
    conductor::api::error::ConductorApiError,
    sweettest::{SweetCell, SweetConductor},
};

use serde_json::json;

#[tokio::test(flavor = "multi_thread")]
pub async fn test_time_period_validation() {
    let (conductor, _agent, cell): (SweetConductor, AgentPubKey, SweetCell) =
        setup_conductor().await;

    let valid_time_period = TimePeriod {
        start_at: Timestamp(1),
        end_at: Timestamp(2),
    };

    let invalid_time_period = TimePeriod {
        start_at: Timestamp(2),
        end_at: Timestamp(1),
    };

    let valid_result: Record = conductor
        .call(
            &cell.zome("grants"),
            "create_time_period",
            valid_time_period,
        )
        .await;

    println!("{:#?}", valid_result);

    let invalid_result: Result<Record, ConductorApiError> = conductor
        .call_fallible(
            &cell.zome("grants"),
            "create_time_period",
            invalid_time_period,
        )
        .await;

    assert!(invalid_result.is_err());
}

#[tokio::test(flavor = "multi_thread")]
pub async fn test_application_template_validation() {
    let (conductor, _agent, cell): (SweetConductor, AgentPubKey, SweetCell) =
        setup_conductor().await;

    let valid_json = json!({
        "schema_title": "Example application template",
        "question_1": "Example question",
        "question_2": "Another example question",
        "sample_int": 11
    })
    .to_string();

    let invalid_json = String::from("invalid_json: $5, , bad example)");

    let valid_template = ApplicationTemplate {
        form_schema: valid_json,
        name: "Example valid".to_string(),
    };

    let invalid_template = ApplicationTemplate {
        form_schema: invalid_json,
        name: "Example invalid".to_string(),
    };

    let valid_result: Record = conductor
        .call(
            &cell.zome("grants"),
            "create_application_template",
            valid_template,
        )
        .await;

    println!("{:#?}", valid_result);

    let invalid_result: Result<Record, ConductorApiError> = conductor
        .call_fallible(
            &cell.zome("grants"),
            "create_application_template",
            invalid_template,
        )
        .await;

    assert!(invalid_result.is_err());
}

#[tokio::test(flavor = "multi_thread")]
pub async fn test_evaluation_template_validation() {
    let (conductor, _agent, cell): (SweetConductor, AgentPubKey, SweetCell) =
        setup_conductor().await;

    let valid_json = json!({
        "schema_title": "Example evaluation template",
        "criteria": "Example criteria",
        "sample_int": 11
    })
    .to_string();

    let invalid_json = String::from("invalid_json: $5, , bad example)");

    let valid_number_range = NumberRange { min: 0, max: 1000 };
    let invalid_number_range = NumberRange { min: 10, max: 1 };

    let valid_template_single = EvaluationTemplate {
        name: "Example valid single".to_string(),
        form_schema: valid_json.clone(),
        score_range: valid_number_range.clone(),
        score: ScoreTemplate::Single,
    };

    let invalid_template_single = EvaluationTemplate {
        name: "Example invalid single".to_string(),
        form_schema: invalid_json.clone(),
        score_range: invalid_number_range.clone(),
        score: ScoreTemplate::Single,
    };

    let valid_single_result: Record = conductor
        .call(
            &cell.zome("grants"),
            "create_evaluation_template",
            valid_template_single,
        )
        .await;

    println!("{:#?}", valid_single_result);

    let invalid_single_result: Result<Record, ConductorApiError> = conductor
        .call_fallible(
            &cell.zome("grants"),
            "create_evaluation_template",
            invalid_template_single,
        )
        .await;

    assert!(invalid_single_result.is_err());

    let attribute_score_template_1 = AttributeScoreTemplate {
        label: "Score template 1".to_string(),
        weight: 1,
    };
    let attribute_score_template_2 = AttributeScoreTemplate {
        label: "Score template 2".to_string(),
        weight: 2,
    };

    let valid_template_weighted = ScoreTemplate::Weighted(vec![
        attribute_score_template_1.clone(),
        attribute_score_template_2,
    ]);

    let invalid_template_weighted = ScoreTemplate::Weighted(vec![attribute_score_template_1]);

    let valid_template_weighted = EvaluationTemplate {
        name: "Example valid weighted".to_string(),
        form_schema: valid_json.clone(),
        score_range: valid_number_range,
        score: valid_template_weighted,
    };

    let invalid_template_weighted = EvaluationTemplate {
        name: "Example invalid weighted".to_string(),
        form_schema: valid_json.clone(),
        score_range: invalid_number_range,
        score: invalid_template_weighted,
    };

    let valid_weighted_result: Record = conductor
        .call(
            &cell.zome("grants"),
            "create_evaluation_template",
            valid_template_weighted,
        )
        .await;

    println!("{:#?}", valid_weighted_result);

    let invalid_weighted_result: Result<Record, ConductorApiError> = conductor
        .call_fallible(
            &cell.zome("grants"),
            "create_evaluation_template",
            invalid_template_weighted,
        )
        .await;

    assert!(invalid_weighted_result.is_err());
}
