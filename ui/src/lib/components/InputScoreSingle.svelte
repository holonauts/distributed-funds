<script lang="ts">
	import { Input, Label, Range } from 'flowbite-svelte';
	import type { NumberRange } from '../../grant_pools/grants/types';

	export let scoreRange: NumberRange;
	export let value: number = scoreRange.min;
</script>

<Label for="rating">Rating</Label>
<div class="flex items-center justify-start space-x-2">
	<div class="flex-grow">
		<Range id="rating" size="sm" min={scoreRange.min} max={scoreRange.max} bind:value />
	</div>
	<Input size="sm" bind:value class="w-14" />
</div>
