<script lang="ts">
	import { createEventDispatcher, onMount } from 'svelte';
	import type { Record } from '@holochain/client';
	import type { ApplicationTemplate } from '../../../grant_pools/grants/types';
	import { Button, Label, Input } from 'flowbite-svelte';
	import { toasts } from '$lib/stores/toast';
	import BaseFormBuilder from '$lib/components/BaseFormBuilder.svelte';
	import { holochainClient } from '$lib/stores/holochainClient';
	import { type BuilderAPI } from '@pragmatic-engineering/svelte-form-builder-community';
	import BaseLabelContent from '$lib/components/BaseLabelContent.svelte';

	const dispatch = createEventDispatcher();

	export let name: string = '';
	export let formSchema: string = '';

	$: isApplicationTemplateValid = name.length > 0;

	let builderApi: typeof BuilderAPI | undefined = undefined;

	async function createApplicationTemplate() {
		if (!builderApi) return;
		formSchema = await builderApi.getDefinitionData();

		const applicationTemplateEntry: ApplicationTemplate = {
			form_schema: formSchema,
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
		if (formSchema === undefined) {
			throw new Error(`The formSchema input is required for the CreateApplicationTemplate element`);
		}
	});
</script>

<BaseLabelContent label="Name" class="mb-8">
	<Input id="name" bind:value={name} />
</BaseLabelContent>

<BaseLabelContent label="Template" class="mb-8">
	<BaseFormBuilder bind:builderApi />
</BaseLabelContent>

<div class="flex justify-end">
	<Button color="green" on:click={createApplicationTemplate} disabled={!isApplicationTemplateValid}>
		Create Application Template
	</Button>
</div>
