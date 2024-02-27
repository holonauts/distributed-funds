<script lang="ts">
	import { Card, Input, Label, TabItem, Tabs } from 'flowbite-svelte';
	import type {
		NumberRange,
		ScoreTemplate,
		NumberRangeWeightedCriteria,
		WeightedCriteria
	} from '../../../grant_pools/grants/types';
	import { ScoreType } from '../../../grant_pools/grants/types';
	import InputList from '$lib/components/InputList.svelte';
	import InputNumberRange from '$lib/components/InputNumberRange.svelte';

	let type: ScoreType = ScoreType.Single;
	let numberRange: NumberRange;
	let weightedCriteria: WeightedCriteria[] = [];
	let addCriteriaLabel: string;
	let addCriteriaWeight: string;

	export let value: ScoreTemplate;

	$: {
		value = {
			type,
			content:
				type === ScoreType.Single
					? numberRange
					: { range: numberRange, weighted_criteria: weightedCriteria }
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
		open={type === ScoreType.Single}
		on:click={() => (type = ScoreType.Single)}
	/>
	<TabItem
		defaultClass="!p-2 text-xs"
		title="Weighted Criteria"
		class="w-full"
		open={type === ScoreType.Weighted}
		on:click={() => (type = ScoreType.Weighted)}
	/>
</Tabs>

<Card size="xl">
	{#if type === ScoreType.Single}
		<Label>Allowed Range</Label>
		<InputNumberRange bind:value={numberRange} />
	{:else if type === ScoreType.Weighted}
		<div class="mb-2">
			<Label>Allowed Range</Label>
			<InputNumberRange bind:value={numberRange} />
		</div>
		<div class="mb-2">
			<Label>Ranking Criteria</Label>
			<InputList
				bind:items={weightedCriteria}
				createIsValid={addCriteriaLabel !== undefined &&
					addCriteriaLabel.length > 0 &&
					addCriteriaWeight !== undefined}
				on:add={() => {
					weightedCriteria = [
						...weightedCriteria,
						{ label: addCriteriaLabel, weight: parseInt(addCriteriaWeight) }
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
		</div>
	{/if}
</Card>
