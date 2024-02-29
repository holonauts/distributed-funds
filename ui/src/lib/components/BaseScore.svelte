<script lang="ts">
	import { ScoreType, type Score } from '../../grant_pools/grants/types';
	import BaseLabelContent from '$lib/components/BaseLabelContent.svelte';

	export let score: Score;
</script>

{#if score.type === ScoreType.Single}
	<BaseLabelContent label="Score">
		{score.content}
	</BaseLabelContent>
{:else if score.type === ScoreType.Weighted}
	{#each score.content as attributeScore}
		<BaseLabelContent
			label={`${attributeScore.label} ${attributeScore.weight > 1 ? `(x${attributeScore.weight})` : ''}`}
		>
			{attributeScore.value}
		</BaseLabelContent>
	{/each}
{/if}
