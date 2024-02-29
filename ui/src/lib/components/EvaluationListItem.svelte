<script lang="ts">
	import { type ActionHash } from '@holochain/client';
	import RecordListItem from '$lib/components/RecordDetail.svelte';
	import BaseLabelContent from '$lib/components/BaseLabelContent.svelte';
	import { Card } from 'flowbite-svelte';
	import ProfileInline from '$lib/components/ProfileInline.svelte';
	import BaseScore from '$lib/components/BaseScore.svelte';

	export let evaluationHash: ActionHash;
</script>

<RecordListItem
	callZomeRequest={{
		cap_secret: null,
		role_name: 'grant_pools',
		zome_name: 'grants',
		fn_name: 'get_evaluation',
		payload: evaluationHash
	}}
>
	<svelte:fragment let:record let:entry>
		<Card size="xl">
			<BaseLabelContent label="Evaluator" class="mb-8">
				<ProfileInline agentPubKey={record.signed_action.hashed.content.author} />
			</BaseLabelContent>

			<BaseLabelContent label="Comments" class="mb-8">
				{entry.comments}
			</BaseLabelContent>

			<BaseScore score={entry.score} />
		</Card>
	</svelte:fragment>
</RecordListItem>
