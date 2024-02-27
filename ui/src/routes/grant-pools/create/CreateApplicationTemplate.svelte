<script lang="ts">
	import { createEventDispatcher, onMount } from 'svelte';
	import type { Record } from '@holochain/client';
	import type { ApplicationTemplate } from '../../../grant_pools/grants/types';
	import { Button, Label, Input } from 'flowbite-svelte';
	import { toasts } from '$lib/stores/toast';
	import BaseFormBuilder from '$lib/components/BaseFormBuilder.svelte';
	import { holochainClient } from '$lib/stores/holochainClient';
	import { type BuilderAPI } from '@pragmatic-engineering/svelte-form-builder-community';

	const dispatch = createEventDispatcher();

	export let jsonSchema: string = '';
	export let name: string = '';

	$: isApplicationTemplateValid = name.length > 0;

	let builderApi: typeof BuilderAPI | undefined = undefined;

	async function createApplicationTemplate() {
		if (!builderApi) return;
		jsonSchema = await builderApi.getDefinitionData();

		const applicationTemplateEntry: ApplicationTemplate = {
			json_schema: jsonSchema,
			name
		};

		try {
			const record: Record = await $holochainClient.client.callZome({
				cap_secret: null,
				role_name: 'grant_pools',
				zome_name: 'grants',
				fn_name: 'create_application_template',
				payload: applicationTemplateEntry
			});
			dispatch('application-template-created', {
				applicationTemplateHash: record.signed_action.hashed.hash
			});
		} catch (e) {
			toasts.error(`Error creating the application template: ${e}`);
		}
	}

	onMount(() => {
		if (jsonSchema === undefined) {
			throw new Error(`The jsonSchema input is required for the CreateApplicationTemplate element`);
		}
	});
</script>

<div class="flex flex-col">
	<div class="mb-8">
		<Label for="name">Name</Label>
		<Input id="name" bind:value={name} />
	</div>

	<div class="mb-8">
		<Label>Template</Label>
		<BaseFormBuilder bind:builderApi />
	</div>

	<Button on:click={createApplicationTemplate} disabled={!isApplicationTemplateValid}>
		Create Application Template
	</Button>
</div>
