<script lang="ts">
	import BaseBreadcrumbs from '$lib/components/BaseBreadcrumbs.svelte';
	import BaseHeading from '$lib/components/BaseHeading.svelte';
	import BaseFormBuilder from '$lib/components/BaseFormBuilder.svelte';
	import RecordDetail from '$lib/components/RecordDetail.svelte';
	import { encodeHashToBase64, type ActionHash, type Record } from '@holochain/client';
	import BaseLabelContent from '$lib/components/BaseLabelContent.svelte';
	import InputTokenAmount from '$lib/components/InputTokenAmount.svelte';
	import { ACCEPTED_TOKEN_DECIMALS, ACCEPTED_TOKEN_SYMBOL } from '$lib/config';
	import { Button } from 'flowbite-svelte';
	import { bigintToU256, u256ToBigint } from '$lib/utils/u256';
	import { toasts } from '$lib/stores/toast';
	import { goto } from '$app/navigation';
	import { holochainClient } from '$lib/stores/holochainClient';
	import { StatusType, type Application } from '../../grant_pools/grants/types';
	import type { RenderAPI } from '@pragmatic-engineering/svelte-form-builder-community';

	export let amount: bigint | undefined = undefined;
	export let renderApi: typeof RenderAPI | undefined = undefined;
	export let actionHash: ActionHash;

	$: actionHashB64 = encodeHashToBase64(actionHash);
	$: isValid = amount !== undefined && amount > 0n;

	async function saveDraft() {
		const formContent = await renderApi?.getData();

		const applicationEntry: Application = {
			grant_pool: actionHash,
			form_content: formContent!,
			amount: bigintToU256(amount!),
			status: { type: StatusType.Draft, content: undefined }
		};
		console.log('applicationEntry', applicationEntry);

		try {
			const record: Record = await $holochainClient.client.callZome({
				cap_secret: null,
				role_name: 'grant_pools',
				zome_name: 'grants',
				fn_name: 'create_application',
				payload: applicationEntry
			});
			goto(
				`/grant-pools/${actionHashB64}/applications/${encodeHashToBase64(record.signed_action.hashed.hash)}`
			);
		} catch (e) {
			toasts.error(`Error saving the application: ${e}`);
		}
	}

	async function submit() {
		const formContent = await renderApi?.getData();
		const applicationEntry: Application = {
			grant_pool: actionHash,
			form_content: formContent!,
			amount: bigintToU256(amount!),
			status: { type: StatusType.Submitted, content: undefined }
		};
		console.log('applicationEntry', applicationEntry);

		try {
			//TODO this should be an update, if no prior exists, otherwise can be a create
			const record: Record = await $holochainClient.client.callZome({
				cap_secret: null,
				role_name: 'grant_pools',
				zome_name: 'grants',
				fn_name: 'create_application',
				payload: applicationEntry
			});
			goto(
				`/grant-pools/${actionHashB64}/applications/${encodeHashToBase64(record.signed_action.hashed.hash)}`
			);
		} catch (e) {
			toasts.error(`Error creating the application: ${e}`);
		}
	}
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
		<RecordDetail
			callZomeRequest={{
				cap_secret: null,
				role_name: 'grant_pools',
				zome_name: 'grants',
				fn_name: 'get_application_template',
				payload: entry.application_template
			}}
		>
			<svelte:fragment let:entry={applicationTemplate}>
				<BaseBreadcrumbs title="Apply" replacements={{ [actionHashB64]: entry.name }} />

				<BaseHeading class="mb-8"><slot /></BaseHeading>

				<div class="mb-8">
					<BaseFormBuilder
						view="render"
						bind:renderApi
						formDefinition={JSON.parse(applicationTemplate.form_schema).formDefinition}
					/>
				</div>

				<BaseLabelContent label="Funding Amount" class="mb-8">
					{amount}
					<InputTokenAmount
						decimals={ACCEPTED_TOKEN_DECIMALS}
						symbol={ACCEPTED_TOKEN_SYMBOL}
						maxValue={u256ToBigint(entry.amount_range.max)}
						bind:value={amount}
						showMaxButton={false}
					/>
				</BaseLabelContent>

				<div class="flex items-center justify-end space-x-8">
					<Button color="alternative" on:click={saveDraft}>Save Draft</Button>
					<Button color="green" on:click={submit} disabled={!isValid}>Submit</Button>
				</div>
			</svelte:fragment>
		</RecordDetail>
	</svelte:fragment>
</RecordDetail>
