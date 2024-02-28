<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import type { Record, ActionHash, AgentPubKey } from '@holochain/client';
	import type { AmountRangeBigInt, GrantPool } from '../../../grant_pools/grants/types';
	import { Label, Textarea, Button, Helper, Input } from 'flowbite-svelte';
	import { toasts } from '$lib/stores/toast';
	import InputApplicationTemplate from './InputApplicationTemplate.svelte';
	import InputEvaluationTemplate from './InputEvaluationTemplate.svelte';
	import { holochainClient } from '$lib/stores/holochainClient';
	import BaseBreadcrumbs from '$lib/components/BaseBreadcrumbs.svelte';
	import SelectTimePeriod from './InputTimePeriod.svelte';
	import InputAgents from '$lib/components/InputAgents.svelte';
	import InputTokenAmountRange from '$lib/components/InputTokenAmountRange.svelte';
	import { ACCEPTED_TOKEN_DECIMALS, ACCEPTED_TOKEN_SYMBOL } from '../../../config';
	import { bigintToU256 } from '$lib/utils/u256';
	import { goto } from '$app/navigation';
	import BaseLabelContent from '$lib/components/BaseLabelContent.svelte';
	import BaseHelper from '$lib/components/BaseHelper.svelte';

	const dispatch = createEventDispatcher();

	export let name: string = '';
	export let purposeDescription: string = '';
	export let rulesDescription: string = '';
	export let timePeriod: ActionHash | undefined = undefined;
	export let amountRange: AmountRangeBigInt | undefined = undefined;
	export let applicationTemplate: ActionHash | undefined = undefined;
	export let evaluationTemplate: ActionHash | undefined = undefined;
	export let evaluators: AgentPubKey[] = [];

	$: isGrantPoolValid =
		name !== '' &&
		purposeDescription !== '' &&
		rulesDescription !== '' &&
		timePeriod !== undefined &&
		amountRange !== undefined &&
		applicationTemplate !== undefined &&
		evaluationTemplate !== undefined &&
		evaluators.length > 0;

	async function createGrantPool() {
		const grantPoolEntry: GrantPool = {
			name: name!,
			purpose_description: purposeDescription!,
			rules_description: rulesDescription!,
			time_period: timePeriod!,
			application_template: applicationTemplate!,
			evaluation_template: evaluationTemplate!,
			amount_range: {
				min: bigintToU256(amountRange!.min),
				max: bigintToU256(amountRange!.max)
			},
			evaluators: evaluators!
		};

		try {
			const record: Record = await $holochainClient.client.callZome({
				cap_secret: null,
				role_name: 'grant_pools',
				zome_name: 'grants',
				fn_name: 'create_grant_pool',
				payload: grantPoolEntry
			});
			dispatch('grant-pool-created', { grantPoolHash: record.signed_action.hashed.hash });
			goto('/grant-pools/');
		} catch (e) {
			toasts.error(`Error creating the grant pool: ${e}`);
		}
	}
</script>

<BaseBreadcrumbs title="Create" />

<BaseLabelContent label="Title" class="mb-8">
	<Input id="name" bind:value={name} required placeholder="Coral Reef Renewal" />
</BaseLabelContent>

<BaseLabelContent label="Purpose" class="mb-8">
	<Textarea
		id="purpose-description"
		class="h-48"
		bind:value={purposeDescription}
		required
		placeholder="To support projects that are effectively improving the health of coral reefs in the south pacific."
	/>
	<BaseHelper>The mission of this grants pool, and what it aims to accomplish.</BaseHelper>
</BaseLabelContent>

<BaseLabelContent label="Rules & Eligability Criteria" class="mb-8">
	<Textarea
		id="rules-description"
		class="h-48"
		bind:value={rulesDescription}
		required
		placeholder="All projects must be based in the east. Have no more than 500K revenue per year."
	/>
	<BaseHelper>The rules and eligability requirements for grant applications.</BaseHelper>
</BaseLabelContent>

<div class="mb-8">
	<InputApplicationTemplate bind:value={applicationTemplate} />
</div>

<div class="mb-8">
	<InputEvaluationTemplate bind:value={evaluationTemplate} />
</div>

<BaseLabelContent label="Grant Funding Range" class="mb-8">
	<InputTokenAmountRange
		symbol={ACCEPTED_TOKEN_SYMBOL}
		decimals={ACCEPTED_TOKEN_DECIMALS}
		bind:value={amountRange}
	/>
	<BaseHelper>The amount of funding that may be awarded in a single grant.</BaseHelper>
</BaseLabelContent>

<div class="mb-8">
	<SelectTimePeriod bind:value={timePeriod} />
</div>

<div class="mb-8">
	<BaseLabelContent label="Evaluators">
		<InputAgents bind:value={evaluators} />
	</BaseLabelContent>
	<BaseHelper>The people invited to evaluate and score grant applications.</BaseHelper>
</div>

<div class="flex justify-end">
	<Button disabled={!isGrantPoolValid} on:click={createGrantPool} color="green">Create</Button>
</div>
