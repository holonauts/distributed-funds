<script lang="ts">
	import { type ActionHash } from '@holochain/client';
	import RecordDetail from '$lib/components/RecordDetail.svelte';
	import InputScore from '$lib/components/InputScore.svelte';
	import BaseTemplateListItem from './BaseTemplateListItem.svelte';
	import { createEventDispatcher } from 'svelte';
	const dispatch = createEventDispatcher();

	export let evaluationTemplateHash: ActionHash;
	export let showCloneButton = false;
	let showDetails = false;
</script>

<RecordDetail
	callZomeRequest={{
		cap_secret: null,
		role_name: 'grant_pools',
		zome_name: 'grants',
		fn_name: 'get_evaluation_template',
		payload: evaluationTemplateHash
	}}
>
	<svelte:fragment let:record let:entry>
		<BaseTemplateListItem
			{showCloneButton}
			{showDetails}
			name={entry.name}
			author={record.signed_action.hashed.content.author}
			timestamp={record.signed_action.hashed.content.timestamp / 1000}
			formDefinition={JSON.parse(entry.form_schema).formDefinition}
			on:clone={() => dispatch('clone', entry)}
		>
			<InputScore scoreTemplate={entry.score} scoreRange={entry.score_range} />
		</BaseTemplateListItem>
	</svelte:fragment>
</RecordDetail>
