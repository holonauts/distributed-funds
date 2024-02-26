<script lang="ts">
	import { createEventDispatcher, getContext, onMount } from 'svelte';
	import type {
		AppAgentClient,
		Record,
		EntryHash,
		AgentPubKey,
		ActionHash,
		DnaHash
	} from '@holochain/client';
	import { clientContext } from '../../contexts';
	import type { Application, Status } from './types';
	import '@material/mwc-button';
	import '@material/mwc-snackbar';
	import type { Snackbar } from '@material/mwc-snackbar';

	let client: AppAgentClient = (getContext(clientContext) as any).getClient();

	const dispatch = createEventDispatcher();

	export let applicationTemplate!: ActionHash;

	export let jsonData!: string;

	export let status!: Status;

	let errorSnackbar: Snackbar;

	$: applicationTemplate, jsonData, status;
	$: isApplicationValid = true;

	onMount(() => {
		if (applicationTemplate === undefined) {
			throw new Error(
				`The applicationTemplate input is required for the CreateApplication element`
			);
		}
		if (jsonData === undefined) {
			throw new Error(`The jsonData input is required for the CreateApplication element`);
		}
		if (status === undefined) {
			throw new Error(`The status input is required for the CreateApplication element`);
		}
	});

	async function createApplication() {
		const applicationEntry: Application = {
			application_template: applicationTemplate!,
			json_data: jsonData!,
			status: status!
		};

		try {
			const record: Record = await client.callZome({
				cap_secret: null,
				role_name: 'grant_pools',
				zome_name: 'grants',
				fn_name: 'create_application',
				payload: applicationEntry
			});
			dispatch('application-created', { applicationHash: record.signed_action.hashed.hash });
		} catch (e) {
			errorSnackbar.labelText = `Error creating the application: ${e.data.data}`;
			errorSnackbar.show();
		}
	}
</script>

<mwc-snackbar bind:this={errorSnackbar} leading> </mwc-snackbar>
<div style="display: flex; flex-direction: column">
	<span style="font-size: 18px">Create Application</span>

	<mwc-button
		raised
		label="Create Application"
		disabled={!isApplicationValid}
		on:click={() => createApplication()}
	></mwc-button>
</div>
