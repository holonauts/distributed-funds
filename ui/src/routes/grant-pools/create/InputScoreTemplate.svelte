<script lang="ts">
	import { Card, Label, TabItem, Tabs } from 'flowbite-svelte';
	import type { NumberRange, ScoreTemplate } from '../../../grant_pools/grants/types';
	import { ScoreType } from '../../../grant_pools/grants/types';
	import InputNumberRange from '$lib/components/InputNumberRange.svelte';
	import InputScoreTemplateWeighted from './InputScoreTemplateWeighted.svelte';

	export let score: ScoreTemplate = {
		type: ScoreType.Single,
		content: []
	};
	export let scoreRange: NumberRange;
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
		open={score.type === ScoreType.Single}
		on:click={() => (score = { type: ScoreType.Single, content: undefined })}
	/>
	<TabItem
		defaultClass="!p-2 text-xs"
		title="Weighted Attributes Score"
		class="w-full"
		open={score.type === ScoreType.Weighted}
		on:click={() => (score = { type: ScoreType.Weighted, content: [] })}
	/>
</Tabs>

<Card size="xl">
	<Label>Allowed Score Range</Label>
	<InputNumberRange bind:value={scoreRange} />

	{#if score.type === ScoreType.Weighted}
		<div class="mt-4">
			<InputScoreTemplateWeighted bind:value={score.content} />
		</div>
	{/if}
</Card>
