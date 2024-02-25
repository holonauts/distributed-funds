<script lang="ts">
import { createEventDispatcher, getContext, onMount } from 'svelte';
import type { AppAgentClient, Record, EntryHash, AgentPubKey, ActionHash, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { Evaluation, QuantitativeRating } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import type { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-textarea';

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

const dispatch = createEventDispatcher();

export let application!: ActionHash;

export let jsonData!: string;

export let quantitativeRating!: QuantitativeRating;


let comments: string = '';

let errorSnackbar: Snackbar;

$: application, jsonData, comments, quantitativeRating;
$: isEvaluationValid = true && comments !== '';

onMount(() => {
  if (application === undefined) {
    throw new Error(`The application input is required for the CreateEvaluation element`);
  }
  if (jsonData === undefined) {
    throw new Error(`The jsonData input is required for the CreateEvaluation element`);
  }
  if (quantitativeRating === undefined) {
    throw new Error(`The quantitativeRating input is required for the CreateEvaluation element`);
  }
});

async function createEvaluation() {  
  const evaluationEntry: Evaluation = { 
    application: application!,
    json_data: jsonData!,
    comments: comments!,
    quantitative_rating: quantitativeRating!,
  };
  
  try {
    const record: Record = await client.callZome({
      cap_secret: null,
      role_name: 'grant_pools',
      zome_name: 'grants',
      fn_name: 'create_evaluation',
      payload: evaluationEntry,
    });
    dispatch('evaluation-created', { evaluationHash: record.signed_action.hashed.hash });
  } catch (e) {
    errorSnackbar.labelText = `Error creating the evaluation: ${e.data.data}`;
    errorSnackbar.show();
  }
}

</script>
<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <span style="font-size: 18px">Create Evaluation</span>
  

  <div style="margin-bottom: 16px">
    <mwc-textarea outlined label="Comments" value={ comments } on:input={e => { comments = e.target.value;} } required></mwc-textarea>          
  </div>
            

  <mwc-button 
    raised
    label="Create Evaluation"
    disabled={!isEvaluationValid}
    on:click={() => createEvaluation()}
  ></mwc-button>
</div>
