<script lang="ts">
	import ApplicationListItem from '$lib/components/ApplicationListItem.svelte';
	import RecordList from '$lib/components/RecordList.svelte';
	import { decodeHashFromBase64 } from '@holochain/client';
	import { page } from '$app/stores';
	import BaseBreadcrumbs from '$lib/components/BaseBreadcrumbs.svelte';
	import RecordDetail from '$lib/components/RecordDetail.svelte';

	$: actionHash = decodeHashFromBase64($page.params.actionHashB64);
</script>

<RecordDetail
	callZomeRequest={{
		cap_secret: null,
		role_name: 'grant_pools',
		zome_name: 'grants',
		fn_name: 'get_grant_pool',
		payload: actionHash
	}}
>
	<svelte:fragment let:entry={grantPool}>
		<BaseBreadcrumbs
			title="Applications"
			replacements={{ [$page.params.actionHashB64]: grantPool.name }}
		/>

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
	</svelte:fragment>
</RecordDetail>
