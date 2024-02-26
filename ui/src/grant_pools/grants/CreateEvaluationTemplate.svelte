<script lang="ts">
	import { createEventDispatcher, getContext, onMount } from 'svelte';
	import type {
		AppAgentClient,
		Record,
		EntryHash,
		AgentPubKey,
		ActionHash,
		DnaHash
	} from '@holochain/client';
	import { clientContext } from '../../contexts';
	import type { EvaluationTemplate, QuantitativeRating } from './types';
	import '@material/mwc-button';
	import '@material/mwc-snackbar';
	import type { Snackbar } from '@material/mwc-snackbar';

	let client: AppAgentClient = (getContext(clientContext) as any).getClient();

	const dispatch = createEventDispatcher();

	export let qualitativeJsonSchema!: string;

	export let quantitativeRating!: QuantitativeRating;

	let errorSnackbar: Snackbar;

	$: qualitativeJsonSchema, quantitativeRating;
	$: isEvaluationTemplateValid = true;

	onMount(() => {
		if (qualitativeJsonSchema === undefined) {
			throw new Error(
				`The qualitativeJsonSchema input is required for the CreateEvaluationTemplate element`
			);
		}
		if (quantitativeRating === undefined) {
			throw new Error(
				`The quantitativeRating input is required for the CreateEvaluationTemplate element`
			);
		}
	});

	async function createEvaluationTemplate() {
		const evaluationTemplateEntry: EvaluationTemplate = {
			qualitative_json_schema: qualitativeJsonSchema!,
			quantitative_rating: quantitativeRating!
		};

		try {
			const record: Record = await client.callZome({
				cap_secret: null,
				role_name: 'grant_pools',
				zome_name: 'grants',
				fn_name: 'create_evaluation_template',
				payload: evaluationTemplateEntry
			});
			dispatch('evaluation-template-created', {
				evaluationTemplateHash: record.signed_action.hashed.hash
			});
		} catch (e) {
			errorSnackbar.labelText = `Error creating the evaluation template: ${e.data.data}`;
			errorSnackbar.show();
		}
	}
</script>

<mwc-snackbar bind:this={errorSnackbar} leading> </mwc-snackbar>
<div style="display: flex; flex-direction: column">
	<span style="font-size: 18px">Create EvaluationTemplate</span>

	<mwc-button
		raised
		label="Create EvaluationTemplate"
		disabled={!isEvaluationTemplateValid}
		on:click={() => createEvaluationTemplate()}
	></mwc-button>
</div>
