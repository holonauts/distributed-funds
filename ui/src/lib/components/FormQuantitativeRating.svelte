<script lang="ts">
	import {
		QuantitativeRatingType,
		type QuantitativeRating,
		type QuantitativeRatingTemplate,
		type RatingCriteria
	} from '../../grant_pools/grants/types';
	import { Range, Label, Input } from 'flowbite-svelte';
	import InputWeightedCriteria from '$lib/components/InputWeightedCriteria.svelte';

	export let quantitativeRatingTemplate: QuantitativeRatingTemplate;
	export let value: QuantitativeRating | undefined = undefined;

	let singleRating: number = 0;
	let weightedRating: RatingCriteria[] = [];

	$: {
		value = {
			type: quantitativeRatingTemplate.type,
			content:
				quantitativeRatingTemplate.type === QuantitativeRatingType.Single
					? singleRating
					: weightedRating
		};
	}
</script>

<div class="w-full px-2">
	{#if quantitativeRatingTemplate.type === QuantitativeRatingType.Single}
		<Label for="rating">Rating</Label>
		<div class="flex items-center justify-start space-x-2">
			<div class="flex-grow">
				<Range
					id="rating"
					size="sm"
					min={quantitativeRatingTemplate.content.min}
					max={quantitativeRatingTemplate.content.max}
					bind:value={singleRating}
				/>
			</div>
			<Input size="sm" bind:value={singleRating} class="w-14" />
		</div>
	{:else if quantitativeRatingTemplate.type === QuantitativeRatingType.Weighted}
		<InputWeightedCriteria
			bind:value={weightedRating}
			template={quantitativeRatingTemplate.content}
		/>
	{/if}
</div>
