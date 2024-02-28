<script lang="ts">
	import { type ActionHash } from '@holochain/client';
	import RecordListItem from '$lib/components/RecordListItem.svelte';
	import { createEventDispatcher } from 'svelte';
	import BaseTemplateListItem from './BaseTemplateListItem.svelte';
	const dispatch = createEventDispatcher();

	export let applicationTemplateHash: ActionHash;
	export let showCloneButton = false;
	let showDetails = false;
</script>

<RecordListItem
	callZomeRequest={{
		cap_secret: null,
		role_name: 'grant_pools',
		zome_name: 'grants',
		fn_name: 'get_application_template',
		payload: applicationTemplateHash
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
		/>
	</svelte:fragment>
</RecordListItem>
