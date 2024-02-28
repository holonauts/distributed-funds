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

		console.log('evaluationTemplateEntry', evaluationTemplateEntry);

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
		<Label>Evaluation Template</Label>
		<BaseFormBuilder bind:builderApi />
	</div>

	<div class="mb-8">
		<Label class="mb-2">Score Template</Label>
		<InputScoreTemplate bind:score bind:scoreRange />
	</div>

	<Button on:click={createEvaluationTemplate} disabled={!isEvaluationTemplateValid}>
		Create Evaluation Template
	</Button>
</div>
