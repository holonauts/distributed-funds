<script lang="ts">
	import ApplicationListItem from '$lib/components/ApplicationListItem.svelte';
	import BaseBreadcrumbs from '$lib/components/BaseBreadcrumbs.svelte';
	import RecordList from '$lib/components/RecordList.svelte';
	import { holochainClient } from '$lib/stores/holochainClient';
</script>

<BaseBreadcrumbs title="My Applications" />

<RecordList
	entryType=" "
	callZomeRequest={{
		cap_secret: null,
		role_name: 'grant_pools',
		zome_name: 'grants',
		fn_name: 'get_my_applications',
		payload: $holochainClient.client.myPubKey
	}}
>
	<svelte:fragment let:hash>
		<ApplicationListItem applicationHash={hash} />
	</svelte:fragment>
</RecordList>
