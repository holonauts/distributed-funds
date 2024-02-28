<script lang="ts">
	import { type ActionHash } from '@holochain/client';
	import RecordDetail from './RecordDetail.svelte';
	import BaseTimePeriodBadge from './BaseTimePeriodBadge.svelte';

	export let timePeriodHash: ActionHash;
	export let activeText = 'Active';
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
		<BaseTimePeriodBadge timePeriod={entry} {activeText} />
	</svelte:fragment>
</RecordDetail>
