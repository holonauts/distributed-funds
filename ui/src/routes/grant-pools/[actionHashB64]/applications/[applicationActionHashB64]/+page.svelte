<script lang="ts">
	import { decodeHashFromBase64 } from '@holochain/client';
	import RecordDetail from '$lib/components/RecordDetail.svelte';
	import { page } from '$app/stores';
	import BaseBreadcrumbs from '$lib/components/BaseBreadcrumbs.svelte';
	import BaseStatusBadge from '$lib/components/BaseStatusBadge.svelte';
	import { StatusType } from '../../../../../grant_pools/grants/types';
	import CreateApplication from '$lib/components/CreateApplication.svelte';
	import BaseLabelContent from '$lib/components/BaseLabelContent.svelte';
	import { u256ToBigint } from '$lib/utils/u256';
	import { ACCEPTED_TOKEN_DECIMALS } from '$lib/config';
	import { formatUnits } from 'viem';

	$: actionHash = decodeHashFromBase64($page.params.actionHashB64);
	$: applicationActionHashB64 = decodeHashFromBase64($page.params.applicationActionHashB64);
</script>

<RecordDetail
	callZomeRequest={{
		cap_secret: null,
		role_name: 'grant_pools',
		zome_name: 'grants',
		fn_name: 'get_latest_application',
		payload: applicationActionHashB64
	}}
>
	<svelte:fragment let:entry>
		<RecordDetail
			callZomeRequest={{
				cap_secret: null,
				role_name: 'grant_pools',
				zome_name: 'grants',
				fn_name: 'get_grant_pool',
				payload: actionHash
			}}
		>
			<svelte:fragment let:entry={grantPool}>
				<BaseBreadcrumbs
					title="Application"
					replacements={{ [$page.params.actionHashB64]: grantPool.name }}
				/>

				<div class="mb-6 flex items-start justify-end">
					<BaseStatusBadge status={entry.status} />
				</div>

				{#if entry.status === StatusType.Draft}
					<CreateApplication {actionHash} />
				{:else}
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
			</svelte:fragment>
		</RecordDetail>
	</svelte:fragment>
</RecordDetail>
