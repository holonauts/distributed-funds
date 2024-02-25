<script lang="ts">
import { createEventDispatcher, getContext, onMount } from 'svelte';
import type { AppAgentClient, Record, EntryHash, AgentPubKey, ActionHash, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { ApplicationTemplate } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import type { Snackbar } from '@material/mwc-snackbar';

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

const dispatch = createEventDispatcher();

export let jsonSchema!: string;



let errorSnackbar: Snackbar;

$: jsonSchema;
$: isApplicationTemplateValid = true;

onMount(() => {
  if (jsonSchema === undefined) {
    throw new Error(`The jsonSchema input is required for the CreateApplicationTemplate element`);
  }
});

async function createApplicationTemplate() {  
  const applicationTemplateEntry: ApplicationTemplate = { 
    json_schema: jsonSchema!,
  };
  
  try {
    const record: Record = await client.callZome({
      cap_secret: null,
      role_name: 'grant_pools',
      zome_name: 'grants',
      fn_name: 'create_application_template',
      payload: applicationTemplateEntry,
    });
    dispatch('application-template-created', { applicationTemplateHash: record.signed_action.hashed.hash });
  } catch (e) {
    errorSnackbar.labelText = `Error creating the application template: ${e.data.data}`;
    errorSnackbar.show();
  }
}

</script>
<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <span style="font-size: 18px">Create ApplicationTemplate</span>
  


  <mwc-button 
    raised
    label="Create ApplicationTemplate"
    disabled={!isApplicationTemplateValid}
    on:click={() => createApplicationTemplate()}
  ></mwc-button>
</div>
