<script lang="ts">
	import { onMount, getContext } from 'svelte';
	import '@material/mwc-circular-progress';
	import type {
		Link,
		ActionHash,
		EntryHash,
		AppAgentClient,
		Record,
		AgentPubKey,
		NewEntryAction
	} from '@holochain/client';
	import { clientContext } from '../../contexts';
	import type { GrantPool } from './types';
	import GrantPoolDetail from './GrantPoolDetail.svelte';

	export let timePeriodHash: ActionHash;

	let client: AppAgentClient = (getContext(clientContext) as any).getClient();

	let links: Array<Link> | undefined;

	let loading = true;
	let error: any = undefined;

	$: links, loading, error;

	onMount(async () => {
		if (timePeriodHash === undefined) {
			throw new Error(
				`The timePeriodHash input is required for the GrantPoolsForTimePeriod element`
			);
		}

		try {
			links = await client.callZome({
				cap_secret: null,
				role_name: 'grant_pools',
				zome_name: 'grants',
				fn_name: 'get_grant_pools_for_time_period',
				payload: timePeriodHash
			});
		} catch (e) {
			error = e;
		}
		loading = false;
	});
</script>

{#if loading}
	<div style="display: flex; flex: 1; align-items: center; justify-content: center">
		<mwc-circular-progress indeterminate></mwc-circular-progress>
	</div>
{:else if error}
	<span>Error fetching grant pools: ${error.data.data}.</span>
{:else if links.length === 0}
	<span>No grant pools found for this time period.</span>
{:else}
	<div style="display: flex; flex-direction: column">
		{#each links as link}
			<div style="margin-bottom: 8px;">
				<GrantPoolDetail grantPoolHash={link.target}></GrantPoolDetail>
			</div>
		{/each}
	</div>
{/if}
