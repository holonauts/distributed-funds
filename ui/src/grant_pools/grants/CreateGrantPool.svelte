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
	import type { GrantPool } from './types';
	import '@material/mwc-button';
	import '@material/mwc-snackbar';
	import type { Snackbar } from '@material/mwc-snackbar';
	import '@material/mwc-textarea';

	import '@material/mwc-textfield';
	let client: AppAgentClient = (getContext(clientContext) as any).getClient();

	const dispatch = createEventDispatcher();

	export let timePeriod!: ActionHash;

	export let applicationTemplate!: ActionHash;

	export let evaluationTemplate!: ActionHash;

	let name: string = '';
	let purposeDescription: string = '';
	let rulesDescription: string = '';

	let errorSnackbar: Snackbar;

	$: name,
		purposeDescription,
		rulesDescription,
		timePeriod,
		applicationTemplate,
		evaluationTemplate;
	$: isGrantPoolValid = true && name !== '' && purposeDescription !== '' && rulesDescription !== '';

	onMount(() => {
		if (timePeriod === undefined) {
			throw new Error(`The timePeriod input is required for the CreateGrantPool element`);
		}
		if (applicationTemplate === undefined) {
			throw new Error(`The applicationTemplate input is required for the CreateGrantPool element`);
		}
		if (evaluationTemplate === undefined) {
			throw new Error(`The evaluationTemplate input is required for the CreateGrantPool element`);
		}
	});

	async function createGrantPool() {
		const grantPoolEntry: GrantPool = {
			name: name!,
			purpose_description: purposeDescription!,
			rules_description: rulesDescription!,
			time_period: timePeriod!,
			application_template: applicationTemplate!,
			evaluation_template: evaluationTemplate!
		};

		try {
			const record: Record = await client.callZome({
				cap_secret: null,
				role_name: 'grant_pools',
				zome_name: 'grants',
				fn_name: 'create_grant_pool',
				payload: grantPoolEntry
			});
			dispatch('grant-pool-created', { grantPoolHash: record.signed_action.hashed.hash });
		} catch (e) {
			errorSnackbar.labelText = `Error creating the grant pool: ${e.data.data}`;
			errorSnackbar.show();
		}
	}
</script>

<mwc-snackbar bind:this={errorSnackbar} leading> </mwc-snackbar>
<div style="display: flex; flex-direction: column">
	<span style="font-size: 18px">Create GrantPool</span>

	<div style="margin-bottom: 16px">
		<mwc-textfield
			outlined
			label="Name"
			value={name}
			on:input={(e) => {
				name = e.target.value;
			}}
			required
		></mwc-textfield>
	</div>

	<div style="margin-bottom: 16px">
		<mwc-textarea
			outlined
			label="Purpose Description"
			value={purposeDescription}
			on:input={(e) => {
				purposeDescription = e.target.value;
			}}
			required
		></mwc-textarea>
	</div>

	<div style="margin-bottom: 16px">
		<mwc-textarea
			outlined
			label="Rules Description"
			value={rulesDescription}
			on:input={(e) => {
				rulesDescription = e.target.value;
			}}
			required
		></mwc-textarea>
	</div>

	<mwc-button
		raised
		label="Create GrantPool"
		disabled={!isGrantPoolValid}
		on:click={() => createGrantPool()}
	></mwc-button>
</div>
