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
	import type { Application, Status } from './types';
	import { Spinner } from 'flowbite-svelte';
	import type { Snackbar } from '@material/mwc-snackbar';
	import '@material/mwc-snackbar';
	import '@material/mwc-icon-button';
	import EditApplication from './EditApplication.svelte';

	const dispatch = createEventDispatcher();

	export let applicationHash: ActionHash;

	let client: AppAgentClient = (getContext(clientContext) as any).getClient();

	let loading = true;
	let error: any = undefined;

	let record: Record | undefined;
	let application: Application | undefined;

	let editing = false;

	$: editing, error, loading, record, application;

	onMount(async () => {
		if (applicationHash === undefined) {
			throw new Error(`The applicationHash input is required for the ApplicationDetail element`);
		}
		await fetchApplication();
	});

	async function fetchApplication() {
		loading = true;
		error = undefined;
		record = undefined;
		application = undefined;

		try {
			record = await client.callZome({
				cap_secret: null,
				role_name: 'grant_pools',
				zome_name: 'grants',
				fn_name: 'get_latest_application',
				payload: applicationHash
			});
			if (record) {
				application = decode((record.entry as any).Present.entry) as Application;
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
	<span>Error fetching the application: {error.data.data}</span>
{:else if editing}
	<EditApplication
		originalApplicationHash={applicationHash}
		currentRecord={record}
		on:application-updated={async () => {
			editing = false;
			await fetchApplication();
		}}
		on:edit-canceled={() => {
			editing = false;
		}}
	></EditApplication>
{:else}
	<div style="display: flex; flex-direction: column">
		<div style="display: flex; flex-direction: row">
			<span style="flex: 1"></span>
			<mwc-icon-button
				style="margin-left: 8px"
				icon="edit"
				on:click={() => {
					editing = true;
				}}
			></mwc-icon-button>
		</div>
	</div>
{/if}
