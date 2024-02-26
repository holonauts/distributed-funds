<script lang="ts">
	import { createEventDispatcher, onMount, getContext } from 'svelte';
	import '@material/mwc-circular-progress';
	import { decode } from '@msgpack/msgpack';
	import type {
		Record,
		ActionHash,
		AppAgentClient,
		EntryHash,
		AgentPubKey,
		DnaHash
	} from '@holochain/client';
	import { clientContext } from '../../contexts';
	import type { TimePeriod } from './types';
	import '@material/mwc-circular-progress';
	import type { Snackbar } from '@material/mwc-snackbar';
	import '@material/mwc-snackbar';
	import '@material/mwc-icon-button';

	const dispatch = createEventDispatcher();

	export let timePeriodHash: ActionHash;

	let client: AppAgentClient = (getContext(clientContext) as any).getClient();

	let loading = true;
	let error: any = undefined;

	let record: Record | undefined;
	let timePeriod: TimePeriod | undefined;

	$: error, loading, record, timePeriod;

	onMount(async () => {
		if (timePeriodHash === undefined) {
			throw new Error(`The timePeriodHash input is required for the TimePeriodDetail element`);
		}
		await fetchTimePeriod();
	});

	async function fetchTimePeriod() {
		loading = true;
		error = undefined;
		record = undefined;
		timePeriod = undefined;

		try {
			record = await client.callZome({
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
			error = e;
		}

		loading = false;
	}
</script>

{#if loading}
	<div style="display: flex; flex: 1; align-items: center; justify-content: center">
		<mwc-circular-progress indeterminate></mwc-circular-progress>
	</div>
{:else if error}
	<span>Error fetching the time period: {error.data.data}</span>
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
