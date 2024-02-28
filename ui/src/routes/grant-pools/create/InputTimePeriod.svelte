<script lang="ts">
	import { decodeHashFromBase64, type ActionHash } from '@holochain/client';
	import { onMount } from 'svelte';
	import type { GrantsSignal } from '../../../grant_pools/grants/types';
	import RadioTimePeriodListItem from './RadioTimePeriod.svelte';
	import { toasts } from '$lib/stores/toast';
	import BaseListHashes from '$lib/components/BaseListHashes.svelte';
	import { holochainClient } from '$lib/stores/holochainClient';
	import { Button, Helper, Label, Modal } from 'flowbite-svelte';
	import { FileSolid } from 'flowbite-svelte-icons';
	import TimePeriodListItem from '$lib/components/TimePeriodListItem.svelte';
	import BaseLabelContent from '$lib/components/BaseLabelContent.svelte';
	import BaseHelper from '$lib/components/BaseHelper.svelte';
	import BaseBordered from '$lib/components/BaseBordered.svelte';

	export let value: ActionHash | undefined;

	let hashes: Array<ActionHash> = [];
	let loading = true;
	let showSelectModal = false;
	let valueBase64: string = '';

	$: {
		if (valueBase64 !== '') {
			value = decodeHashFromBase64(valueBase64);
			showSelectModal = false;
		}
	}

	onMount(async () => {
		await fetchTimePeriods();
		$holochainClient.client.on('signal', (signal) => {
			if (signal.zome_name !== 'grants') return;
			const payload = signal.payload as GrantsSignal;
			if (payload.type !== 'EntryCreated') return;
			if (payload.app_entry.type !== 'TimePeriod') return;
			hashes = [...hashes, payload.action.hashed.hash];
		});
	});

	async function fetchTimePeriods() {
		try {
			const links = await $holochainClient.client.callZome({
				cap_secret: null,
				role_name: 'grant_pools',
				zome_name: 'grants',
				fn_name: 'get_all_time_periods',
				payload: null
			});
			hashes = links.map((l) => l.target);
		} catch (e) {
			toasts.error(`Failed to fetch Time Period ${e}`);
		}
		loading = false;
	}
</script>

<div class="flex w-full items-center justify-between">
	<BaseLabelContent label="Time Period" />
	<div class="mb-2 flex items-center space-x-4">
		<Button on:click={() => (showSelectModal = true)} size="xs" class="mt-1 px-2 py-1" color="blue">
			<FileSolid class="mr-2 h-4 w-4" />
			Select
		</Button>
	</div>
</div>

<div class="mb-2">
	{#if value !== undefined}
		<TimePeriodListItem timePeriodHash={value} />
	{:else}
		<BaseBordered>No time period selected</BaseBordered>
	{/if}
</div>
<BaseHelper>The time period allowed for application submissions.</BaseHelper>

<Modal size="xl" outsideclose title="Select Time Period" bind:open={showSelectModal}>
	<BaseListHashes {loading} {hashes}>
		<svelte:fragment let:hash>
			<RadioTimePeriodListItem timePeriodHash={hash} bind:group={valueBase64} />
		</svelte:fragment>
	</BaseListHashes>
</Modal>
