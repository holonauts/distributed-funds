<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import type { Record } from '@holochain/client';
	import {
		QuantitativeRatingType,
		type EvaluationTemplate,
		type QuantitativeRatingTemplate
	} from '../../../grant_pools/grants/types';
	import { Button, Label, Input } from 'flowbite-svelte';
	import { toasts } from '$lib/stores/toast';
	import BaseFormBuilder from '$lib/components/BaseFormBuilder.svelte';
	import { holochainClient } from '$lib/stores/holochainClient';
	import { type BuilderAPI } from '@pragmatic-engineering/svelte-form-builder-community';
	import InputQuantitativeRatingTemplate from './InputQuantitativeRatingTemplate.svelte';

	const dispatch = createEventDispatcher();

	export let qualitativeJsonSchema: string = '';
	export let name: string = '';
	export let quantitativeRating: QuantitativeRatingTemplate = {
		type: QuantitativeRatingType.Single,
		content: { min: 0, max: 10 }
	};

	let builderApi: typeof BuilderAPI | undefined = undefined;

	$: isEvaluationTemplateValid = name.length > 0;

	async function createEvaluationTemplate() {
		if (!builderApi) return;
		qualitativeJsonSchema = await builderApi.getDefinitionData();

		const evaluationTemplateEntry: EvaluationTemplate = {
			name,
			qualitative_json_schema: qualitativeJsonSchema,
			quantitative_rating: quantitativeRating
		};

		try {
			const record: Record = await $holochainClient.client.callZome({
				cap_secret: null,
				role_name: 'grant_pools',
				zome_name: 'grants',
				fn_name: 'create_evaluation_template',
				payload: evaluationTemplateEntry
			});
			dispatch('evaluation-template-created', {
				evaluationTemplateHash: record.signed_action.hashed.hash
			});
		} catch (e) {
			toasts.error(`Error creating the evaluation template: ${e}`);
		}
	}
</script>

<div class="flex flex-col">
	<div class="mb-8">
		<Label for="name">Name</Label>
		<Input id="name" bind:value={name} />
	</div>

	<div class="mb-8">
		<Label>Qualitative Evaluation Template</Label>
		<BaseFormBuilder bind:builderApi />
	</div>

	<div class="mb-8">
		<Label class="mb-2">Quantitative Rating Template</Label>
		<InputQuantitativeRatingTemplate bind:value={quantitativeRating} />
	</div>

	<Button on:click={createEvaluationTemplate} disabled={!isEvaluationTemplateValid}>
		Create Evaluation Template
	</Button>
</div>
