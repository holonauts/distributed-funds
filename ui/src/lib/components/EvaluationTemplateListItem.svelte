<script lang="ts">
	import { decode } from '@msgpack/msgpack';
	import { type Record, type ActionHash } from '@holochain/client';
	import type { EvaluationTemplate } from '../../grant_pools/grants/types';
	import { holochainClient } from '$lib/stores/holochainClient';
	import BaseEvaluationTemplateListItem from '$lib/components/BaseEvaluationTemplateListItem.svelte';
	import { Card } from 'flowbite-svelte';

	export let evaluationTemplateHash: ActionHash;

	let loading = true;
	let record: Record | undefined;
	let evaluationTemplate: EvaluationTemplate | undefined;

	$: evaluationTemplateHash, fetchEvaluationTemplate();

	async function fetchEvaluationTemplate() {
		loading = true;
		record = undefined;
		evaluationTemplate = undefined;

		try {
			record = await $holochainClient.client.callZome({
				cap_secret: null,
				role_name: 'grant_pools',
				zome_name: 'grants',
				fn_name: 'get_evaluation_template',
				payload: evaluationTemplateHash
			});
			if (record) {
				evaluationTemplate = decode((record.entry as any).Present.entry) as EvaluationTemplate;
			}
		} catch (e) {
			console.error(e);
		}

		loading = false;
	}
</script>

{#if loading}
	<div class="h-8 w-full animate-pulse rounded-md"></div>
{:else if record === undefined || evaluationTemplate === undefined}
	<div>Evaluation template not found</div>
{:else}
	<Card size="xl">
		<BaseEvaluationTemplateListItem {record} {evaluationTemplate} />
	</Card>
{/if}
