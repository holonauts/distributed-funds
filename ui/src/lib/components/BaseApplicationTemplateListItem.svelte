<script lang="ts">
	import { formatRecordTimestampHumanized } from '$lib/utils/date';
	import { type Record } from '@holochain/client';
	import type { ApplicationTemplate } from '../../grant_pools/grants/types';
	import { AngleDownOutline } from 'flowbite-svelte-icons';
	import BaseFormBuilder from './BaseFormBuilder.svelte';
	import { Card, Label } from 'flowbite-svelte';
	import ProfileInline from './ProfileInline.svelte';

	export let applicationTemplate: ApplicationTemplate;
	export let record: Record;
	let showSchema = false;
</script>

<div class="flex w-full items-center justify-between">
	<div class="w-full">
		<div class="mb-4 text-sm dark:text-white">{applicationTemplate.name}</div>
		<div class="flex w-full items-end justify-between">
			<div>
				<div class="text-xs dark:text-white">
					<div class="mb-1 text-xs text-gray-700 dark:text-gray-400">Author</div>
					<ProfileInline agentPubKey={record.signed_action.hashed.content.author} />
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
			on:click={() => (showSchema = !showSchema)}
		/>
	</div>
</div>

{#if showSchema}
	<div class="mt-4">
		<Label class="mb-2">Template Preview</Label>

		<Card size="xl">
			<BaseFormBuilder
				view="render"
				formDefinition={JSON.parse(applicationTemplate.form_schema).formDefinition}
			/>
		</Card>
	</div>
{/if}
