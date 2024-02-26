<script lang="ts">
	import { onMount, getContext } from 'svelte';
	import { Spinner } from 'flowbite-svelte';
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
	import type { GrantPoolOutcome } from './types';
	import GrantPoolOutcomeDetail from './GrantPoolOutcomeDetail.svelte';

	export let grantPoolHash: ActionHash;

	let client: AppAgentClient = (getContext(clientContext) as any).getClient();

	let links: Array<Link> | undefined;

	let loading = true;
	let error: any = undefined;

	$: links, loading, error;

	onMount(async () => {
		if (grantPoolHash === undefined) {
			throw new Error(
				`The grantPoolHash input is required for the GrantPoolOutcomesForGrantPool element`
			);
		}

		try {
			links = await client.callZome({
				cap_secret: null,
				role_name: 'grant_pools',
				zome_name: 'grants',
				fn_name: 'get_grant_pool_outcomes_for_grant_pool',
				payload: grantPoolHash
			});
		} catch (e) {
			error = e;
		}
		loading = false;
	});
</script>

{#if loading}
	<div style="display: flex; flex: 1; align-items: center; justify-content: center">
		<Spinner class="h-4 w-4" />
	</div>
{:else if error}
	<span>Error fetching grant pool outcomes: ${error.data.data}.</span>
{:else if links.length === 0}
	<span>No grant pool outcomes found for this grant pool.</span>
{:else}
	<div style="display: flex; flex-direction: column">
		{#each links as link}
			<div style="margin-bottom: 8px;">
				<GrantPoolOutcomeDetail grantPoolOutcomeHash={link.target}></GrantPoolOutcomeDetail>
			</div>
		{/each}
	</div>
{/if}
