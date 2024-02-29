<script lang="ts">
	import { Input, Range } from 'flowbite-svelte';
	import type { NumberRange } from '../../grant_pools/grants/types';
	import BaseLabelContent from './BaseLabelContent.svelte';

	export let scoreRange: NumberRange;
	export let value: number = scoreRange.min;
	export let label = 'Score';
</script>

<BaseLabelContent {label}>
	<div class="flex items-center justify-start space-x-2">
		<div class="flex-grow">
			<Range id="rating" size="sm" min={scoreRange.min} max={scoreRange.max} bind:value />
		</div>
		<Input size="sm" bind:value class="w-14" />
	</div>
</BaseLabelContent>
