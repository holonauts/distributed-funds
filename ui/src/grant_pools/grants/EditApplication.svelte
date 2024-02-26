<script lang="ts">
	import { createEventDispatcher, getContext, onMount } from 'svelte';
	import type {
		AppAgentClient,
		Record,
		EntryHash,
		AgentPubKey,
		DnaHash,
		ActionHash
	} from '@holochain/client';
	import { decode } from '@msgpack/msgpack';
	import { clientContext } from '../../contexts';
	import type { Application, Status } from './types';
	import '@material/mwc-button';
	import '@material/mwc-snackbar';
	import type { Snackbar } from '@material/mwc-snackbar';

	let client: AppAgentClient = (getContext(clientContext) as any).getClient();

	const dispatch = createEventDispatcher();

	export let originalApplicationHash!: ActionHash;

	export let currentRecord!: Record;
	let currentApplication: Application = decode(
		(currentRecord.entry as any).Present.entry
	) as Application;

	let errorSnackbar: Snackbar;

	$:;
	$: isApplicationValid = true;

	onMount(() => {
		if (currentRecord === undefined) {
			throw new Error(`The currentRecord input is required for the EditApplication element`);
		}
		if (originalApplicationHash === undefined) {
			throw new Error(
				`The originalApplicationHash input is required for the EditApplication element`
			);
		}
	});

	async function updateApplication() {
		const application: Application = {
			application_template: currentApplication.application_template,
			json_data: currentApplication.json_data,
			status: currentApplication.status
		};

		try {
			const updateRecord: Record = await client.callZome({
				cap_secret: null,
				role_name: 'grant_pools',
				zome_name: 'grants',
				fn_name: 'update_application',
				payload: {
					original_application_hash: originalApplicationHash,
					previous_application_hash: currentRecord.signed_action.hashed.hash,
					updated_application: application
				}
			});

			dispatch('application-updated', { actionHash: updateRecord.signed_action.hashed.hash });
		} catch (e) {
			errorSnackbar.labelText = `Error updating the application: ${e.data.data}`;
			errorSnackbar.show();
		}
	}
</script>

<mwc-snackbar bind:this={errorSnackbar} leading> </mwc-snackbar>
<div style="display: flex; flex-direction: column">
	<span style="font-size: 18px">Edit Application</span>

	<div style="display: flex; flex-direction: row">
		<mwc-button
			outlined
			label="Cancel"
			on:click={() => dispatch('edit-canceled')}
			style="flex: 1; margin-right: 16px"
		></mwc-button>
		<mwc-button
			raised
			label="Save"
			disabled={!isApplicationValid}
			on:click={() => updateApplication()}
			style="flex: 1;"
		></mwc-button>
	</div>
</div>
