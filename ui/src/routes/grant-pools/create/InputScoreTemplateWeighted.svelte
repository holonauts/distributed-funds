<script lang="ts">
	import { Input, Label } from 'flowbite-svelte';
	import type { AttributeScoreTemplate } from '../../../grant_pools/grants/types';
	import InputList from '$lib/components/InputList.svelte';

	const DEFAULT_ATTRIBUTE_SCORE_TEMPLATE = { label: '', weight: '' };

	export let value: AttributeScoreTemplate[] = [];
	let createAttributeScoreTemplate = DEFAULT_ATTRIBUTE_SCORE_TEMPLATE;

	function add() {
		value = [
			...value,
			{
				label: createAttributeScoreTemplate.label,
				weight: parseInt(createAttributeScoreTemplate.weight)
			}
		];

		resetCreateForm();
	}

	function resetCreateForm() {
		createAttributeScoreTemplate = DEFAULT_ATTRIBUTE_SCORE_TEMPLATE;
	}
</script>

<div class="mb-2">
	<Label>Weighted Attributes</Label>
	<InputList
		bind:items={value}
		createIsValid={createAttributeScoreTemplate !== undefined &&
			createAttributeScoreTemplate.label.length > 0 &&
			createAttributeScoreTemplate.weight.length > 0}
		on:add={add}
	>
		<svelte:fragment slot="item" let:item>
			<div class="flex items-center justify-start">
				<div class="flex-grow">
					<Label for="label">Label</Label>
					<div>{item.label}</div>
				</div>
				<div class="w-32">
					<Label for="weight">Weight</Label>
					<div>{item.weight}</div>
				</div>
			</div>
		</svelte:fragment>

		<svelte:fragment slot="create">
			<div class="flex items-center justify-center space-x-10">
				<div class="flex-grow">
					<Label for="label">Label</Label>
					<Input id="label" bind:value={createAttributeScoreTemplate.label} placeholder="Impact" />
				</div>
				<div class="w-32">
					<Label for="weight">Weight</Label>
					<Input id="weight" bind:value={createAttributeScoreTemplate.weight} placeholder="1" />
				</div>
			</div>
		</svelte:fragment>
	</InputList>
</div>
