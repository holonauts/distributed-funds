pub mod grant_pool_to_applications;
pub use grant_pool_to_applications::*;
pub mod sponsor_to_grant_pools;
pub use sponsor_to_grant_pools::*;
pub mod agent_to_evm_wallet;
pub use agent_to_evm_wallet::*;
pub mod grant_pool_outcome;
pub use grant_pool_outcome::*;
pub mod evaluation;
pub use evaluation::*;
pub mod application;
pub use application::*;
pub mod grant_pool;
pub use grant_pool::*;
pub mod time_period;
pub use time_period::*;
pub mod evaluation_template;
pub use evaluation_template::*;
pub mod application_template;
pub use application_template::*;
pub mod evaluator_to_applications;
pub use evaluator_to_applications::*;
use hdi::prelude::*;
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    ApplicationTemplate(ApplicationTemplate),
    EvaluationTemplate(EvaluationTemplate),
    TimePeriod(TimePeriod),
    GrantPool(GrantPool),
    Application(Application),
    Evaluation(Evaluation),
    GrantPoolOutcome(GrantPoolOutcome),
}
#[derive(Serialize, Deserialize)]
#[hdk_link_types]
pub enum LinkTypes {
    AllApplicationTemplates,
    AllEvaluationTemplates,
    AllTimePeriods,
    TimePeriodToGrantPools,
    ApplicationTemplateToGrantPools,
    EvaluationTemplateToGrantPools,
    AllGrantPools,
    ApplicationUpdates,
    AllApplications,
    MyApplications,
    ApplicationToEvaluation,
    GrantPoolToGrantPoolOutcomes,
    AgentToEvmWallet,
    SponsorToGrantPool,
    GrantPoolToSponsor,
    GrantPoolToApplication,
    EvaluatorToApplications,
    ApplicationToEvaluators,
}

