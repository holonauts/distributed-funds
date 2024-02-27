<script lang="ts">
	import {
		ScoreType,
		type ScoreTemplate,
		type AttributeScore,
		type NumberRange,
		type Score
	} from '../../grant_pools/grants/types';
	import InputScoreWeighted from '$lib/components/InputScoreWeighted.svelte';
	import InputScoreSingle from './InputScoreSingle.svelte';

	export let scoreRange: NumberRange;
	export let scoreTemplate: ScoreTemplate;
	export let value: Score;

	let singleScore: number = 0;
	let attributeScores: AttributeScore[] = [];

	$: {
		value = {
			type: scoreTemplate.type,
			content: scoreTemplate.type === ScoreType.Single ? singleScore : attributeScores
		};
	}
</script>

<div class="w-full px-2">
	{#if scoreTemplate.type === ScoreType.Single}
		<InputScoreSingle bind:value={singleScore} {scoreRange} />
	{:else if scoreTemplate.type === ScoreType.Weighted}
		<InputScoreWeighted
			bind:value={attributeScores}
			{scoreRange}
			weightedCriteria={scoreTemplate.content}
		/>
	{/if}
</div>
