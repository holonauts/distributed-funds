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
	import TimePeriodBadge from '$lib/components/TimePeriodBadge.svelte';
	import BaseHeading from '$lib/components/BaseHeading.svelte';

	$: actionHash = decodeHashFromBase64($page.params.actionHashB64);
</script>

<RecordDetail
	callZomeRequest={{
		cap_secret: null,
		role_name: 'grant_pools',
		zome_name: 'grants',
		fn_name: 'get_grant_pool',
		payload: actionHash
	}}
>
	<svelte:fragment let:entry>
		<BaseBreadcrumbs title={entry.name} />

		<div class="mb-6 flex items-start justify-between">
			<BaseHeading>{entry.name}</BaseHeading>

			<TimePeriodBadge timePeriodHash={entry.time_period} />
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
			<BaseTokenAmountRange amountRange={entry.amount_range} />
		</BaseLabelContent>

		<BaseLabelContent label="Funders" class="mb-8">
			<ProfilesInlineList agentPubKeys={[]} />
		</BaseLabelContent>

		<BaseLabelContent label="Evaluators" class="mb-8">
			<ProfilesInlineList agentPubKeys={entry.evaluators} />
		</BaseLabelContent>

		<div class="flex justify-end">
			<Button color="green" size="xl" href={`/`}>Apply Now</Button>
		</div>
	</svelte:fragment>
</RecordDetail>
