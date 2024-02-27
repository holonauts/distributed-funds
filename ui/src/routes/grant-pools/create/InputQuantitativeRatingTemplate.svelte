<script lang="ts">
	import { Card, Input, Label, TabItem, Tabs } from 'flowbite-svelte';
	import type { QuantitativeRating, WeightedCriteria } from '../../../grant_pools/grants/types';
	import { QuantitativeRatingType } from '../../../grant_pools/grants/types';
	import InputList from '$lib/components/InputList.svelte';

	let type: QuantitativeRatingType = QuantitativeRatingType.Single;
	let numberRangeMin: number;
	let numberRangeMax: number;
	let weightedCriteria: WeightedCriteria[] = [];
	let addCriteriaLabel: string;
	let addCriteriaWeight: number;

	export let value: QuantitativeRating;

	$: {
		value = {
			type,
			content:
				type === QuantitativeRatingType.Single
					? { min: numberRangeMin, max: numberRangeMax }
					: weightedCriteria
		};
	}
</script>

<Tabs
	contentClass="mb-4"
	style="full"
	defaultClass="flex rounded-lg divide-x rtl:divide-x-reverse divide-gray-200 shadow dark:divide-gray-700"
>
	<TabItem
		defaultClass="!p-2 text-xs"
		title="Single Score"
		class="w-full"
		open={type === QuantitativeRatingType.Single}
		on:click={() => (type = QuantitativeRatingType.Single)}
	/>
	<TabItem
		defaultClass="!p-2 text-xs"
		title="Weighted Criteria"
		class="w-full"
		open={type === QuantitativeRatingType.Weighted}
		on:click={() => (type = QuantitativeRatingType.Weighted)}
	/>
</Tabs>

<Card size="xl">
	{#if type === QuantitativeRatingType.Single}
		<div class="flex items-center justify-center space-x-10">
			<div>
				<Label for="min-score">Minimum</Label>
				<Input id="min-score" bind:value={numberRangeMin} placeholder="0" />
			</div>
			<div>
				<Label for="min-score">Maximum</Label>
				<Input id="min-score" bind:value={numberRangeMax} placeholder="100" />
			</div>
		</div>
	{:else if type === QuantitativeRatingType.Weighted}
		<InputList
			bind:items={weightedCriteria}
			createIsValid={addCriteriaLabel !== undefined &&
				addCriteriaLabel.length > 0 &&
				addCriteriaWeight !== undefined}
			on:add={() => {
				weightedCriteria = [
					...weightedCriteria,
					{ label: addCriteriaLabel, weight: addCriteriaWeight }
				];
				addCriteriaLabel = '';
				addCriteriaWeight = undefined;
			}}
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
						<Input id="label" bind:value={addCriteriaLabel} placeholder="Impact" />
					</div>
					<div class="w-32">
						<Label for="weight">Weight</Label>
						<Input id="weight" bind:value={addCriteriaWeight} placeholder="1" />
					</div>
				</div>
			</svelte:fragment>
		</InputList>
	{/if}
</Card>
