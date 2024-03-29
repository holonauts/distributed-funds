<script lang="ts">
	import { decodeHashFromBase64, type ActionHash, encodeHashToBase64 } from '@holochain/client';
	import { onMount } from 'svelte';
	import type { GrantsSignal } from '../../../grant_pools/grants/types';
	import RadioApplicationTemplateListItem from './RadioApplicationTemplate.svelte';
	import { toasts } from '$lib/stores/toast';
	import BaseListHashes from '$lib/components/BaseListHashes.svelte';
	import { holochainClient } from '$lib/stores/holochainClient';
	import { Button, Card, Modal } from 'flowbite-svelte';
	import CreateApplicationTemplate from './CreateApplicationTemplate.svelte';
	import { FileSolid, PlusSolid } from 'flowbite-svelte-icons';
	import ApplicationTemplateListItem from './ApplicationTemplateListItem.svelte';
	import BaseLabelContent from '$lib/components/BaseLabelContent.svelte';
	import BaseHelper from '$lib/components/BaseHelper.svelte';
	import BaseBordered from '$lib/components/BaseBordered.svelte';
	export let value: ActionHash | undefined;

	let hashes: Array<ActionHash> = [];
	let loading = true;
	let showCreateModal = false;
	let showSelectModal = false;
	let valueBase64: string = '';
	let name = '';
	let formSchema = '';

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

	function reset() {
		name = '';
		formSchema = '';
		showCreateModal = false;
		showSelectModal = false;
	}
</script>

<div class="flex w-full items-center justify-between">
	<BaseLabelContent label="Application Template" />
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
		<Card size="xl">
			<ApplicationTemplateListItem applicationTemplateHash={value} />
		</Card>
	{:else}
		<BaseBordered>No template selected</BaseBordered>
	{/if}
</div>
<BaseHelper>The application form that grant applicants will be required to submit.</BaseHelper>

<Modal
	size="lg"
	outsideclose
	title="Create Application Template"
	bind:open={showCreateModal}
	on:close={reset}
>
	<CreateApplicationTemplate
		{name}
		{formSchema}
		on:application-template-created={(e) => {
			valueBase64 = encodeHashToBase64(e.detail.applicationTemplateHash);
			value = e.detail.applicationTemplateHash;
			showCreateModal = false;
		}}
	/>
</Modal>

<Modal size="lg" outsideclose title="Select Application Template" bind:open={showSelectModal}>
	<BaseListHashes {loading} {hashes}>
		<svelte:fragment let:hash>
			<RadioApplicationTemplateListItem
				applicationTemplateHash={hash}
				bind:group={valueBase64}
				on:clone={(e) => {
					name = e.detail.name;
					formSchema = e.detail.form_schema;
					showSelectModal = false;
					showCreateModal = true;
				}}
			/>
		</svelte:fragment>
	</BaseListHashes>
</Modal>
