<script lang="ts">
	import { page } from '$app/stores';
	import { decodeHashFromBase64 } from '@holochain/client';
	import type { Evaluation, Score } from '../../../../grant_pools/grants/types';
	import { type RenderAPI } from '@pragmatic-engineering/svelte-form-builder-community';
	import { holochainClient } from '$lib/stores/holochainClient';
	import { goto } from '$app/navigation';
	import { toasts } from '$lib/stores/toast';
	import RecordDetail from '$lib/components/RecordDetail.svelte';
	import BaseFormBuilder from '$lib/components/BaseFormBuilder.svelte';
	import BaseHeading from '$lib/components/BaseHeading.svelte';
	import { Button, Textarea, Card } from 'flowbite-svelte';
	import BaseBreadcrumbs from '$lib/components/BaseBreadcrumbs.svelte';
	import BaseLabelContent from '$lib/components/BaseLabelContent.svelte';
	import ProfileInline from '$lib/components/ProfileInline.svelte';
	import { formatUnits } from 'viem';
	import { u256ToBigint } from '$lib/utils/u256';
	import { ACCEPTED_TOKEN_DECIMALS } from '$lib/config';
	import InputScore from '$lib/components/InputScore.svelte';

	export let renderApi: typeof RenderAPI | undefined = undefined;
	export let comments: string = '';
	export let score: Score;

	$: actionHash = decodeHashFromBase64($page.params.actionHashB64);
	$: isValid = score !== undefined;

	async function submit() {
		const formContent = await renderApi?.getData();
		const evaluationEntry: Evaluation = {
			application: actionHash!,
			form_content: formContent!,
			comments: comments!,
			score: score!
		};
		console.log('evaluation is ', evaluationEntry);

		try {
			await $holochainClient.client.callZome({
				cap_secret: null,
				role_name: 'grant_pools',
				zome_name: 'grants',
				fn_name: 'create_evaluation',
				payload: evaluationEntry
			});
			goto(`/applications/${$page.params.actionHashB64}`);
		} catch (e) {
			toasts.error(`Error creating the evaluation: ${e}`);
		}
	}
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
				<RecordDetail
					callZomeRequest={{
						cap_secret: null,
						role_name: 'grant_pools',
						zome_name: 'grants',
						fn_name: 'get_evaluation_template',
						payload: grantPool.evaluation_template
					}}
				>
					<svelte:fragment let:entry={evaluationTemplate}>
						<BaseBreadcrumbs title="Evaluation" disabled={['evaluations']} />

						<div class="mb-3 flex items-start justify-between">
							<BaseHeading>Application</BaseHeading>
						</div>

						<Card size="xl" class="mb-12">
							<BaseLabelContent label="Grant Pool" class="mb-8">
								{grantPool.name}
							</BaseLabelContent>

							<BaseLabelContent label="Applicant" class="mb-8">
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
						</Card>

						<BaseHeading class="mb-3">Evaluation</BaseHeading>
						<div class="mb-8">
							<BaseFormBuilder
								view="render"
								bind:renderApi
								formDefinition={JSON.parse(evaluationTemplate.form_schema).formDefinition}
							/>
						</div>

						<BaseLabelContent label="Comments" class="mb-8">
							<Textarea bind:value={comments} rows="10" />
						</BaseLabelContent>

						<div class="mb-8">
							<InputScore
								bind:value={score}
								scoreRange={evaluationTemplate.score_range}
								scoreTemplate={evaluationTemplate.score}
							/>
						</div>

						<div class="flex items-center justify-end">
							<Button color="green" on:click={submit} disabled={!isValid}>Submit</Button>
						</div>
					</svelte:fragment>
				</RecordDetail>
			</svelte:fragment>
		</RecordDetail>
	</svelte:fragment>
</RecordDetail>
