<script lang="ts">
	import { formatTimestampHumanized } from '$lib/utils/date';
	import { AngleDownOutline } from 'flowbite-svelte-icons';
	import BaseFormBuilder from '$lib/components/BaseFormBuilder.svelte';
	import { Button, Card, Label } from 'flowbite-svelte';
	import ProfileInline from '$lib/components/ProfileInline.svelte';
	import { createEventDispatcher } from 'svelte';
	import { type AgentPubKey } from '@holochain/client';
	import type { FormDefinition } from '@pragmatic-engineering/svelte-form-builder-community/Utils/types';
	const dispatch = createEventDispatcher();

	export let showCloneButton = false;
	export let showDetails = false;
	export let name: string;
	export let author: AgentPubKey;
	export let timestamp: number;
	export let formDefinition: FormDefinition[];
</script>

<div class="relative">
	{#if showCloneButton}
		<div class="absolute right-0 top-0">
			<Button size="xs" class="p-1" color="alternative" on:click={() => dispatch('clone')}
				>Clone</Button
			>
		</div>
	{/if}

	<div class="flex w-full items-center justify-between">
		<div class="w-full">
			<div class="mb-4 text-sm dark:text-white">{name}</div>
			<div class="flex w-full items-end justify-between">
				<div>
					<div class="text-xs dark:text-white">
						<div class="mb-1 text-xs text-gray-700 dark:text-gray-400">Author</div>
						<ProfileInline agentPubKey={author} />
					</div>
				</div>
				<div>
					<div class="mb-1 text-xs text-gray-700 dark:text-gray-400">Created</div>
					<div class="text-xs dark:text-white">
						{formatTimestampHumanized(timestamp)}
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
				<BaseFormBuilder view="render" {formDefinition} />

				<slot />
			</Card>
		</div>
	{/if}
</div>
