<script lang="ts">
	import { encodeHashToBase64, type ActionHash } from '@holochain/client';
	import RecordListItem from '$lib/components/RecordDetail.svelte';
	import BaseLabelContent from '$lib/components/BaseLabelContent.svelte';
	import { Card } from 'flowbite-svelte';
	import TimePeriodListItem from '$lib/components/TimePeriodListItem.svelte';
	import { goto } from '$app/navigation';
	import BaseTokenAmountRange from '../../lib/components/BaseTokenAmountRange.svelte';
	import TimePeriodRecordBadge from '$lib/components/TimePeriodRecordBadge.svelte';
	import { ACCEPTED_TOKEN_DECIMALS, ACCEPTED_TOKEN_SYMBOL } from '$lib/config';

	export let grantPoolHash: ActionHash;
</script>

<RecordListItem
	callZomeRequest={{
		cap_secret: null,
		role_name: 'grant_pools',
		zome_name: 'grants',
		fn_name: 'get_grant_pool',
		payload: grantPoolHash
	}}
>
	<svelte:fragment let:record let:entry>
		<Card
			size="xl"
			on:click={() => goto(`/grant-pools/${encodeHashToBase64(record.signed_action.hashed.hash)}`)}
		>
			<div class="flex items-start justify-between">
				<div class="mb-4 text-xl">{entry.name}</div>
				<TimePeriodRecordBadge timePeriodHash={entry.time_period} />
			</div>

			<BaseLabelContent label="Application Period">
				<TimePeriodListItem timePeriodHash={entry.time_period} />
			</BaseLabelContent>

			<BaseLabelContent label="Grant Amount">
				<BaseTokenAmountRange
					amountRange={entry.amount_range}
					decimals={ACCEPTED_TOKEN_DECIMALS}
					symbol={ACCEPTED_TOKEN_SYMBOL}
				/>
			</BaseLabelContent>
		</Card>
	</svelte:fragment>
</RecordListItem>
