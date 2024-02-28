<script lang="ts">
	import { onMount } from 'svelte';
	import { decode } from '@msgpack/msgpack';
	import type { Record, ActionHash } from '@holochain/client';
	import type { TimePeriod } from './types';
	import { Spinner } from 'flowbite-svelte';
	import { toasts } from '$lib/stores/toast';
	import { holochainClient } from '$lib/stores/holochainClient';

	export let timePeriodHash: ActionHash;

	let loading = true;
	let record: Record | undefined;
	let timePeriod: TimePeriod;

	onMount(async () => {
		await fetchTimePeriod();
	});

	async function fetchTimePeriod() {
		loading = true;

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
			toasts.error(e);
		}

		loading = false;
	}
</script>

{#if loading}
	<div class="flex items-center justify-center">
		<Spinner class="h-4 w-4" />
	</div>
{:else}
	<div style="display: flex; flex-direction: column">
		<div style="display: flex; flex-direction: row">
			<span style="flex: 1"></span>
		</div>

		<div style="display: flex; flex-direction: row; margin-bottom: 16px">
			<span style="margin-right: 4px"><strong>Start At:</strong></span>
			<span style="white-space: pre-line"
				>{new Date(timePeriod.start_at / 1000).toLocaleString()}</span
			>
		</div>

		<div style="display: flex; flex-direction: row; margin-bottom: 16px">
			<span style="margin-right: 4px"><strong>End At:</strong></span>
			<span style="white-space: pre-line"
				>{new Date(timePeriod.end_at / 1000).toLocaleString()}</span
			>
		</div>
	</div>
{/if}
