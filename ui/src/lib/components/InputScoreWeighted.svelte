<script lang="ts">
	import { Badge, Label, Range } from 'flowbite-svelte';
	import type {
		AttributeScoreTemplate,
		AttributeScore,
		NumberRange
	} from '../../grant_pools/grants/types';
	import { onMount } from 'svelte';
	import InputScoreSingle from './InputScoreSingle.svelte';

	export let scoreRange: NumberRange;
	export let attributeScoreTemplate: AttributeScoreTemplate[] = [];
	export let value: AttributeScore[] = [];

	onMount(() => {
		value = attributeScoreTemplate.map((c) => ({
			label: c.label,
			value: scoreRange.min,
			weight: c.weight
		}));
	});
</script>

{#if value.length === attributeScoreTemplate.length}
	{#each attributeScoreTemplate as criteria, i}
		<InputScoreSingle {scoreRange} label={criteria.label} bind:value={value[i].value} />
	{/each}
{/if}
