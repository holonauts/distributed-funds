<script lang="ts">
import { createEventDispatcher, getContext, onMount } from 'svelte';
import type { AppAgentClient, Record, EntryHash, AgentPubKey, ActionHash, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { GrantPoolOutcome } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import type { Snackbar } from '@material/mwc-snackbar';

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

const dispatch = createEventDispatcher();

export let grantPool!: ActionHash;

export let outcomes!: string;

export let coupon!: Array<number>;



let errorSnackbar: Snackbar;

$: grantPool, outcomes, coupon;
$: isGrantPoolOutcomeValid = true;

onMount(() => {
  if (grantPool === undefined) {
    throw new Error(`The grantPool input is required for the CreateGrantPoolOutcome element`);
  }
  if (outcomes === undefined) {
    throw new Error(`The outcomes input is required for the CreateGrantPoolOutcome element`);
  }
  if (coupon === undefined) {
    throw new Error(`The coupon input is required for the CreateGrantPoolOutcome element`);
  }
});

async function createGrantPoolOutcome() {  
  const grantPoolOutcomeEntry: GrantPoolOutcome = { 
    grant_pool: grantPool!,
    outcomes: outcomes!,
    coupon: coupon,
  };
  
  try {
    const record: Record = await client.callZome({
      cap_secret: null,
      role_name: 'grant_pools',
      zome_name: 'grants',
      fn_name: 'create_grant_pool_outcome',
      payload: grantPoolOutcomeEntry,
    });
    dispatch('grant-pool-outcome-created', { grantPoolOutcomeHash: record.signed_action.hashed.hash });
  } catch (e) {
    errorSnackbar.labelText = `Error creating the grant pool outcome: ${e.data.data}`;
    errorSnackbar.show();
  }
}

</script>
<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <span style="font-size: 18px">Create GrantPoolOutcome</span>
  


  <mwc-button 
    raised
    label="Create GrantPoolOutcome"
    disabled={!isGrantPoolOutcomeValid}
    on:click={() => createGrantPoolOutcome()}
  ></mwc-button>
</div>
