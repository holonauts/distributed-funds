<script lang="ts"></script>

<script lang="ts">
	import { decodeHashFromBase64 } from '@holochain/client';
	import RecordDetail from '$lib/components/RecordDetail.svelte';
	import { page } from '$app/stores';
	import BaseLabelContent from '$lib/components/BaseLabelContent.svelte';
	import { Button } from 'flowbite-svelte';
	import TimePeriodListItem from '$lib/components/TimePeriodListItem.svelte';
	import BaseBreadcrumbs from '$lib/components/BaseBreadcrumbs.svelte';
	import BaseTokenAmountRange from '../BaseTokenAmountRange.svelte';
	import ProfilesInlineList from '$lib/components/ProfilesInlineList.svelte';
	import BaseTimePeriodBadge from '$lib/components/BaseTimePeriodBadge.svelte';
	import BaseHeading from '$lib/components/BaseHeading.svelte';
	import dayjs from 'dayjs';
	import { ACCEPTED_TOKEN_DECIMALS, ACCEPTED_TOKEN_SYMBOL } from '../../../config';

	let now = dayjs().valueOf();

	$: actionHash = decodeHashFromBase64($page.params.actionHashB64);
</script>

<RecordDetail
	callZomeRequest={{
		cap_secret: null,
		role_name: 'grant_pools',
		zome_name: 'grants',
		fn_name: 'get_application',
		payload: actionHash
	}}
>
	<svelte:fragment let:entry>
		<RecordDetail
			callZomeRequest={{
				cap_secret: null,
				role_name: 'grant_pools',
				zome_name: 'grants',
				fn_name: 'get_grant_pool',
				payload: entry.grant_pool
			}}
		>
			<svelte:fragment let:entry={grantPool} >
				<BaseBreadcrumbs title="Application" replacements={{[$page.params.actionHashB64]: grantPool.name}}/>

				<div class="mb-6 flex items-start justify-between">
					<BaseHeading>{entry.name}</BaseHeading>
					<BaseTimePeriodBadge {timePeriod} />
				</div>

				<BaseLabelContent label="Purpose" class="mb-8">
					{entry.purpose_description}
				</BaseLabelContent>

				<BaseLabelContent label="Rules" class="mb-8">
					{entry.rules_description}
				</BaseLabelContent>

				<BaseLabelContent label="Application Period" class="mb-8">
					<TimePeriodListItem timePeriodHash={entry.time_period} />
				</BaseLabelContent>

				<BaseLabelContent label="Grant Amount" class="mb-8">
					<BaseTokenAmountRange
						amountRange={entry.amount_range}
						decimals={ACCEPTED_TOKEN_DECIMALS}
						symbol={ACCEPTED_TOKEN_SYMBOL}
					/>
				</BaseLabelContent>

				<BaseLabelContent label="Funders" class="mb-8">
					<ProfilesInlineList agentPubKeys={[]} />
				</BaseLabelContent>

				<BaseLabelContent label="Evaluators" class="mb-8">
					<ProfilesInlineList agentPubKeys={entry.evaluators} />
				</BaseLabelContent>

				{#if now > timePeriod.start_at && now < timePeriod.end_at}
					<div class="flex justify-end">
						<Button
							color="green"
							size="xl"
							href={`/grant-pools/${$page.params.actionHashB64}/apply`}>Apply Now</Button
						>
					</div>
				{/if}
			</svelte:fragment>
		</RecordDetail>
	</svelte:fragment>
</RecordDetail>
