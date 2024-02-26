<script lang="ts">
	import { onMount, getContext } from 'svelte';
	import { Spinner } from 'flowbite-svelte';
	import type {
		EntryHash,
		Record,
		AgentPubKey,
		ActionHash,
		AppAgentClient,
		NewEntryAction
	} from '@holochain/client';
	import { clientContext } from '../../contexts';
	import EvaluationTemplateDetail from './EvaluationTemplateDetail.svelte';
	import type { GrantsSignal } from './types';

	let client: AppAgentClient = (getContext(clientContext) as any).getClient();

	let hashes: Array<ActionHash> | undefined;
	let loading = true;
	let error: any = undefined;

	$: hashes, loading, error;

	onMount(async () => {
		await fetchEvaluationTemplates();
		client.on('signal', (signal) => {
			if (signal.zome_name !== 'grants') return;
			const payload = signal.payload as GrantsSignal;
			if (payload.type !== 'EntryCreated') return;
			if (payload.app_entry.type !== 'EvaluationTemplate') return;
			hashes = [...hashes, payload.action.hashed.hash];
		});
	});

	async function fetchEvaluationTemplates() {
		try {
			const links = await client.callZome({
				cap_secret: null,
				role_name: 'grant_pools',
				zome_name: 'grants',
				fn_name: 'get_all_evaluation_templates',
				payload: null
			});
			hashes = links.map((l) => l.target);
		} catch (e) {
			error = e;
		}
		loading = false;
	}
</script>

{#if loading}
	<div style="display: flex; flex: 1; align-items: center; justify-content: center">
		<Spinner class="h-4 w-4" />
	</div>
{:else if error}
	<span>Error fetching the evaluation templates: {error.data.data}.</span>
{:else if hashes.length === 0}
	<span>No evaluation templates found.</span>
{:else}
	<div style="display: flex; flex-direction: column">
		{#each hashes as hash}
			<div style="margin-bottom: 8px;">
				<EvaluationTemplateDetail
					evaluationTemplateHash={hash}
					on:evaluation-template-deleted={() => fetchEvaluationTemplates()}
				></EvaluationTemplateDetail>
			</div>
		{/each}
	</div>
{/if}
