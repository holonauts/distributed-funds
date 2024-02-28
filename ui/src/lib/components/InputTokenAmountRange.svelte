<script lang="ts">
	import type { AmountRangeBigInt } from '../../grant_pools/grants/types';
	import InputTokenAmount from './InputTokenAmount.svelte';
	import BaseHelper from './BaseHelper.svelte';

	export let value: AmountRangeBigInt = { min: 0n, max: 0n };
	export let decimals: number = 0;
	export let symbol: string;
</script>

<div class="mb-2 flex items-center justify-start space-x-4">
	<div class="flex flex-col space-y-1">
		<InputTokenAmount {symbol} {decimals} bind:value={value.min} placeholder="5000" />
		<BaseHelper>Minimum</BaseHelper>
	</div>
	<div class="flex flex-col space-y-1">
		<InputTokenAmount {symbol} {decimals} bind:value={value.max} placeholder="10000" />
		<BaseHelper>Maximum</BaseHelper>
	</div>
</div>
