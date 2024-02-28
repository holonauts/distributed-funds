<script lang="ts">
	import { type ActionHash } from '@holochain/client';
	import { formatTimestampLocal } from '$lib/utils/date';
	import RecordListItem from './RecordListItem.svelte';

	export let timePeriodHash: ActionHash;
</script>

<RecordListItem
	callZomeRequest={{
		cap_secret: null,
		role_name: 'grant_pools',
		zome_name: 'grants',
		fn_name: 'get_time_period',
		payload: timePeriodHash
	}}
>
	<svelte:fragment let:entry>
		<div class="flex w-full items-center justify-between">
			<div class="w-full">
				<div class="text-xs dark:text-white">
					{formatTimestampLocal(entry.start_at)} -
					{formatTimestampLocal(entry.end_at)}
				</div>
			</div>
		</div>
	</svelte:fragment>
</RecordListItem>
