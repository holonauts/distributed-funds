import { CallableCell } from '@holochain/tryorama';
import { NewEntryAction, ActionHash, Record, AppBundleSource, fakeActionHash, fakeAgentPubKey, fakeEntryHash, fakeDnaHash } from '@holochain/client';



export async function sampleApplicationTemplate(cell: CallableCell, partialApplicationTemplate = {}) {
    return {
        ...{
	  form_schema: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
        },
        ...partialApplicationTemplate
    };
}

export async function createApplicationTemplate(cell: CallableCell, applicationTemplate = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "grants",
      fn_name: "create_application_template",
      payload: applicationTemplate || await sampleApplicationTemplate(cell),
    });
}



export async function sampleEvaluationTemplate(cell: CallableCell, partialEvaluationTemplate = {}) {
    return {
        ...{
	  form_schema: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  score: { type: 'Single' },
        },
        ...partialEvaluationTemplate
    };
}

export async function createEvaluationTemplate(cell: CallableCell, evaluationTemplate = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "grants",
      fn_name: "create_evaluation_template",
      payload: evaluationTemplate || await sampleEvaluationTemplate(cell),
    });
}



export async function sampleTimePeriod(cell: CallableCell, partialTimePeriod = {}) {
    return {
        ...{
	  start_at: 1674053334548000,
	  end_at: 1674053334548000,
        },
        ...partialTimePeriod
    };
}

export async function createTimePeriod(cell: CallableCell, timePeriod = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "grants",
      fn_name: "create_time_period",
      payload: timePeriod || await sampleTimePeriod(cell),
    });
}



export async function sampleGrantPool(cell: CallableCell, partialGrantPool = {}) {
    return {
        ...{
	  name: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  purpose_description: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  rules_description: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
          time_period: (await createTimePeriod(cell)).signed_action.hashed.hash,
          application_template: (await createApplicationTemplate(cell)).signed_action.hashed.hash,
          evaluation_template: (await createEvaluationTemplate(cell)).signed_action.hashed.hash,
        },
        ...partialGrantPool
    };
}

export async function createGrantPool(cell: CallableCell, grantPool = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "grants",
      fn_name: "create_grant_pool",
      payload: grantPool || await sampleGrantPool(cell),
    });
}



export async function sampleApplication(cell: CallableCell, partialApplication = {}) {
    return {
        ...{
	  application_template: (await fakeActionHash()),
	  json_data: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  status: { type: 'Draft' },
        },
        ...partialApplication
    };
}

export async function createApplication(cell: CallableCell, application = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "grants",
      fn_name: "create_application",
      payload: application || await sampleApplication(cell),
    });
}



export async function sampleEvaluation(cell: CallableCell, partialEvaluation = {}) {
    return {
        ...{
          application: (await createApplication(cell)).signed_action.hashed.hash,
	  json_data: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  comments: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  score: { type: 'Single' },
        },
        ...partialEvaluation
    };
}

export async function createEvaluation(cell: CallableCell, evaluation = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "grants",
      fn_name: "create_evaluation",
      payload: evaluation || await sampleEvaluation(cell),
    });
}



export async function sampleGrantPoolOutcome(cell: CallableCell, partialGrantPoolOutcome = {}) {
    return {
        ...{
          grant_pool: (await createGrantPool(cell)).signed_action.hashed.hash,
	  outcomes: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  coupon: [10],
        },
        ...partialGrantPoolOutcome
    };
}

export async function createGrantPoolOutcome(cell: CallableCell, grantPoolOutcome = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "grants",
      fn_name: "create_grant_pool_outcome",
      payload: grantPoolOutcome || await sampleGrantPoolOutcome(cell),
    });
}

