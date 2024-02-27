<script lang="ts">
	import { onMount } from 'svelte';
	import { decode } from '@msgpack/msgpack';
	import { type Record, type ActionHash } from '@holochain/client';
	import type { ApplicationTemplate } from '../../grant_pools/grants/types';
	import { holochainClient } from '$lib/stores/holochainClient';
	import BaseApplicationTemplateListItem from '$lib/components/BaseApplicationTemplateListItem.svelte';
	import { Card } from 'flowbite-svelte';

	export let applicationTemplateHash: ActionHash;

	let loading = true;
	let record: Record | undefined;
	let applicationTemplate: ApplicationTemplate | undefined;

	$: applicationTemplateHash, fetchApplicationTemplate();

	async function fetchApplicationTemplate() {
		loading = true;
		record = undefined;
		applicationTemplate = undefined;

		try {
			record = await $holochainClient.client.callZome({
				cap_secret: null,
				role_name: 'grant_pools',
				zome_name: 'grants',
				fn_name: 'get_application_template',
				payload: applicationTemplateHash
			});
			if (record) {
				applicationTemplate = decode((record.entry as any).Present.entry) as ApplicationTemplate;
			}
		} catch (e) {
			console.error(e);
		}

		loading = false;
	}
</script>

{#if loading}
	<div class="h-8 w-full animate-pulse rounded-md"></div>
{:else if record === undefined || applicationTemplate === undefined}
	<div>Application template not found</div>
{:else}
	<Card size="xl">
		<BaseApplicationTemplateListItem {record} {applicationTemplate} />
	</Card>
{/if}
