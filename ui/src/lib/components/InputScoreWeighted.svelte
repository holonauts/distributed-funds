<script lang="ts">
	import { Badge, Label, Range } from 'flowbite-svelte';
	import type {
		AttributeScoreTemplate,
		AttributeScore,
		NumberRange
	} from '../../grant_pools/grants/types';
	import { onMount } from 'svelte';

	export let scoreRange: NumberRange;
	export let attributeScoreTemplate: AttributeScoreTemplate[] = [];
	export let value: AttributeScore[] = [];

	onMount(() => {
		value = attributeScoreTemplate.map((c) => ({
			label: c.label,
			value: scoreRange.min
		}));
	});
</script>

{#if value.length === attributeScoreTemplate.length}
	{#each attributeScoreTemplate as criteria, i}
		<div class="mb-2">
			<Label for="rating">{criteria.label} <span>(Weight {criteria.weight})</span></Label>
			<div class="flex-grow">
				<Range
					id="rating"
					size="sm"
					min={scoreRange.min}
					max={scoreRange.max}
					bind:value={value[i].value}
				/>
				<div class="flex items-center justify-between">
					<Badge color="none">{scoreRange.min}</Badge>
					<Badge color="none">{scoreRange.max}</Badge>
				</div>
			</div>
		</div>
	{/each}
{/if}
