<script lang="ts">
	import ApplicationListItem from './ApplicationListItem.svelte';
	import RecordList from '$lib/components/RecordList.svelte';
	import { decodeHashFromBase64 } from '@holochain/client';
	import { page } from '$app/stores';

	$: actionHash = decodeHashFromBase64($page.params.actionHashB64);
</script>

<RecordList
	entryType="Application"
	callZomeRequest={{
		cap_secret: null,
		role_name: 'grant_pools',
		zome_name: 'grants',
		fn_name: 'get_applications_for_grant_pool',
		payload: actionHash
	}}
>
	<svelte:fragment let:hash>
		<ApplicationListItem applicationHash={hash} />
	</svelte:fragment>
</RecordList>
