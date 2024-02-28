<script lang="ts">
	import { isEqual } from 'lodash';
	import { u256ToBigint } from '$lib/utils/u256';
	import type { AmountRange } from '../../grant_pools/grants/types';
	import { formatUnits } from 'viem';

	export let amountRange: AmountRange;
	export let symbol: string;
	export let decimals: number;
</script>

{#if isEqual(amountRange.min, amountRange.max)}
	{formatUnits(u256ToBigint(amountRange.min), decimals)} {symbol}
{:else}
	{formatUnits(u256ToBigint(amountRange.min), decimals)} -
	{formatUnits(u256ToBigint(amountRange.max), decimals)}
	{symbol}
{/if}
