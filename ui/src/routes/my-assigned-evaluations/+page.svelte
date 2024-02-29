<script lang="ts">
	import EvaluationListItem from '$lib/components/EvaluationListItem.svelte';
	import BaseBreadcrumbs from '$lib/components/BaseBreadcrumbs.svelte';
	import RecordList from '$lib/components/RecordList.svelte';
	import { holochainClient } from '$lib/stores/holochainClient';
</script>

<BaseBreadcrumbs title="My Assigned Evaluations" />

<RecordList
	entryType=" "
	callZomeRequest={{
		cap_secret: null,
		role_name: 'grant_pools',
		zome_name: 'grants',
		fn_name: 'get_applications_for_evaluator',
		payload: $holochainClient.client.myPubKey
	}}
>
	<svelte:fragment let:hash>
		<EvaluationListItem applicationHash={hash} />
	</svelte:fragment>
</RecordList>