#[hdk_extern]
pub fn genesis_self_check(_data: GenesisSelfCheckData) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_agent_joining(
    _agent_pub_key: AgentPubKey,
    _membrane_proof: &Option<MembraneProof>,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
#[hdk_extern]
pub fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
    match op.flattened::<EntryTypes, LinkTypes>()? {
        FlatOp::StoreEntry(store_entry) => match store_entry {
            OpEntry::CreateEntry { app_entry, action } => match app_entry {
                EntryTypes::ApplicationTemplate(application_template) => {
                    validate_create_application_template(
                        EntryCreationAction::Create(action),
                        application_template,
                    )
                }
                EntryTypes::EvaluationTemplate(evaluation_template) => {
                    validate_create_evaluation_template(
                        EntryCreationAction::Create(action),
                        evaluation_template,
                    )
                }
                EntryTypes::TimePeriod(time_period) => {
                    validate_create_time_period(EntryCreationAction::Create(action), time_period)
                }
                EntryTypes::GrantPool(grant_pool) => {
                    validate_create_grant_pool(EntryCreationAction::Create(action), grant_pool)
                }
                EntryTypes::Application(application) => {
                    validate_create_application(EntryCreationAction::Create(action), application)
                }
                EntryTypes::Evaluation(evaluation) => {
                    validate_create_evaluation(EntryCreationAction::Create(action), evaluation)
                }
                EntryTypes::GrantPoolOutcome(grant_pool_outcome) => {
                    validate_create_grant_pool_outcome(
                        EntryCreationAction::Create(action),
                        grant_pool_outcome,
                    )
                }
            },
            OpEntry::UpdateEntry {
                app_entry, action, ..
            } => match app_entry {
                EntryTypes::ApplicationTemplate(application_template) => {
                    validate_create_application_template(
                        EntryCreationAction::Update(action),
                        application_template,
                    )
                }
                EntryTypes::EvaluationTemplate(evaluation_template) => {
                    validate_create_evaluation_template(
                        EntryCreationAction::Update(action),
                        evaluation_template,
                    )
                }
                EntryTypes::TimePeriod(time_period) => {
                    validate_create_time_period(EntryCreationAction::Update(action), time_period)
                }
                EntryTypes::GrantPool(grant_pool) => {
                    validate_create_grant_pool(EntryCreationAction::Update(action), grant_pool)
                }
                EntryTypes::Application(application) => {
                    validate_create_application(EntryCreationAction::Update(action), application)
                }
                EntryTypes::Evaluation(evaluation) => {
                    validate_create_evaluation(EntryCreationAction::Update(action), evaluation)
                }
                EntryTypes::GrantPoolOutcome(grant_pool_outcome) => {
                    validate_create_grant_pool_outcome(
                        EntryCreationAction::Update(action),
                        grant_pool_outcome,
                    )
                }
            },
            _ => Ok(ValidateCallbackResult::Valid),
        },
        FlatOp::RegisterUpdate(update_entry) => match update_entry {
            OpUpdate::Entry {
                original_action,
                original_app_entry,
                app_entry,
                action,
            } => match (app_entry, original_app_entry) {
                (
                    EntryTypes::GrantPoolOutcome(grant_pool_outcome),
                    EntryTypes::GrantPoolOutcome(original_grant_pool_outcome),
                ) => validate_update_grant_pool_outcome(
                    action,
                    grant_pool_outcome,
                    original_action,
                    original_grant_pool_outcome,
                ),
                (
                    EntryTypes::Evaluation(evaluation),
                    EntryTypes::Evaluation(original_evaluation),
                ) => validate_update_evaluation(
                    action,
                    evaluation,
                    original_action,
                    original_evaluation,
                ),
                (
                    EntryTypes::Application(application),
                    EntryTypes::Application(original_application),
                ) => validate_update_application(
                    action,
                    application,
                    original_action,
                    original_application,
                ),
                (EntryTypes::GrantPool(grant_pool), EntryTypes::GrantPool(original_grant_pool)) => {
                    validate_update_grant_pool(
                        action,
                        grant_pool,
                        original_action,
                        original_grant_pool,
                    )
                }
                (
                    EntryTypes::TimePeriod(time_period),
                    EntryTypes::TimePeriod(original_time_period),
                ) => validate_update_time_period(
                    action,
                    time_period,
                    original_action,
                    original_time_period,
                ),
                (
                    EntryTypes::EvaluationTemplate(evaluation_template),
                    EntryTypes::EvaluationTemplate(original_evaluation_template),
                ) => validate_update_evaluation_template(
                    action,
                    evaluation_template,
                    original_action,
                    original_evaluation_template,
                ),
                (
                    EntryTypes::ApplicationTemplate(application_template),
                    EntryTypes::ApplicationTemplate(original_application_template),
                ) => validate_update_application_template(
                    action,
                    application_template,
                    original_action,
                    original_application_template,
                ),
                _ => Ok(ValidateCallbackResult::Invalid(
                    "Original and updated entry types must be the same".to_string(),
                )),
            },
            _ => Ok(ValidateCallbackResult::Valid),
        },
        FlatOp::RegisterDelete(delete_entry) => match delete_entry {
            OpDelete::Entry {
                original_action,
                original_app_entry,
                action,
            } => match original_app_entry {
                EntryTypes::ApplicationTemplate(application_template) => {
                    validate_delete_application_template(
                        action,
                        original_action,
                        application_template,
                    )
                }
                EntryTypes::EvaluationTemplate(evaluation_template) => {
                    validate_delete_evaluation_template(
                        action,
                        original_action,
                        evaluation_template,
                    )
                }
                EntryTypes::TimePeriod(time_period) => {
                    validate_delete_time_period(action, original_action, time_period)
                }
                EntryTypes::GrantPool(grant_pool) => {
                    validate_delete_grant_pool(action, original_action, grant_pool)
                }
                EntryTypes::Application(application) => {
                    validate_delete_application(action, original_action, application)
                }
                EntryTypes::Evaluation(evaluation) => {
                    validate_delete_evaluation(action, original_action, evaluation)
                }
                EntryTypes::GrantPoolOutcome(grant_pool_outcome) => {
                    validate_delete_grant_pool_outcome(action, original_action, grant_pool_outcome)
                }
            },
            _ => Ok(ValidateCallbackResult::Valid),
        },
        FlatOp::RegisterCreateLink {
            link_type,
            base_address,
            target_address,
            tag,
            action,
        } => match link_type {
            LinkTypes::AllApplicationTemplates => validate_create_link_all_application_templates(
                action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::AllEvaluationTemplates => validate_create_link_all_evaluation_templates(
                action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::AllTimePeriods => {
                validate_create_link_all_time_periods(action, base_address, target_address, tag)
            }
            LinkTypes::TimePeriodToGrantPools => validate_create_link_time_period_to_grant_pools(
                action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::ApplicationTemplateToGrantPools => {
                validate_create_link_application_template_to_grant_pools(
                    action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::EvaluationTemplateToGrantPools => {
                validate_create_link_evaluation_template_to_grant_pools(
                    action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::AllGrantPools => {
                validate_create_link_all_grant_pools(action, base_address, target_address, tag)
            }
            LinkTypes::ApplicationUpdates => {
                validate_create_link_application_updates(action, base_address, target_address, tag)
            }
            LinkTypes::AllApplications => {
                validate_create_link_all_applications(action, base_address, target_address, tag)
            }
            LinkTypes::MyApplications => {
                validate_create_link_my_applications(action, base_address, target_address, tag)
            }
            LinkTypes::ApplicationToEvaluation => validate_create_link_application_to_evaluation(
                action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::GrantPoolToGrantPoolOutcomes => {
                validate_create_link_grant_pool_to_grant_pool_outcomes(
                    action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::AgentToEvmWallet => {
                validate_create_link_agent_to_evm_wallet(action, base_address, target_address, tag)
            }
            LinkTypes::SponsorToGrantPool => validate_create_link_sponsor_to_grant_pool(
                action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::GrantPoolToSponsor => validate_create_link_grant_pool_to_sponsor(
                action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::GrantPoolToApplication => validate_create_link_grant_pool_to_application(
                action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::EvaluatorToApplications => validate_create_link_evaluator_to_applications(
                action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::ApplicationToEvaluators => validate_create_link_application_to_evaluators(
                action,
                base_address,
                target_address,
                tag,
            ),
        },
        FlatOp::RegisterDeleteLink {
            link_type,
            base_address,
            target_address,
            tag,
            original_action,
            action,
        } => match link_type {
            LinkTypes::AllApplicationTemplates => validate_delete_link_all_application_templates(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::AllEvaluationTemplates => validate_delete_link_all_evaluation_templates(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::AllTimePeriods => validate_delete_link_all_time_periods(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::TimePeriodToGrantPools => validate_delete_link_time_period_to_grant_pools(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::ApplicationTemplateToGrantPools => {
                validate_delete_link_application_template_to_grant_pools(
                    action,
                    original_action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::EvaluationTemplateToGrantPools => {
                validate_delete_link_evaluation_template_to_grant_pools(
                    action,
                    original_action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::AllGrantPools => validate_delete_link_all_grant_pools(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::ApplicationUpdates => validate_delete_link_application_updates(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::AllApplications => validate_delete_link_all_applications(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::MyApplications => validate_delete_link_my_applications(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::ApplicationToEvaluation => validate_delete_link_application_to_evaluation(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::GrantPoolToGrantPoolOutcomes => {
                validate_delete_link_grant_pool_to_grant_pool_outcomes(
                    action,
                    original_action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::AgentToEvmWallet => validate_delete_link_agent_to_evm_wallet(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::SponsorToGrantPool => validate_delete_link_sponsor_to_grant_pool(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::GrantPoolToSponsor => validate_delete_link_grant_pool_to_sponsor(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::GrantPoolToApplication => validate_delete_link_grant_pool_to_applications(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::EvaluatorToApplications => validate_delete_link_evaluator_to_applications(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::ApplicationToEvaluators => validate_delete_link_application_to_evaluators(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
        },
        FlatOp::StoreRecord(store_record) => match store_record {
            OpRecord::CreateEntry { app_entry, action } => match app_entry {
                EntryTypes::ApplicationTemplate(application_template) => {
                    validate_create_application_template(
                        EntryCreationAction::Create(action),
                        application_template,
                    )
                }
                EntryTypes::EvaluationTemplate(evaluation_template) => {
                    validate_create_evaluation_template(
                        EntryCreationAction::Create(action),
                        evaluation_template,
                    )
                }
                EntryTypes::TimePeriod(time_period) => {
                    validate_create_time_period(EntryCreationAction::Create(action), time_period)
                }
                EntryTypes::GrantPool(grant_pool) => {
                    validate_create_grant_pool(EntryCreationAction::Create(action), grant_pool)
                }
                EntryTypes::Application(application) => {
                    validate_create_application(EntryCreationAction::Create(action), application)
                }
                EntryTypes::Evaluation(evaluation) => {
                    validate_create_evaluation(EntryCreationAction::Create(action), evaluation)
                }
                EntryTypes::GrantPoolOutcome(grant_pool_outcome) => {
                    validate_create_grant_pool_outcome(
                        EntryCreationAction::Create(action),
                        grant_pool_outcome,
                    )
                }
            },
            OpRecord::UpdateEntry {
                original_action_hash,
                app_entry,
                action,
                ..
            } => {
                let original_record = must_get_valid_record(original_action_hash)?;
                let original_action = original_record.action().clone();
                let original_action = match original_action {
                    Action::Create(create) => EntryCreationAction::Create(create),
                    Action::Update(update) => EntryCreationAction::Update(update),
                    _ => {
                        return Ok(ValidateCallbackResult::Invalid(
                            "Original action for an update must be a Create or Update action"
                                .to_string(),
                        ));
                    }
                };
                match app_entry {
                    EntryTypes::ApplicationTemplate(application_template) => {
                        let result = validate_create_application_template(
                            EntryCreationAction::Update(action.clone()),
                            application_template.clone(),
                        )?;
                        if let ValidateCallbackResult::Valid = result {
                            let original_application_template: Option<ApplicationTemplate> =
                                original_record
                                    .entry()
                                    .to_app_option()
                                    .map_err(|e| wasm_error!(e))?;
                            let original_application_template = match original_application_template
                            {
                                Some(application_template) => application_template,
                                None => {
                                    return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                }
                            };
                            validate_update_application_template(
                                action,
                                application_template,
                                original_action,
                                original_application_template,
                            )
                        } else {
                            Ok(result)
                        }
                    }
                    EntryTypes::EvaluationTemplate(evaluation_template) => {
                        let result = validate_create_evaluation_template(
                            EntryCreationAction::Update(action.clone()),
                            evaluation_template.clone(),
                        )?;
                        if let ValidateCallbackResult::Valid = result {
                            let original_evaluation_template: Option<EvaluationTemplate> =
                                original_record
                                    .entry()
                                    .to_app_option()
                                    .map_err(|e| wasm_error!(e))?;
                            let original_evaluation_template = match original_evaluation_template {
                                Some(evaluation_template) => evaluation_template,
                                None => {
                                    return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                }
                            };
                            validate_update_evaluation_template(
                                action,
                                evaluation_template,
                                original_action,
                                original_evaluation_template,
                            )
                        } else {
                            Ok(result)
                        }
                    }
                    EntryTypes::TimePeriod(time_period) => {
                        let result = validate_create_time_period(
                            EntryCreationAction::Update(action.clone()),
                            time_period.clone(),
                        )?;
                        if let ValidateCallbackResult::Valid = result {
                            let original_time_period: Option<TimePeriod> = original_record
                                .entry()
                                .to_app_option()
                                .map_err(|e| wasm_error!(e))?;
                            let original_time_period = match original_time_period {
                                Some(time_period) => time_period,
                                None => {
                                    return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                }
                            };
                            validate_update_time_period(
                                action,
                                time_period,
                                original_action,
                                original_time_period,
                            )
                        } else {
                            Ok(result)
                        }
                    }
                    EntryTypes::GrantPool(grant_pool) => {
                        let result = validate_create_grant_pool(
                            EntryCreationAction::Update(action.clone()),
                            grant_pool.clone(),
                        )?;
                        if let ValidateCallbackResult::Valid = result {
                            let original_grant_pool: Option<GrantPool> = original_record
                                .entry()
                                .to_app_option()
                                .map_err(|e| wasm_error!(e))?;
                            let original_grant_pool = match original_grant_pool {
                                Some(grant_pool) => grant_pool,
                                None => {
                                    return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                }
                            };
                            validate_update_grant_pool(
                                action,
                                grant_pool,
                                original_action,
                                original_grant_pool,
                            )
                        } else {
                            Ok(result)
                        }
                    }
                    EntryTypes::Application(application) => {
                        let result = validate_create_application(
                            EntryCreationAction::Update(action.clone()),
                            application.clone(),
                        )?;
                        if let ValidateCallbackResult::Valid = result {
                            let original_application: Option<Application> = original_record
                                .entry()
                                .to_app_option()
                                .map_err(|e| wasm_error!(e))?;
                            let original_application = match original_application {
                                Some(application) => application,
                                None => {
                                    return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                }
                            };
                            validate_update_application(
                                action,
                                application,
                                original_action,
                                original_application,
                            )
                        } else {
                            Ok(result)
                        }
                    }
                    EntryTypes::Evaluation(evaluation) => {
                        let result = validate_create_evaluation(
                            EntryCreationAction::Update(action.clone()),
                            evaluation.clone(),
                        )?;
                        if let ValidateCallbackResult::Valid = result {
                            let original_evaluation: Option<Evaluation> = original_record
                                .entry()
                                .to_app_option()
                                .map_err(|e| wasm_error!(e))?;
                            let original_evaluation = match original_evaluation {
                                Some(evaluation) => evaluation,
                                None => {
                                    return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                }
                            };
                            validate_update_evaluation(
                                action,
                                evaluation,
                                original_action,
                                original_evaluation,
                            )
                        } else {
                            Ok(result)
                        }
                    }
                    EntryTypes::GrantPoolOutcome(grant_pool_outcome) => {
                        let result = validate_create_grant_pool_outcome(
                            EntryCreationAction::Update(action.clone()),
                            grant_pool_outcome.clone(),
                        )?;
                        if let ValidateCallbackResult::Valid = result {
                            let original_grant_pool_outcome: Option<GrantPoolOutcome> =
                                original_record
                                    .entry()
                                    .to_app_option()
                                    .map_err(|e| wasm_error!(e))?;
                            let original_grant_pool_outcome = match original_grant_pool_outcome {
                                Some(grant_pool_outcome) => grant_pool_outcome,
                                None => {
                                    return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                }
                            };
                            validate_update_grant_pool_outcome(
                                action,
                                grant_pool_outcome,
                                original_action,
                                original_grant_pool_outcome,
                            )
                        } else {
                            Ok(result)
                        }
                    }
                }
            }
            OpRecord::DeleteEntry {
                original_action_hash,
                action,
                ..
            } => {
                let original_record = must_get_valid_record(original_action_hash)?;
                let original_action = original_record.action().clone();
                let original_action = match original_action {
                    Action::Create(create) => EntryCreationAction::Create(create),
                    Action::Update(update) => EntryCreationAction::Update(update),
                    _ => {
                        return Ok(ValidateCallbackResult::Invalid(
                            "Original action for a delete must be a Create or Update action"
                                .to_string(),
                        ));
                    }
                };
                let app_entry_type = match original_action.entry_type() {
                    EntryType::App(app_entry_type) => app_entry_type,
                    _ => {
                        return Ok(ValidateCallbackResult::Valid);
                    }
                };
                let entry = match original_record.entry().as_option() {
                    Some(entry) => entry,
                    None => {
                        if original_action.entry_type().visibility().is_public() {
                            return Ok(
                                    ValidateCallbackResult::Invalid(
                                        "Original record for a delete of a public entry must contain an entry"
                                            .to_string(),
                                    ),
                                );
                        } else {
                            return Ok(ValidateCallbackResult::Valid);
                        }
                    }
                };
                let original_app_entry = match EntryTypes::deserialize_from_type(
                    app_entry_type.zome_index,
                    app_entry_type.entry_index,
                    entry,
                )? {
                    Some(app_entry) => app_entry,
                    None => {
                        return Ok(
                                ValidateCallbackResult::Invalid(
                                    "Original app entry must be one of the defined entry types for this zome"
                                        .to_string(),
                                ),
                            );
                    }
                };
                match original_app_entry {
                    EntryTypes::ApplicationTemplate(original_application_template) => {
                        validate_delete_application_template(
                            action,
                            original_action,
                            original_application_template,
                        )
                    }
                    EntryTypes::EvaluationTemplate(original_evaluation_template) => {
                        validate_delete_evaluation_template(
                            action,
                            original_action,
                            original_evaluation_template,
                        )
                    }
                    EntryTypes::TimePeriod(original_time_period) => {
                        validate_delete_time_period(action, original_action, original_time_period)
                    }
                    EntryTypes::GrantPool(original_grant_pool) => {
                        validate_delete_grant_pool(action, original_action, original_grant_pool)
                    }
                    EntryTypes::Application(original_application) => {
                        validate_delete_application(action, original_action, original_application)
                    }
                    EntryTypes::Evaluation(original_evaluation) => {
                        validate_delete_evaluation(action, original_action, original_evaluation)
                    }
                    EntryTypes::GrantPoolOutcome(original_grant_pool_outcome) => {
                        validate_delete_grant_pool_outcome(
                            action,
                            original_action,
                            original_grant_pool_outcome,
                        )
                    }
                }
            }
            OpRecord::CreateLink {
                base_address,
                target_address,
                tag,
                link_type,
                action,
            } => match link_type {
                LinkTypes::AllApplicationTemplates => {
                    validate_create_link_all_application_templates(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::AllEvaluationTemplates => validate_create_link_all_evaluation_templates(
                    action,
                    base_address,
                    target_address,
                    tag,
                ),
                LinkTypes::AllTimePeriods => {
                    validate_create_link_all_time_periods(action, base_address, target_address, tag)
                }
                LinkTypes::TimePeriodToGrantPools => {
                    validate_create_link_time_period_to_grant_pools(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::ApplicationTemplateToGrantPools => {
                    validate_create_link_application_template_to_grant_pools(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::EvaluationTemplateToGrantPools => {
                    validate_create_link_evaluation_template_to_grant_pools(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::AllGrantPools => {
                    validate_create_link_all_grant_pools(action, base_address, target_address, tag)
                }
                LinkTypes::ApplicationUpdates => validate_create_link_application_updates(
                    action,
                    base_address,
                    target_address,
                    tag,
                ),
                LinkTypes::AllApplications => {
                    validate_create_link_all_applications(action, base_address, target_address, tag)
                }
                LinkTypes::MyApplications => {
                    validate_create_link_my_applications(action, base_address, target_address, tag)
                }
                LinkTypes::ApplicationToEvaluation => {
                    validate_create_link_application_to_evaluation(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::GrantPoolToGrantPoolOutcomes => {
                    validate_create_link_grant_pool_to_grant_pool_outcomes(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::AgentToEvmWallet => validate_create_link_agent_to_evm_wallet(
                    action,
                    base_address,
                    target_address,
                    tag,
                ),
                LinkTypes::SponsorToGrantPool => validate_create_link_sponsor_to_grant_pool(
                    action,
                    base_address,
                    target_address,
                    tag,
                ),
                LinkTypes::GrantPoolToSponsor => validate_create_link_grant_pool_to_sponsor(
                    action,
                    base_address,
                    target_address,
                    tag,
                ),
                LinkTypes::GrantPoolToApplication => {
                    validate_create_link_grant_pool_to_application(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::EvaluatorToApplications => {
                    validate_create_link_evaluator_to_applications(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::ApplicationToEvaluators => {
                    validate_create_link_application_to_evaluators(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
            },
            OpRecord::DeleteLink {
                original_action_hash,
                base_address,
                action,
            } => {
                let record = must_get_valid_record(original_action_hash)?;
                let create_link = match record.action() {
                    Action::CreateLink(create_link) => create_link.clone(),
                    _ => {
                        return Ok(ValidateCallbackResult::Invalid(
                            "The action that a DeleteLink deletes must be a CreateLink".to_string(),
                        ));
                    }
                };
                let link_type =
                    match LinkTypes::from_type(create_link.zome_index, create_link.link_type)? {
                        Some(lt) => lt,
                        None => {
                            return Ok(ValidateCallbackResult::Valid);
                        }
                    };
                match link_type {
                    LinkTypes::AllApplicationTemplates => {
                        validate_delete_link_all_application_templates(
                            action,
                            create_link.clone(),
                            base_address,
                            create_link.target_address,
                            create_link.tag,
                        )
                    }
                    LinkTypes::AllEvaluationTemplates => {
                        validate_delete_link_all_evaluation_templates(
                            action,
                            create_link.clone(),
                            base_address,
                            create_link.target_address,
                            create_link.tag,
                        )
                    }
                    LinkTypes::AllTimePeriods => validate_delete_link_all_time_periods(
                        action,
                        create_link.clone(),
                        base_address,
                        create_link.target_address,
                        create_link.tag,
                    ),
                    LinkTypes::TimePeriodToGrantPools => {
                        validate_delete_link_time_period_to_grant_pools(
                            action,
                            create_link.clone(),
                            base_address,
                            create_link.target_address,
                            create_link.tag,
                        )
                    }
                    LinkTypes::ApplicationTemplateToGrantPools => {
                        validate_delete_link_application_template_to_grant_pools(
                            action,
                            create_link.clone(),
                            base_address,
                            create_link.target_address,
                            create_link.tag,
                        )
                    }
                    LinkTypes::EvaluationTemplateToGrantPools => {
                        validate_delete_link_evaluation_template_to_grant_pools(
                            action,
                            create_link.clone(),
                            base_address,
                            create_link.target_address,
                            create_link.tag,
                        )
                    }
                    LinkTypes::AllGrantPools => validate_delete_link_all_grant_pools(
                        action,
                        create_link.clone(),
                        base_address,
                        create_link.target_address,
                        create_link.tag,
                    ),
                    LinkTypes::ApplicationUpdates => validate_delete_link_application_updates(
                        action,
                        create_link.clone(),
                        base_address,
                        create_link.target_address,
                        create_link.tag,
                    ),
                    LinkTypes::AllApplications => validate_delete_link_all_applications(
                        action,
                        create_link.clone(),
                        base_address,
                        create_link.target_address,
                        create_link.tag,
                    ),
                    LinkTypes::MyApplications => validate_delete_link_my_applications(
                        action,
                        create_link.clone(),
                        base_address,
                        create_link.target_address,
                        create_link.tag,
                    ),
                    LinkTypes::ApplicationToEvaluation => {
                        validate_delete_link_application_to_evaluation(
                            action,
                            create_link.clone(),
                            base_address,
                            create_link.target_address,
                            create_link.tag,
                        )
                    }
                    LinkTypes::GrantPoolToGrantPoolOutcomes => {
                        validate_delete_link_grant_pool_to_grant_pool_outcomes(
                            action,
                            create_link.clone(),
                            base_address,
                            create_link.target_address,
                            create_link.tag,
                        )
                    }
                    LinkTypes::AgentToEvmWallet => validate_delete_link_agent_to_evm_wallet(
                        action,
                        create_link.clone(),
                        base_address,
                        create_link.target_address,
                        create_link.tag,
                    ),
                    LinkTypes::SponsorToGrantPool => validate_delete_link_sponsor_to_grant_pool(
                        action,
                        create_link.clone(),
                        base_address,
                        create_link.target_address,
                        create_link.tag,
                    ),
                    LinkTypes::GrantPoolToSponsor => validate_delete_link_grant_pool_to_sponsor(
                        action,
                        create_link.clone(),
                        base_address,
                        create_link.target_address,
                        create_link.tag,
                    ),
                    LinkTypes::GrantPoolToApplication => {
                        validate_delete_link_grant_pool_to_applications(
                            action,
                            create_link.clone(),
                            base_address,
                            create_link.target_address,
                            create_link.tag,
                        )
                    }
                    LinkTypes::EvaluatorToApplications => {
                        validate_delete_link_evaluator_to_applications(
                            action,
                            create_link.clone(),
                            base_address,
                            create_link.target_address,
                            create_link.tag,
                        )
                    }
                    LinkTypes::ApplicationToEvaluators => {
                        validate_delete_link_application_to_evaluators(
                            action,
                            create_link.clone(),
                            base_address,
                            create_link.target_address,
                            create_link.tag,
                        )
                    }
                }
            }
            OpRecord::CreatePrivateEntry { .. } => Ok(ValidateCallbackResult::Valid),
            OpRecord::UpdatePrivateEntry { .. } => Ok(ValidateCallbackResult::Valid),
            OpRecord::CreateCapClaim { .. } => Ok(ValidateCallbackResult::Valid),
            OpRecord::CreateCapGrant { .. } => Ok(ValidateCallbackResult::Valid),
            OpRecord::UpdateCapClaim { .. } => Ok(ValidateCallbackResult::Valid),
            OpRecord::UpdateCapGrant { .. } => Ok(ValidateCallbackResult::Valid),
            OpRecord::Dna { .. } => Ok(ValidateCallbackResult::Valid),
            OpRecord::OpenChain { .. } => Ok(ValidateCallbackResult::Valid),
            OpRecord::CloseChain { .. } => Ok(ValidateCallbackResult::Valid),
            OpRecord::InitZomesComplete { .. } => Ok(ValidateCallbackResult::Valid),
            _ => Ok(ValidateCallbackResult::Valid),
        },
        FlatOp::RegisterAgentActivity(agent_activity) => match agent_activity {
            OpActivity::CreateAgent { agent, action } => {
                let previous_action = must_get_action(action.prev_action)?;
                match previous_action.action() {
                        Action::AgentValidationPkg(
                            AgentValidationPkg { membrane_proof, .. },
                        ) => validate_agent_joining(agent, membrane_proof),
                        _ => {
                            Ok(
                                ValidateCallbackResult::Invalid(
                                    "The previous action for a `CreateAgent` action must be an `AgentValidationPkg`"
                                        .to_string(),
                                ),
                            )
                        }
                    }
            }
            _ => Ok(ValidateCallbackResult::Valid),
        },
    }
}
