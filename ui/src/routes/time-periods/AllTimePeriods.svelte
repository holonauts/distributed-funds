<script lang="ts">
	import TimePeriodListItem from '$lib/components/TimePeriodListItem.svelte';
	import RecordList from '$lib/components/RecordList.svelte';
	import BaseBordered from '$lib/components/BaseBordered.svelte';
</script>

<RecordList
	entryType="TimePeriod"
	callZomeRequest={{
		cap_secret: null,
		role_name: 'grant_pools',
		zome_name: 'grants',
		fn_name: 'get_all_time_periods',
		payload: null
	}}
>
	<svelte:fragment let:hash>
		<BaseBordered><TimePeriodListItem timePeriodHash={hash} /></BaseBordered>
	</svelte:fragment>
</RecordList>
