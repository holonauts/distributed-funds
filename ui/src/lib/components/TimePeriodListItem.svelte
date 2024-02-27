<script lang="ts">
	import { decode } from '@msgpack/msgpack';
	import { type Record, type ActionHash } from '@holochain/client';
	import type { TimePeriod } from '../../grant_pools/grants/types';
	import { holochainClient } from '$lib/stores/holochainClient';
	import BaseTimePeriodListItem from '$lib/components/BaseTimePeriodListItem.svelte';
	import { Card } from 'flowbite-svelte';

	export let timePeriodHash: ActionHash;

	let loading = true;
	let record: Record | undefined;
	let timePeriod: TimePeriod | undefined;

	$: timePeriodHash, fetchTimePeriod();

	async function fetchTimePeriod() {
		loading = true;
		record = undefined;
		timePeriod = undefined;

		try {
			record = await $holochainClient.client.callZome({
				cap_secret: null,
				role_name: 'grant_pools',
				zome_name: 'grants',
				fn_name: 'get_time_period',
				payload: timePeriodHash
			});
			if (record) {
				timePeriod = decode((record.entry as any).Present.entry) as TimePeriod;
			}
		} catch (e) {
			console.error(e);
		}

		loading = false;
	}
</script>

{#if loading}
	<div class="h-8 w-full animate-pulse rounded-md"></div>
{:else if record === undefined || timePeriod === undefined}
	<div>Application template not found</div>
{:else}
	<Card size="xl">
		<BaseTimePeriodListItem {record} {timePeriod} />
	</Card>
{/if}
