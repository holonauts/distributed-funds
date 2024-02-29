<script lang="ts">
	import { encodeHashToBase64, type ActionHash } from '@holochain/client';
	import RecordListItem from '$lib/components/RecordDetail.svelte';
	import BaseLabelContent from '$lib/components/BaseLabelContent.svelte';
	import { Card } from 'flowbite-svelte';
	import { goto } from '$app/navigation';
	import { ACCEPTED_TOKEN_DECIMALS, ACCEPTED_TOKEN_SYMBOL } from '$lib/config';
	import BaseBadgeRecordTimestamp from '$lib/components/BaseBadgeRecordTimestamp.svelte';
	import { formatUnits } from 'viem';
	import { u256ToBigint } from '$lib/utils/u256';
	import ProfileInline from '$lib/components/ProfileInline.svelte';
	import BaseStatusBadge from '$lib/components/BaseStatusBadge.svelte';

	export let applicationHash: ActionHash;
</script>

<RecordListItem
	callZomeRequest={{
		cap_secret: null,
		role_name: 'grant_pools',
		zome_name: 'grants',
		fn_name: 'get_latest_application',
		payload: applicationHash
	}}
>
	<svelte:fragment let:record let:entry>
		<Card
			size="xl"
			on:click={() => goto(`/grant-pools/applications/${encodeHashToBase64(applicationHash)}`)}
		>
			<div class="flex items-start justify-between">
				<ProfileInline agentPubKey={record.signed_action.hashed.content.author} />

				<div class="flex flex-col items-end justify-start">
					<div>
						<BaseStatusBadge status={entry.status} />
					</div>
					<div>
						<BaseBadgeRecordTimestamp {record} />
					</div>
				</div>
			</div>

			<BaseLabelContent label="Funding Amount">
				{formatUnits(u256ToBigint(entry.amount), ACCEPTED_TOKEN_DECIMALS)}
			</BaseLabelContent>
		</Card>
	</svelte:fragment>
</RecordListItem>
