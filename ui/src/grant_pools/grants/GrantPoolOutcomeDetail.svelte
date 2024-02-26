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
	import type { GrantPoolOutcome } from './types';
	import '@material/mwc-circular-progress';
	import type { Snackbar } from '@material/mwc-snackbar';
	import '@material/mwc-snackbar';
	import '@material/mwc-icon-button';

	const dispatch = createEventDispatcher();

	export let grantPoolOutcomeHash: ActionHash;

	let client: AppAgentClient = (getContext(clientContext) as any).getClient();

	let loading = true;
	let error: any = undefined;

	let record: Record | undefined;
	let grantPoolOutcome: GrantPoolOutcome | undefined;

	$: error, loading, record, grantPoolOutcome;

	onMount(async () => {
		if (grantPoolOutcomeHash === undefined) {
			throw new Error(
				`The grantPoolOutcomeHash input is required for the GrantPoolOutcomeDetail element`
			);
		}
		await fetchGrantPoolOutcome();
	});

	async function fetchGrantPoolOutcome() {
		loading = true;
		error = undefined;
		record = undefined;
		grantPoolOutcome = undefined;

		try {
			record = await client.callZome({
				cap_secret: null,
				role_name: 'grant_pools',
				zome_name: 'grants',
				fn_name: 'get_grant_pool_outcome',
				payload: grantPoolOutcomeHash
			});
			if (record) {
				grantPoolOutcome = decode((record.entry as any).Present.entry) as GrantPoolOutcome;
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
	<span>Error fetching the grant pool outcome: {error.data.data}</span>
{:else}
	<div style="display: flex; flex-direction: column">
		<div style="display: flex; flex-direction: row">
			<span style="flex: 1"></span>
		</div>
	</div>
{/if}
