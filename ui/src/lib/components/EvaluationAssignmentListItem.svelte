<script lang="ts">
	import { encodeHashToBase64, type ActionHash } from '@holochain/client';
	import RecordListItem from '$lib/components/RecordDetail.svelte';
	import BaseLabelContent from '$lib/components/BaseLabelContent.svelte';
	import { Badge, Card } from 'flowbite-svelte';
	import { goto } from '$app/navigation';
	import { ACCEPTED_TOKEN_DECIMALS, ACCEPTED_TOKEN_SYMBOL } from '$lib/config';
	import { formatUnits } from 'viem';
	import { u256ToBigint } from '$lib/utils/u256';
	import ProfileInline from '$lib/components/ProfileInline.svelte';
	import { holochainClient } from '$lib/stores/holochainClient';
	import RecordList from './RecordList.svelte';

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
			on:click={() => goto(`/applications/${encodeHashToBase64(applicationHash)}/evaluate`)}
		>
			<div class="flex items-start justify-between">
				<div class="mb-4 flex items-start justify-between">
					<ProfileInline agentPubKey={record.signed_action.hashed.content.author} />
				</div>

				<RecordList
					displayNoneMessage={false}
					entryType="Evaluation"
					callZomeRequest={{
						cap_secret: null,
						role_name: 'grant_pools',
						zome_name: 'grants',
						fn_name: 'get_evaluations_for_application_by_agent',
						payload: {
							application_hash: applicationHash,
							agent: $holochainClient.client.myPubKey
						}
					}}
				>
					<svelte:fragment let:hash>
						<Badge color="green">Evaluated</Badge>
					</svelte:fragment>
				</RecordList>
			</div>

			<BaseLabelContent label="Requested Amount">
				{formatUnits(u256ToBigint(entry.amount), ACCEPTED_TOKEN_DECIMALS)}
				{ACCEPTED_TOKEN_SYMBOL}
			</BaseLabelContent>
		</Card>
	</svelte:fragment>
</RecordListItem>
