<script lang="ts">
	import GrantPoolListItem from './GrantPoolListItem.svelte';
	import RecordList from '$lib/components/RecordList.svelte';
</script>

<RecordList
	entryType="GrantPool"
	callZomeRequest={{
		cap_secret: null,
		role_name: 'grant_pools',
		zome_name: 'grants',
		fn_name: 'get_all_grant_pools',
		payload: null
	}}
>
	<svelte:fragment let:hash>
		<GrantPoolListItem grantPoolHash={hash} />
	</svelte:fragment>
</RecordList>
