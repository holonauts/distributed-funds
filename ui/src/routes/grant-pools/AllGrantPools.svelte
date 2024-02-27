<script lang="ts">
	import { onMount } from 'svelte';
	import type { ActionHash, Link } from '@holochain/client';
	import GrantPoolDetail from '../../grant_pools/grants/GrantPoolDetail.svelte';
	import type { GrantsSignal } from '../../grant_pools/grants/types';
	import { toasts } from '$lib/stores/toast';
	import { holochainClient } from '$lib/stores/holochainClient';
	import BaseListHashes from '$lib/components/BaseListHashes.svelte';

	let hashes: Array<ActionHash> = [];
	let loading = true;

	onMount(async () => {
		try {
			await fetchGrantPools();
			$holochainClient.client.on('signal', (signal) => {
				if (signal.zome_name !== 'grants') return;
				const payload = signal.payload as GrantsSignal;
				if (payload.type !== 'EntryCreated') return;
				if (payload.app_entry.type !== 'GrantPool') return;
				hashes = [...hashes, payload.action.hashed.hash];
			});
		} catch (e) {
			toasts.error(`Error fetching the grant pools: ${e as string}`);
		}
	});

	async function fetchGrantPools() {
		try {
			const links = await $holochainClient.client.callZome({
				cap_secret: null,
				role_name: 'grant_pools',
				zome_name: 'grants',
				fn_name: 'get_all_grant_pools',
				payload: null
			});
			hashes = links.map((l: Link) => l.target);
		} catch (e) {
			toasts.error(`Error fetching the grant pools: ${e as string}`);
		}
		loading = false;
	}
</script>

<BaseListHashes {loading} {hashes}>
	<svelte:fragment let:hash>
		<GrantPoolDetail grantPoolHash={hash} on:grant-pool-deleted={() => fetchGrantPools()} />
	</svelte:fragment>
</BaseListHashes>
