<script lang="ts">
	import { Input, Label } from 'flowbite-svelte';
	import type { NumberRange } from '../../grant_pools/grants/types';

	let min: string;
	let max: string;
	export let value: NumberRange | undefined;

	$: {
		value = { min: parseInt(min), max: parseInt(max) };
	}
</script>

<div class="flex items-center justify-start space-x-4">
	<div>
		<Input type="number" id="min" bind:value={min} placeholder="0" />
		<Label for="min" class="text-xs">Minimum</Label>
	</div>
	<div>
		<Input type="number" id="max" bind:value={max} placeholder="100" />
		<Label for="max" class="text-xs">Maximum</Label>
	</div>
</div>
