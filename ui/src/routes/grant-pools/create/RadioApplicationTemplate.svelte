<script lang="ts">
	import { type ActionHash, encodeHashToBase64 } from '@holochain/client';
	import ApplicationTemplateListItem from './ApplicationTemplateListItem.svelte';
	import BaseRadioListItem from '$lib/components/BaseRadioListItem.svelte';

	export let applicationTemplateHash: ActionHash;
	export let group: string;
</script>

<BaseRadioListItem bind:group value={encodeHashToBase64(applicationTemplateHash)}>
	<ApplicationTemplateListItem {applicationTemplateHash} on:clone showCloneButton />
</BaseRadioListItem>
