<script lang="ts">
	import { createEventDispatcher, onMount } from 'svelte';
	import { Button, Radio } from 'flowbite-svelte';
	import { decode } from '@msgpack/msgpack';
	import { type Record, type ActionHash, encodeHashToBase64 } from '@holochain/client';
	import type { EvaluationTemplate } from '../../../grant_pools/grants/types';
	import { holochainClient } from '$lib/stores/holochainClient';
	import EvaluationTemplateListItem from '$lib/components/EvaluationTemplateListItem.svelte';
	const dispatch = createEventDispatcher<{
		clone: EvaluationTemplate;
	}>();
	export let evaluationTemplateHash: ActionHash;
	export let group: string;

	let loading = true;
	let record: Record | undefined;
	let evaluationTemplate: EvaluationTemplate;

	onMount(async () => {
		if (evaluationTemplateHash === undefined) {
			throw new Error(
				`The evaluationTemplateHash input is required for the EvaluationTemplateDetail element`
			);
		}
		await fetchEvaluationTemplate();
	});

	async function fetchEvaluationTemplate() {
		loading = true;
		record = undefined;

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
	<div class="flex w-full items-start justify-start space-x-4">
		<Radio bind:group name="hash" value={encodeHashToBase64(evaluationTemplateHash)} />
		<EvaluationTemplateListItem {evaluationTemplateHash} />
		<Button size="xs" color="alternative" on:click={() => dispatch('clone', evaluationTemplate)}>
			Clone
		</Button>
	</div>
{/if}
