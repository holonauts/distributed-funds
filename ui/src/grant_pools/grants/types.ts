import type {
	Record,
	ActionHash,
	DnaHash,
	SignedActionHashed,
	EntryHash,
	AgentPubKey,
	Create,
	Update,
	Delete,
	CreateLink,
	DeleteLink
} from '@holochain/client';

export type GrantsSignal =
	| {
			type: 'EntryCreated';
			action: SignedActionHashed<Create>;
			app_entry: EntryTypes;
	  }
	| {
			type: 'EntryUpdated';
			action: SignedActionHashed<Update>;
			app_entry: EntryTypes;
			original_app_entry: EntryTypes;
	  }
	| {
			type: 'EntryDeleted';
			action: SignedActionHashed<Delete>;
			original_app_entry: EntryTypes;
	  }
	| {
			type: 'LinkCreated';
			action: SignedActionHashed<CreateLink>;
			link_type: string;
	  }
	| {
			type: 'LinkDeleted';
			action: SignedActionHashed<DeleteLink>;
			link_type: string;
	  };

export type EntryTypes =
	| ({ type: 'GrantPoolOutcome' } & GrantPoolOutcome)
	| ({ type: 'Evaluation' } & Evaluation)
	| ({ type: 'Application' } & Application)
	| ({ type: 'GrantPool' } & GrantPool)
	| ({ type: 'TimePeriod' } & TimePeriod)
	| ({ type: 'EvaluationTemplate' } & EvaluationTemplate)
	| ({ type: 'ApplicationTemplate' } & ApplicationTemplate);

export interface ApplicationTemplate {
	form_schema: string;
	name: string;
}

export interface AmountRange {
	min: Uint8Array;
	max: Uint8Array;
};

export interface AmountRangeBigInt {
	min: bigint;
	max: bigint;
};


export interface NumberRange {
	min: number;
	max: number;
}

export interface AttributeScoreTemplate {
	label: string,
	weight: number,
}

export enum ScoreType {
	Single = 'Single',
	Weighted = 'Weighted'
}

export interface ScoreTemplate {
	type: ScoreType;
	content: undefined | AttributeScoreTemplate[];
}

export interface EvaluationTemplate {
	name: string;
	
	form_schema: string;

	score_range: NumberRange,
	
	score: ScoreTemplate;
}

export interface TimePeriod {
	start_at: number;

	end_at: number;
}

export interface FlowCloneEvm {
    flow_clone_address: string,
    deposit_expression_address: string,
    close_expression_address: string,
    claim_expression_address: string,
};

export interface GrantPool {
	name: string;

	purpose_description: string;

	rules_description: string;

	time_period: ActionHash;

	application_template: ActionHash;

	evaluation_template: ActionHash;

	amount_range: AmountRange;

	evaluators: AgentPubKey[];
	
	notary_evm_wallet: string;

	flow_evm: FlowCloneEvm;
}

export enum StatusType {
	Draft = 'Draft',
	Submitted = 'Submitted',
	Claimed = 'Claimed',
}

export interface ApplicationOutcome {
	approved: boolean;
	grant_pool: ActionHash;
}

export interface Status {
	type: StatusType;
	content: ActionHash | Uint8Array;
}

export interface Application {
	grant_pool: ActionHash;

	form_content: string;

	amount: Uint8Array;

	status: Status;

	evm_wallet: `0x${string}`;
}

export interface AttributeScore {
    label: string,
    value: number,
		weight: number
}

export interface Score {
	type: ScoreType
	content: number | AttributeScore[]
}

export interface Evaluation {
	application: ActionHash;

	form_content: string;

	comments: string;

	score: Score;
}

export interface GrantPoolOutcome {
	grant_pool: ActionHash;

	outcomes: string;

	coupon: Array<number>;
}
