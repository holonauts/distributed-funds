<script lang="ts">
	import { type ActionHash } from '@holochain/client';
	import { formatTimestampLocal } from '$lib/utils/date';
	import RecordDetail from './RecordDetail.svelte';

	export let timePeriodHash: ActionHash;
</script>

<RecordDetail
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
				<div class="dark:text-white">
					{formatTimestampLocal(entry.start_at)} -
					{formatTimestampLocal(entry.end_at)}
				</div>
			</div>
		</div>
	</svelte:fragment>
</RecordDetail>
