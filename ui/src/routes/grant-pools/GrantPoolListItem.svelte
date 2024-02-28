<script lang="ts">
	import type { ActionHash } from '@holochain/client';
	import RecordListItem from '$lib/components/RecordListItem.svelte';
	import { Label } from 'flowbite-svelte';
	export let grantPoolHash: ActionHash;
</script>

<RecordListItem
	callZomeRequest={{
		cap_secret: null,
		role_name: 'grant_pools',
		zome_name: 'grants',
		fn_name: 'get_grant_pool',
		payload: grantPoolHash
	}}
>
	<svelte:fragment let:entry>
		<div class="mb-4 text-2xl">{entry.name}</div>

		<div class="mb-2">
			<Label>Purpose</Label>
			<div>{entry.purpose_description}</div>
		</div>

		<div class="mb-2">
			<Label>Application Period</Label>
			<div>{entry.purpose_description}</div>
		</div>
	</svelte:fragment>
</RecordListItem>
