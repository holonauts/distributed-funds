<script lang="ts">
	import {
		ScoreType,
		type Score,
		type ScoreTemplate,
		type AttributeScore
	} from '../../grant_pools/grants/types';
	import { Range, Label, Input } from 'flowbite-svelte';
	import InputWeightedCriteria from '$lib/components/InputWeightedCriteria.svelte';

	export let scoreTemplate: ScoreTemplate;
	export let value: Score | undefined = undefined;

	let singleRating: number = 0;
	let weightedRating: AttributeScore[] = [];

	$: {
		value = {
			type: scoreTemplate.type,
			content: scoreTemplate.type === ScoreType.Single ? singleRating : weightedRating
		};
	}
</script>

<div class="w-full px-2">
	{#if scoreTemplate.type === ScoreType.Single}
		<Label for="rating">Rating</Label>
		<div class="flex items-center justify-start space-x-2">
			<div class="flex-grow">
				<Range
					id="rating"
					size="sm"
					min={scoreTemplate.content.min}
					max={scoreTemplate.content.max}
					bind:value={singleRating}
				/>
			</div>
			<Input size="sm" bind:value={singleRating} class="w-14" />
		</div>
	{:else if scoreTemplate.type === ScoreType.Weighted}
		<InputWeightedCriteria bind:value={weightedRating} template={scoreTemplate.content} />
	{/if}
</div>
