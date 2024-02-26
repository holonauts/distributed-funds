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
	import type { GrantPool } from './types';
	import '@material/mwc-circular-progress';
	import type { Snackbar } from '@material/mwc-snackbar';
	import '@material/mwc-snackbar';
	import '@material/mwc-icon-button';

	const dispatch = createEventDispatcher();

	export let grantPoolHash: ActionHash;

	let client: AppAgentClient = (getContext(clientContext) as any).getClient();

	let loading = true;
	let error: any = undefined;

	let record: Record | undefined;
	let grantPool: GrantPool | undefined;

	$: error, loading, record, grantPool;

	onMount(async () => {
		if (grantPoolHash === undefined) {
			throw new Error(`The grantPoolHash input is required for the GrantPoolDetail element`);
		}
		await fetchGrantPool();
	});

	async function fetchGrantPool() {
		loading = true;
		error = undefined;
		record = undefined;
		grantPool = undefined;

		try {
			record = await client.callZome({
				cap_secret: null,
				role_name: 'grant_pools',
				zome_name: 'grants',
				fn_name: 'get_grant_pool',
				payload: grantPoolHash
			});
			if (record) {
				grantPool = decode((record.entry as any).Present.entry) as GrantPool;
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
	<span>Error fetching the grant pool: {error.data.data}</span>
{:else}
	<div style="display: flex; flex-direction: column">
		<div style="display: flex; flex-direction: row">
			<span style="flex: 1"></span>
		</div>

		<div style="display: flex; flex-direction: row; margin-bottom: 16px">
			<span style="margin-right: 4px"><strong>Name:</strong></span>
			<span style="white-space: pre-line">{grantPool.name}</span>
		</div>

		<div style="display: flex; flex-direction: row; margin-bottom: 16px">
			<span style="margin-right: 4px"><strong>Purpose Description:</strong></span>
			<span style="white-space: pre-line">{grantPool.purpose_description}</span>
		</div>

		<div style="display: flex; flex-direction: row; margin-bottom: 16px">
			<span style="margin-right: 4px"><strong>Rules Description:</strong></span>
			<span style="white-space: pre-line">{grantPool.rules_description}</span>
		</div>
	</div>
{/if}
