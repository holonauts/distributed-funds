<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import type { Record } from '@holochain/client';
	import {
		ScoreType,
		type EvaluationTemplate,
		type ScoreTemplate,
		type NumberRange
	} from '../../../grant_pools/grants/types';
	import { Button, Label, Input } from 'flowbite-svelte';
	import { toasts } from '$lib/stores/toast';
	import BaseFormBuilder from '$lib/components/BaseFormBuilder.svelte';
	import { holochainClient } from '$lib/stores/holochainClient';
	import { type BuilderAPI } from '@pragmatic-engineering/svelte-form-builder-community';
	import InputScoreTemplate from './InputScoreTemplate.svelte';
	import BaseLabelContent from '$lib/components/BaseLabelContent.svelte';

	const dispatch = createEventDispatcher();

	export let name: string = '';
	export let formSchema: string = '';
	export let scoreRange: NumberRange = { min: 0, max: 10 };
	export let score: ScoreTemplate = {
		type: ScoreType.Single,
		content: undefined
	};

	let builderApi: typeof BuilderAPI | undefined = undefined;

	$: isEvaluationTemplateValid = name.length > 0 && score !== undefined && scoreRange !== undefined;

	async function createEvaluationTemplate() {
		if (!builderApi) return;
		formSchema = await builderApi.getDefinitionData();

		const evaluationTemplateEntry: EvaluationTemplate = {
			name,
			form_schema: formSchema,
			score_range: scoreRange,
			score
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

<BaseLabelContent label="Template Name" class="mb-8">
	<Input id="name" bind:value={name} />
</BaseLabelContent>

<BaseLabelContent label="Evaluation Form" class="mb-8">
	<BaseFormBuilder bind:builderApi />
</BaseLabelContent>

<BaseLabelContent label="Scoring System" class="!mb-8">
	<InputScoreTemplate bind:score bind:scoreRange />
</BaseLabelContent>

<div class="flex justify-end">
	<Button color="green" on:click={createEvaluationTemplate} disabled={!isEvaluationTemplateValid}>
		Create Evaluation Template
	</Button>
</div>
