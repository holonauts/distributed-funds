<script lang="ts">
	import { isEqual } from 'lodash';
	import { u256ToBigint } from '$lib/utils/u256';
	import { ACCEPTED_TOKEN_SYMBOL } from '../../config';
	import type { AmountRange } from '../../grant_pools/grants/types';

	export let amountRange: AmountRange;
</script>

{#if isEqual(amountRange.min, amountRange.max)}
	{u256ToBigint(amountRange.min)} {ACCEPTED_TOKEN_SYMBOL}
{:else}
	{u256ToBigint(amountRange.min)} - {u256ToBigint(amountRange.max)}
	{ACCEPTED_TOKEN_SYMBOL}
{/if}
