<script lang="ts">
	import { Input, Label } from 'flowbite-svelte';
	import type { AttributeScoreTemplate } from '../../../grant_pools/grants/types';
	import InputList from '$lib/components/InputList.svelte';

	export let value: AttributeScoreTemplate[] = [];
	let createAttributeScoreTemplate = { label: '', weight: 1 };

	$: createIsValid =
		createAttributeScoreTemplate !== undefined &&
		createAttributeScoreTemplate.label.length > 0 &&
		createAttributeScoreTemplate.weight;

	function add() {
		value = [...value, createAttributeScoreTemplate];

		reset();
	}

	function reset() {
		createAttributeScoreTemplate = { label: '', weight: 1 };
	}
</script>

<div class="mb-2">
	<Label>Weighted Attributes</Label>
	<InputList bind:items={value} {createIsValid} on:add={add}>
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
					<input
						type="number"
						id="weight"
						bind:value={createAttributeScoreTemplate.weight}
						placeholder="1"
						class="focus:border-primary-500 focus:ring-primary-500 dark:focus:border-primary-500 dark:focus:ring-primary-500 block w-full rounded-lg border-gray-300 bg-gray-50 p-2.5 text-sm text-gray-900 disabled:cursor-not-allowed disabled:opacity-50 rtl:text-right dark:border-gray-500 dark:bg-gray-600 dark:text-white dark:placeholder-gray-400"
					/>
				</div>
			</div>
		</svelte:fragment>
	</InputList>
</div>
