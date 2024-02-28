<script lang="ts">
	import { encodeHashToBase64, type ActionHash } from '@holochain/client';
	import RecordListItem from '$lib/components/RecordDetail.svelte';
	import BaseLabelContent from '$lib/components/BaseLabelContent.svelte';
	import { Card, Label } from 'flowbite-svelte';
	import TimePeriodListItem from '$lib/components/TimePeriodListItem.svelte';
	import { goto } from '$app/navigation';
	import BaseTokenAmountRange from './BaseTokenAmountRange.svelte';

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
			on:click={() => goto(`grant-pools/${encodeHashToBase64(record.signed_action.hashed.hash)}`)}
		>
			<div class="mb-4 text-2xl">{entry.name}</div>

			<BaseLabelContent label="Application Period">
				<TimePeriodListItem timePeriodHash={entry.timePeriod} />
			</BaseLabelContent>

			<BaseLabelContent label="Grant Amounts">
				<BaseTokenAmountRange amountRange={entry.amount_range} />
			</BaseLabelContent>
		</Card>
	</svelte:fragment>
</RecordListItem>
