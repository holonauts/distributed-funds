<script lang="ts">
	import { onMount } from 'svelte';
	import type { ActionHash } from '@holochain/client';
	import TimePeriodDetail from '../../grant_pools/grants/TimePeriodDetail.svelte';
	import type { GrantsSignal } from '../../grant_pools/grants/types';
	import { toasts } from '$lib/stores/toast';
	import { holochainClient } from '$lib/stores/holochainClient';
	import BaseListHashes from '$lib/components/BaseListHashes.svelte';

	let hashes: Array<ActionHash> = [];
	let loading = true;

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
			toasts.error(e);
		}
		loading = false;
	}
</script>

<BaseListHashes {loading} {hashes}>
	<svelte:fragment let:hash>
		<TimePeriodDetail timePeriodHash={hash} on:time-period-deleted={() => fetchTimePeriods()} />
	</svelte:fragment>
</BaseListHashes>
