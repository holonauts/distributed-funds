<script lang="ts">
	import { Label } from 'flowbite-svelte';
	import type { AmountRangeBigInt } from '../../grant_pools/grants/types';
	import InputTokenAmount from './InputTokenAmount.svelte';

	export let value: AmountRangeBigInt = { min: 0n, max: 0n };
	export let decimals: number = 0;
</script>

<div class="flex items-center justify-start space-x-4">
	<div>
		<InputTokenAmount {decimals} bind:value={value.min} />
		<Label for="min" class="text-xs">Minimum</Label>
	</div>
	<div>
		<InputTokenAmount {decimals} bind:value={value.min} />
		<Label for="max" class="text-xs">Maximum</Label>
	</div>
</div>
