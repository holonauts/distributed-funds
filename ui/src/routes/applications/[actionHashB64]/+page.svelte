<script lang="ts">
	import { encodeHashToBase64, decodeHashFromBase64 } from '@holochain/client';
	import RecordDetail from '$lib/components/RecordDetail.svelte';
	import BaseStatusBadge from '$lib/components/BaseStatusBadge.svelte';
	import { StatusType } from '../../../grant_pools/grants/types';
	import CreateApplication from '$lib/components/CreateApplication.svelte';
	import BaseLabelContent from '$lib/components/BaseLabelContent.svelte';
	import { u256ToBigint } from '$lib/utils/u256';
	import { ACCEPTED_TOKEN_DECIMALS } from '$lib/config';
	import { formatUnits } from 'viem';
	import BaseBadgeRecordTimestamp from '$lib/components/BaseBadgeRecordTimestamp.svelte';
	import BaseHeading from '$lib/components/BaseHeading.svelte';
	import BaseBreadcrumbs from '$lib/components/BaseBreadcrumbs.svelte';
	import ProfileInline from '$lib/components/ProfileInline.svelte';
	import { page } from '$app/stores';
	import RecordList from '$lib/components/RecordList.svelte';
	import EvaluationListItem from '$lib/components/EvaluationListItem.svelte';

	$: actionHash = decodeHashFromBase64($page.params.actionHashB64);
</script>

<RecordDetail
	callZomeRequest={{
		cap_secret: null,
		role_name: 'grant_pools',
		zome_name: 'grants',
		fn_name: 'get_latest_application',
		payload: actionHash
	}}
>
	<svelte:fragment let:entry let:record>
		<RecordDetail
			callZomeRequest={{
				cap_secret: null,
				role_name: 'grant_pools',
				zome_name: 'grants',
				fn_name: 'get_grant_pool',
				payload: entry.grant_pool
			}}
		>
			<svelte:fragment let:entry={grantPool}>
				{#if entry.status.type === StatusType.Draft}
					<CreateApplication amount={u256ToBigint(entry.amount)} {actionHash}>
						<div class="mb-6 flex items-start justify-between">
							<BaseHeading>Application for {grantPool.name}</BaseHeading>
							<div class="flex flex-col items-end justify-start">
								<div>
									<BaseStatusBadge status={entry.status} />
								</div>
								<div>
									<BaseBadgeRecordTimestamp {record} />
								</div>
							</div>
						</div>
					</CreateApplication>
				{:else}
					<BaseBreadcrumbs
						title="Application"
						replacements={{ [encodeHashToBase64(entry.grant_pool)]: grantPool.name }}
						disabled={['applications']}
					/>

					<div class="mb-6 flex items-start justify-between">
						<BaseHeading>Application for {grantPool.name}</BaseHeading>
						<div class="flex flex-col items-end justify-start">
							<div>
								<BaseStatusBadge status={entry.status} />
							</div>
							<div>
								<BaseBadgeRecordTimestamp {record} />
							</div>
						</div>
					</div>

					<BaseLabelContent label="Applicant">
						<ProfileInline agentPubKey={record.signed_action.hashed.hash} />
					</BaseLabelContent>
					<div class="mb-8">
						{#each JSON.parse(entry.form_content) as field}
							<BaseLabelContent label={field.name} class="mb-8">
								{field.value}
							</BaseLabelContent>
						{/each}
					</div>
					<BaseLabelContent label="Funding Amount" class="mb-8">
						{formatUnits(u256ToBigint(entry.amount), ACCEPTED_TOKEN_DECIMALS)}
					</BaseLabelContent>
				{/if}

				<BaseHeading class="mb-4">Evaluations</BaseHeading>
				<RecordList
					entryType="Evaluation"
					callZomeRequest={{
						cap_secret: null,
						role_name: 'grant_pools',
						zome_name: 'grants',
						fn_name: 'get_evaluations_for_application',
						payload: actionHash
					}}
				>
					<svelte:fragment let:hash>
						<EvaluationListItem evaluationHash={hash} />
					</svelte:fragment>
				</RecordList>
			</svelte:fragment>
		</RecordDetail>
	</svelte:fragment>
</RecordDetail>
