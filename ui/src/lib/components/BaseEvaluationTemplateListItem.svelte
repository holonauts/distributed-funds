<script lang="ts">
	import { formatRecordTimestampHumanized } from '$lib/utils/date';
	import { encodeHashToBase64, type Record } from '@holochain/client';
	import type { EvaluationTemplate } from '../../grant_pools/grants/types';
	import { AngleDownOutline } from 'flowbite-svelte-icons';
	import BaseFormBuilder from './BaseFormBuilder.svelte';
	import { Card, Label } from 'flowbite-svelte';
	import FormQuantitativeRating from '$lib/components/FormQuantitativeRating.svelte';

	export let evaluationTemplate: EvaluationTemplate;
	export let record: Record;
	let showDetails = false;
</script>

<div class="flex w-full items-center justify-between">
	<div class="w-full">
		<div class="mb-4 text-sm dark:text-white">{evaluationTemplate.name}</div>
		<div class="flex w-full items-end justify-between">
			<div>
				<div class="text-xs dark:text-white">
					<div class="mb-1 text-xs text-gray-700 dark:text-gray-400">Author</div>
					{encodeHashToBase64(record.signed_action.hashed.content.author)}
				</div>
			</div>
			<div>
				<div class="mb-1 text-xs text-gray-700 dark:text-gray-400">Created</div>
				<div class="text-xs dark:text-white">
					{formatRecordTimestampHumanized(record)}
				</div>
			</div>
		</div>
	</div>

	<div class="ml-10 py-0 text-xs">
		<AngleDownOutline
			class="h-8 w-8 text-gray-700 dark:text-gray-400"
			on:click={() => (showDetails = !showDetails)}
		/>
	</div>
</div>

{#if showDetails}
	<div class="mt-4">
		<Label class="mb-2">Template Preview</Label>
		<Card size="xl">
			<div class="mb-2">
				<BaseFormBuilder
					view="render"
					formDefinition={JSON.parse(evaluationTemplate.qualitative_json_schema).formDefinition}
				/>
			</div>

			<FormQuantitativeRating quantitativeRatingTemplate={evaluationTemplate.quantitative_rating} />
		</Card>
	</div>
{/if}
