<script lang="ts">
	import { decodeHashFromBase64, type ActionHash, encodeHashToBase64 } from '@holochain/client';
	import { onMount } from 'svelte';
	import type { GrantsSignal } from '../../../grant_pools/grants/types';
	import RadioEvaluationTemplateListItem from './RadioEvaluationTemplateListItem.svelte';
	import { toasts } from '$lib/stores/toast';
	import BaseListHashes from '$lib/components/BaseListHashes.svelte';
	import { holochainClient } from '$lib/stores/holochainClient';
	import { Button, Helper, Label, Modal } from 'flowbite-svelte';
	import CreateEvaluationTemplate from './CreateEvaluationTemplate.svelte';
	import { FileSolid, PlusSolid } from 'flowbite-svelte-icons';
	import EvaluationTemplateListItem from '$lib/components/EvaluationTemplateListItem.svelte';

	export let value: ActionHash | undefined;

	let hashes: Array<ActionHash> = [];
	let loading = true;
	let showCreateModal = false;
	let showSelectModal = false;
	let valueBase64: string = '';

	$: {
		if (valueBase64 !== '') {
			value = decodeHashFromBase64(valueBase64);
			showCreateModal = false;
			showSelectModal = false;
		}
	}

	onMount(async () => {
		await fetchEvaluationTemplate();
		$holochainClient.client.on('signal', (signal) => {
			if (signal.zome_name !== 'grants') return;
			const payload = signal.payload as GrantsSignal;
			if (payload.type !== 'EntryCreated') return;
			if (payload.app_entry.type !== 'EvaluationTemplate') return;
			hashes = [...hashes, payload.action.hashed.hash];
		});
	});

	async function fetchEvaluationTemplate() {
		try {
			const links = await $holochainClient.client.callZome({
				cap_secret: null,
				role_name: 'grant_pools',
				zome_name: 'grants',
				fn_name: 'get_all_evaluation_templates',
				payload: null
			});
			hashes = links.map((l) => l.target);
		} catch (e) {
			toasts.error(`Failed to fetch Evaluation Template ${e}`);
		}
		loading = false;
	}
</script>

<div class="flex w-full items-center justify-between">
	<Label>Evaluation Template</Label>
	<div class="mb-2 flex items-center space-x-4">
		<Button on:click={() => (showSelectModal = true)} size="xs" class="mt-1 px-2 py-1" color="blue">
			<FileSolid class="mr-2 h-4 w-4" />
			Select
		</Button>

		<Button
			on:click={() => (showCreateModal = true)}
			size="xs"
			class="mt-1 px-2 py-1"
			color="green"
		>
			<PlusSolid class="mr-2 h-4 w-4" />
			Create
		</Button>
	</div>
</div>

<div class="mb-2">
	{#if value !== undefined}
		<EvaluationTemplateListItem evaluationTemplateHash={value} />
	{:else}
		<p class="dark:text-700 text-sm dark:text-gray-400">No template selected</p>
	{/if}
</div>
<Helper>The evaluation form that grant applicants will be required to submit.</Helper>

<Modal size="lg" outsideclose title="Create Evaluation Template" bind:open={showCreateModal}>
	<CreateEvaluationTemplate
		on:evaluation-template-created={(e) => {
			valueBase64 = encodeHashToBase64(e.detail.evaluationTemplateHash);
			value = e.detail.evaluationTemplateHash;
			showCreateModal = false;
		}}
	/>
</Modal>

<Modal size="lg" outsideclose title="Select Evaluation Template" bind:open={showSelectModal}>
	<BaseListHashes {loading} {hashes}>
		<svelte:fragment let:hash>
			<RadioEvaluationTemplateListItem evaluationTemplateHash={hash} bind:group={valueBase64} />
		</svelte:fragment>
	</BaseListHashes>
</Modal>