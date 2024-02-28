<script lang="ts">
	import BaseBreadcrumbs from '$lib/components/BaseBreadcrumbs.svelte';
	import BaseHeading from '$lib/components/BaseHeading.svelte';
	import BaseFormBuilder from '$lib/components/BaseFormBuilder.svelte';
	import RecordDetail from '$lib/components/RecordDetail.svelte';
	import { decodeHashFromBase64 } from '@holochain/client';
	import BaseLabelContent from '$lib/components/BaseLabelContent.svelte';
	import InputTokenAmount from '$lib/components/InputTokenAmount.svelte';
	import { ACCEPTED_TOKEN_DECIMALS, ACCEPTED_TOKEN_SYMBOL } from '../../../../config';
	import { page } from '$app/stores';
	import { Button } from 'flowbite-svelte';

	let amount: bigint;

	$: actionHash = decodeHashFromBase64($page.params.actionHashB64);

	async function saveDraft() {}

	async function submit() {}
</script>

<BaseBreadcrumbs title="Apply" />

<RecordDetail
	callZomeRequest={{
		cap_secret: null,
		role_name: 'grant_pools',
		zome_name: 'grants',
		fn_name: 'get_grant_pool',
		payload: actionHash
	}}
>
	pub application_template: ActionHash, pub form_content: String, pub amount: U256, pub status:
	ApplicationStatus,

	<svelte:fragment let:entry>
		<BaseHeading>{entry.name}</BaseHeading>

		<BaseFormBuilder view="render" formDefinition={JSON.parse(entry.form_content).formDefinition} />

		<BaseLabelContent label="Funding Amount">
			<InputTokenAmount
				decimals={ACCEPTED_TOKEN_DECIMALS}
				symbol={ACCEPTED_TOKEN_SYMBOL}
				bind:value={amount}
			/>
		</BaseLabelContent>

		<div class="flex items-center justify-end">
			<Button color="alternative" on:click={saveDraft}>Save Draft</Button>
			<Button color="green" on:click={submit}>Submit</Button>
		</div>
	</svelte:fragment>
</RecordDetail>
