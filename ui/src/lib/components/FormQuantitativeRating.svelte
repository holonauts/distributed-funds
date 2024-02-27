<script lang="ts">
	import {
		QuantitativeRatingType,
		type QuantitativeRating,
		type QuantitativeRatingTemplate,
		type RatingCriteria
	} from '../../grant_pools/grants/types';
	import { Range, Label } from 'flowbite-svelte';

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

{#if quantitativeRatingTemplate.type === QuantitativeRatingType.Single}
	<Label>Rating</Label>
	<Range id="small-range" size="sm" bind:value={singleRating} />
{:else if quantitativeRatingTemplate.type === QuantitativeRatingType.Weighted}
	{#each quantitativeRatingTemplate.content as criteria, i}
		<div class="mb-4">
			<Label for={`criteria-${i}`}>{criteria.label}</Label>
			<Range id={`criteria-${i}`} size="sm" bind:value={weightedRating[i].value} />
		</div>
	{/each}
{/if}
