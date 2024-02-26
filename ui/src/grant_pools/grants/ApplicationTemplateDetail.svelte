<script lang="ts">
	import { createEventDispatcher, onMount, getContext } from 'svelte';
	import { Spinner } from 'flowbite-svelte';
	import { decode } from '@msgpack/msgpack';
	import type {
		Record,
		ActionHash,
		AppAgentClient,
		EntryHash,
		AgentPubKey,
		DnaHash
	} from '@holochain/client';
	import { clientContext } from '../../contexts';
	import type { ApplicationTemplate } from './types';
	import { Spinner } from 'flowbite-svelte';
	import type { Snackbar } from '@material/mwc-snackbar';
	import '@material/mwc-snackbar';
	import '@material/mwc-icon-button';

	const dispatch = createEventDispatcher();

	export let applicationTemplateHash: ActionHash;

	let client: AppAgentClient = (getContext(clientContext) as any).getClient();

	let loading = true;
	let error: any = undefined;

	let record: Record | undefined;
	let applicationTemplate: ApplicationTemplate | undefined;

	$: error, loading, record, applicationTemplate;

	onMount(async () => {
		if (applicationTemplateHash === undefined) {
			throw new Error(
				`The applicationTemplateHash input is required for the ApplicationTemplateDetail element`
			);
		}
		await fetchApplicationTemplate();
	});

	async function fetchApplicationTemplate() {
		loading = true;
		error = undefined;
		record = undefined;
		applicationTemplate = undefined;

		try {
			record = await client.callZome({
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
			error = e;
		}

		loading = false;
	}
</script>

{#if loading}
	<div style="display: flex; flex: 1; align-items: center; justify-content: center">
		<Spinner class="h-4 w-4" />
	</div>
{:else if error}
	<span>Error fetching the application template: {error.data.data}</span>
{:else}
	<div style="display: flex; flex-direction: column">
		<div style="display: flex; flex-direction: row">
			<span style="flex: 1"></span>
		</div>
	</div>
{/if}
