<script lang="ts">
	import { onMount, getContext } from 'svelte';
	import '@material/mwc-circular-progress';
	import type {
		EntryHash,
		Record,
		AgentPubKey,
		ActionHash,
		AppAgentClient,
		NewEntryAction
	} from '@holochain/client';
	import { clientContext } from '../../contexts';
	import GrantPoolDetail from './GrantPoolDetail.svelte';
	import type { GrantsSignal } from './types';

	let client: AppAgentClient = (getContext(clientContext) as any).getClient();

	let hashes: Array<ActionHash> | undefined;
	let loading = true;
	let error: any = undefined;

	$: hashes, loading, error;

	onMount(async () => {
		await fetchGrantPools();
		client.on('signal', (signal) => {
			if (signal.zome_name !== 'grants') return;
			const payload = signal.payload as GrantsSignal;
			if (payload.type !== 'EntryCreated') return;
			if (payload.app_entry.type !== 'GrantPool') return;
			hashes = [...hashes, payload.action.hashed.hash];
		});
	});

	async function fetchGrantPools() {
		try {
			const links = await client.callZome({
				cap_secret: null,
				role_name: 'grant_pools',
				zome_name: 'grants',
				fn_name: 'get_all_grant_pools',
				payload: null
			});
			hashes = links.map((l) => l.target);
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
	<span>Error fetching the grant pools: {error.data.data}.</span>
{:else if hashes.length === 0}
	<span>No grant pools found.</span>
{:else}
	<div style="display: flex; flex-direction: column">
		{#each hashes as hash}
			<div style="margin-bottom: 8px;">
				<GrantPoolDetail grantPoolHash={hash} on:grant-pool-deleted={() => fetchGrantPools()}
				></GrantPoolDetail>
			</div>
		{/each}
	</div>
{/if}
