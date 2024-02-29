<script lang="ts">
	import { toasts } from '$lib/stores/toast';
	import type { GrantsSignal } from '../../grant_pools/grants/types';
	import type { ActionHash, AgentPubKey, AppAgentCallZomeRequest, Link } from '@holochain/client';
	import { onMount } from 'svelte';
	import { holochainClient } from '$lib/stores/holochainClient';
	import Alert from 'flowbite-svelte/Alert.svelte';

	export let entryType: string;
	export let callZomeRequest: AppAgentCallZomeRequest;

	let hashes: Array<ActionHash> = [];
	let loading = true;

	onMount(async () => {
		try {
			await fetch();
			$holochainClient.client.on('signal', (signal) => {
				if (signal.zome_name !== 'grants') return;
				const payload = signal.payload as GrantsSignal;
				if (payload.type !== 'EntryCreated') return;
				if (payload.app_entry.type !== entryType) return;
				hashes = [...hashes, payload.action.hashed.hash];
			});
		} catch (e) {
			toasts.error(`Error fetching record: ${e}`);
		}
	});

	async function fetch() {
		try {
			const links = await $holochainClient.client.callZome(callZomeRequest);
			hashes = links.map((l: Link) => l.target);
		} catch (e) {
			toasts.error(`Error fetching records: ${e as string}`);
		}
		loading = false;
	}
</script>

{#if loading}
	{#each Array(5) as i}
		<div class="mb-2 h-12 w-full animate-pulse rounded-lg bg-gray-200 dark:bg-slate-800"></div>
	{/each}
{:else if hashes.length === 0}
	<Alert class="my-8 text-center">None found.</Alert>
{:else}
	<div class="flex flex-col">
		{#each hashes as hash}
			<div class="mb-4">
				<slot {hash} />
			</div>
		{/each}
	</div>
{/if}
