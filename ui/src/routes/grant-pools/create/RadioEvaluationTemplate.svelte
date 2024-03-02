<script lang="ts">
	import { type ActionHash, encodeHashToBase64 } from '@holochain/client';
	import EvaluationTemplateListItem from './EvaluationTemplateListItem.svelte';
	import BaseRadioListItem from '$lib/components/BaseRadioListItem.svelte';

	export let evaluationTemplateHash: ActionHash;
	export let group: string;
</script>

<BaseRadioListItem bind:group value={encodeHashToBase64(evaluationTemplateHash)}>
	<EvaluationTemplateListItem {evaluationTemplateHash} showCloneButton={true} on:clone />
</BaseRadioListItem>
