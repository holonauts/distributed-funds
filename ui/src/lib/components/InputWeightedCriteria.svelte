<script lang="ts">
	import { Badge, Label, Range } from 'flowbite-svelte';
	import type { NumberRangeWeightedCriteria, RatingCriteria } from '../../grant_pools/grants/types';
	import { onMount } from 'svelte';

	export let template: NumberRangeWeightedCriteria;
	export let value: RatingCriteria[] = [];

	onMount(() => {
		value = template.weighted_criteria.map((c) => ({
			label: c.label,
			value: template.range.min
		}));
	});
</script>

{#if value.length === template.weighted_criteria.length}
	{#each template.weighted_criteria as criteria, i}
		<div class="mb-2">
			<Label for="rating">{criteria.label} <span>(Weight {criteria.weight})</span></Label>
			<div class="flex-grow">
				<Range
					id="rating"
					size="sm"
					min={template.range.min}
					max={template.range.max}
					bind:value={value[i].value}
				/>
				<div class="flex items-center justify-between">
					<Badge color="none">{template.range.min}</Badge>
					<Badge color="none">{template.range.max}</Badge>
				</div>
			</div>
		</div>
	{/each}
{/if}
