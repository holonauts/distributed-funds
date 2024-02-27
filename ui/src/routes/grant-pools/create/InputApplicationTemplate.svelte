<script lang="ts">
	import { decodeHashFromBase64, type ActionHash, encodeHashToBase64 } from '@holochain/client';
	import { onMount } from 'svelte';
	import type { GrantsSignal } from '../../../grant_pools/grants/types';
	import RadioApplicationTemplateListItem from './RadioApplicationTemplateListItem.svelte';
	import { toasts } from '$lib/stores/toast';
	import BaseListHashes from '$lib/components/BaseListHashes.svelte';
	import { holochainClient } from '$lib/stores/holochainClient';
	import { Button, Helper, Label, Modal } from 'flowbite-svelte';
	import CreateApplicationTemplate from './CreateApplicationTemplate.svelte';
	import { FileSolid, PlusSolid } from 'flowbite-svelte-icons';
	import ApplicationTemplateListItem from '$lib/components/ApplicationTemplateListItem.svelte';

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
		await fetchApplicationTemplates();
		$holochainClient.client.on('signal', (signal) => {
			if (signal.zome_name !== 'grants') return;
			const payload = signal.payload as GrantsSignal;
			if (payload.type !== 'EntryCreated') return;
			if (payload.app_entry.type !== 'ApplicationTemplate') return;
			hashes = [...hashes, payload.action.hashed.hash];
		});
	});

	async function fetchApplicationTemplates() {
		try {
			const links = await $holochainClient.client.callZome({
				cap_secret: null,
				role_name: 'grant_pools',
				zome_name: 'grants',
				fn_name: 'get_all_application_templates',
				payload: null
			});
			hashes = links.map((l) => l.target);
		} catch (e) {
			toasts.error(`Failed to fetch Application Template ${e}`);
		}
		loading = false;
	}
</script>

<div class="flex w-full items-center justify-between">
	<Label>Application Template</Label>
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
		<ApplicationTemplateListItem applicationTemplateHash={value} />
	{:else}
		<p class="dark:text-700 text-sm dark:text-gray-400">No template selected</p>
	{/if}
</div>
<Helper>The application form that grant applicants will be required to submit.</Helper>

<Modal size="xl" outsideclose title="Create Application Template" bind:open={showCreateModal}>
	<CreateApplicationTemplate
		on:application-template-created={(e) => {
			valueBase64 = encodeHashToBase64(e.detail.applicationTemplateHash);
			value = e.detail.applicationTemplateHash;
			showCreateModal = false;
		}}
	/>
</Modal>

<Modal size="xl" outsideclose title="Select Application Template" bind:open={showSelectModal}>
	<BaseListHashes {loading} {hashes}>
		<svelte:fragment let:hash>
			<RadioApplicationTemplateListItem applicationTemplateHash={hash} bind:group={valueBase64} />
		</svelte:fragment>
	</BaseListHashes>
</Modal>
