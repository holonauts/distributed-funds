<script lang="ts">
import { createEventDispatcher, getContext, onMount } from 'svelte';
import type { AppAgentClient, Record, EntryHash, AgentPubKey, ActionHash, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { TimePeriod } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import type { Snackbar } from '@material/mwc-snackbar';

import '@vaadin/date-time-picker/theme/material/vaadin-date-time-picker.js';
let client: AppAgentClient = (getContext(clientContext) as any).getClient();

const dispatch = createEventDispatcher();


let startAt: number = Date.now();
let endAt: number = Date.now();

let errorSnackbar: Snackbar;

$: startAt, endAt;
$: isTimePeriodValid = true && true && true;

onMount(() => {
});

async function createTimePeriod() {  
  const timePeriodEntry: TimePeriod = { 
    start_at: startAt!,
    end_at: endAt!,
  };
  
  try {
    const record: Record = await client.callZome({
      cap_secret: null,
      role_name: 'grant_pools',
      zome_name: 'grants',
      fn_name: 'create_time_period',
      payload: timePeriodEntry,
    });
    dispatch('time-period-created', { timePeriodHash: record.signed_action.hashed.hash });
  } catch (e) {
    errorSnackbar.labelText = `Error creating the time period: ${e.data.data}`;
    errorSnackbar.show();
  }
}

</script>
<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <span style="font-size: 18px">Create TimePeriod</span>
  

  <div style="margin-bottom: 16px">
    <vaadin-date-time-picker label="Start At" value={new Date(startAt / 1000).toISOString()} on:change={e => { startAt = new Date(e.target.value).valueOf() * 1000;} } required></vaadin-date-time-picker>          
  </div>
            
  <div style="margin-bottom: 16px">
    <vaadin-date-time-picker label="End At" value={new Date(endAt / 1000).toISOString()} on:change={e => { endAt = new Date(e.target.value).valueOf() * 1000;} } required></vaadin-date-time-picker>          
  </div>
            

  <mwc-button 
    raised
    label="Create TimePeriod"
    disabled={!isTimePeriodValid}
    on:click={() => createTimePeriod()}
  ></mwc-button>
</div>
