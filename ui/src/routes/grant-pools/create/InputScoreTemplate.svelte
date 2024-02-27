<script lang="ts">
	import { Card, Label, TabItem, Tabs } from 'flowbite-svelte';
	import type {
		NumberRange,
		ScoreTemplate,
		AttributeScoreTemplate
	} from '../../../grant_pools/grants/types';
	import { ScoreType } from '../../../grant_pools/grants/types';
	import InputNumberRange from '$lib/components/InputNumberRange.svelte';
	import InputScoreTemplateWeighted from './InputScoreTemplateWeighted.svelte';

	let type: ScoreType = ScoreType.Single;
	let scoreRange: NumberRange;
	let attributeScoreTemplates: AttributeScoreTemplate[] = [];

	export let value: ScoreTemplate;

	$: {
		value = {
			type,
			content: type === ScoreType.Single ? undefined : attributeScoreTemplates
		};
	}
</script>

<Tabs
	contentClass="mb-4"
	style="full"
	defaultClass="flex rounded-lg divide-x rtl:divide-x-reverse divide-gray-200 shadow dark:divide-gray-700"
>
	<TabItem
		defaultClass="!p-2 text-xs"
		title="Single Score"
		class="w-full"
		open={type === ScoreType.Single}
		on:click={() => (type = ScoreType.Single)}
	/>
	<TabItem
		defaultClass="!p-2 text-xs"
		title="Weighted Attributes Score"
		class="w-full"
		open={type === ScoreType.Weighted}
		on:click={() => (type = ScoreType.Weighted)}
	/>
</Tabs>

<Card size="xl">
	<Label>Allowed Score Range</Label>
	<InputNumberRange bind:value={scoreRange} />

	{#if type === ScoreType.Weighted}
		<div class="mt-4">
			<InputScoreTemplateWeighted bind:value={attributeScoreTemplates} />
		</div>
	{/if}
</Card>
