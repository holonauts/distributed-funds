<script lang="ts">
	import { createEventDispatcher, onMount, getContext } from 'svelte';
	import '@material/mwc-circular-progress';
	import { decode } from '@msgpack/msgpack';
	import type {
		Record,
		ActionHash,
		AppAgentClient,
		EntryHash,
		AgentPubKey,
		DnaHash
	} from '@holochain/client';
	import { clientContext } from '../../contexts';
	import type { Evaluation, QuantitativeRating } from './types';
	import '@material/mwc-circular-progress';
	import type { Snackbar } from '@material/mwc-snackbar';
	import '@material/mwc-snackbar';
	import '@material/mwc-icon-button';

	const dispatch = createEventDispatcher();

	export let evaluationHash: ActionHash;

	let client: AppAgentClient = (getContext(clientContext) as any).getClient();

	let loading = true;
	let error: any = undefined;

	let record: Record | undefined;
	let evaluation: Evaluation | undefined;

	$: error, loading, record, evaluation;

	onMount(async () => {
		if (evaluationHash === undefined) {
			throw new Error(`The evaluationHash input is required for the EvaluationDetail element`);
		}
		await fetchEvaluation();
	});

	async function fetchEvaluation() {
		loading = true;
		error = undefined;
		record = undefined;
		evaluation = undefined;

		try {
			record = await client.callZome({
				cap_secret: null,
				role_name: 'grant_pools',
				zome_name: 'grants',
				fn_name: 'get_evaluation',
				payload: evaluationHash
			});
			if (record) {
				evaluation = decode((record.entry as any).Present.entry) as Evaluation;
			}
		} catch (e) {
			error = e;
		}

		loading = false;
	}
</script>

{#if loading}
	<div style="display: flex; flex: 1; align-items: center; justify-content: center">
		<mwc-circular-progress indeterminate></mwc-circular-progress>
	</div>
{:else if error}
	<span>Error fetching the evaluation: {error.data.data}</span>
{:else}
	<div style="display: flex; flex-direction: column">
		<div style="display: flex; flex-direction: row">
			<span style="flex: 1"></span>
		</div>

		<div style="display: flex; flex-direction: row; margin-bottom: 16px">
			<span style="margin-right: 4px"><strong>Comments:</strong></span>
			<span style="white-space: pre-line">{evaluation.comments}</span>
		</div>
	</div>
{/if}
