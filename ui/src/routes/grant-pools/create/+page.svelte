<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import type { Record, ActionHash, AgentPubKey } from '@holochain/client';
	import type { GrantPool, NumberRange } from '../../../grant_pools/grants/types';
	import { Label, Textarea, Button, Helper, Input } from 'flowbite-svelte';
	import { toasts } from '$lib/stores/toast';
	import InputApplicationTemplate from './InputApplicationTemplate.svelte';
	import InputEvaluationTemplate from './InputEvaluationTemplate.svelte';
	import { holochainClient } from '$lib/stores/holochainClient';
	import BaseBreadcrumbs from '$lib/components/BaseBreadcrumbs.svelte';
	import InputNumberRange from '$lib/components/InputNumberRange.svelte';
	import SelectTimePeriod from './SelectTimePeriod.svelte';

	const dispatch = createEventDispatcher();

	export let name: string = '';
	export let purposeDescription: string = '';
	export let rulesDescription: string = '';
	export let timePeriod: ActionHash | undefined = undefined;
	export let amountRange: NumberRange | undefined = undefined;
	export let applicationTemplate: ActionHash | undefined = undefined;
	export let evaluationTemplate: ActionHash | undefined = undefined;
	export let evaluators: AgentPubKey[] = [];

	$: name,
		purposeDescription,
		rulesDescription,
		timePeriod,
		applicationTemplate,
		evaluationTemplate;
	$: isGrantPoolValid = true && name !== '' && purposeDescription !== '' && rulesDescription !== '';

	async function createGrantPool() {
		const grantPoolEntry: GrantPool = {
			name: name!,
			purpose_description: purposeDescription!,
			rules_description: rulesDescription!,
			time_period: timePeriod!,
			application_template: applicationTemplate!,
			evaluation_template: evaluationTemplate!,
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
		} catch (e) {
			toasts.error(`Error creating the grant pool: ${e.data.data}`);
		}
	}
</script>

<BaseBreadcrumbs title="Create" />

<div class="flex h-full flex-col">
	<div class="mb-8">
		<Label for="name" class="mb-2">Title</Label>
		<Input id="name" bind:name required placeholder="Coral Reef Renewal" />
	</div>

	<div class="mb-8">
		<Label for="purpose-description" class="mb-2">Purpose</Label>
		<Textarea
			id="purpose-description"
			class="h-48"
			bind:purposeDescription
			required
			placeholder="To support projects that are effectively improving the health of coral reefs in the south pacific."
		/>
		<Helper>
			What is the mission of this grants pool? What should the grants be trying to accomplish?
		</Helper>
	</div>

	<div class="mb-8">
		<Label for="rules-description" class="mb-2">Rules & Eligability Criteria</Label>
		<Textarea
			id="rules-description"
			class="h-48"
			bind:rulesDescription
			required
			placeholder="All projects must be based in the east. Have no more than 500K revenue per year."
		/>
	</div>

	<div class="mb-8 w-full">
		<InputApplicationTemplate bind:value={applicationTemplate} />
	</div>

	<div class="mb-8">
		<InputEvaluationTemplate bind:value={evaluationTemplate} />
	</div>

	<div class="mb-8">
		<Label>Allowed Grant Amount (USDC)</Label>
		<InputNumberRange bind:value={amountRange} />
		<Helper class="mt-2">How much funding can be awarded in a single grant?</Helper>
	</div>

	<div class="mb-8">
		<SelectTimePeriod bind:value={amountRange} />
	</div>

	<div class="mb-8">
		<Label class="mb-2">Evaluators</Label>
		<!-- <InputEvaluators bind:evaluators /> -->
		<Helper>The people who will evaluate and score grant applications.</Helper>
	</div>

	<Button disabled={!isGrantPoolValid} on:click={createGrantPool}>Create</Button>
</div>
