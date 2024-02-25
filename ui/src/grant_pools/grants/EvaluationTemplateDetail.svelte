<script lang="ts">
import { createEventDispatcher, onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import { decode } from '@msgpack/msgpack';
import type { Record, ActionHash, AppAgentClient, EntryHash, AgentPubKey, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { EvaluationTemplate, QuantitativeRating } from './types';
import '@material/mwc-circular-progress';
import type { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-snackbar';
import '@material/mwc-icon-button';

const dispatch = createEventDispatcher();

export let evaluationTemplateHash: ActionHash;

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let loading = true;
let error: any = undefined;

let record: Record | undefined;
let evaluationTemplate: EvaluationTemplate | undefined;


  
$:  error, loading, record, evaluationTemplate;

onMount(async () => {
  if (evaluationTemplateHash === undefined) {
    throw new Error(`The evaluationTemplateHash input is required for the EvaluationTemplateDetail element`);
  }
  await fetchEvaluationTemplate();
});

async function fetchEvaluationTemplate() {
  loading = true;
  error = undefined;
  record = undefined;
  evaluationTemplate = undefined;
  
  try {
    record = await client.callZome({
      cap_secret: null,
      role_name: 'grant_pools',
      zome_name: 'grants',
      fn_name: 'get_evaluation_template',
      payload: evaluationTemplateHash,
    });
    if (record) {
      evaluationTemplate = decode((record.entry as any).Present.entry) as EvaluationTemplate;
    }
  } catch (e) {
    error = e;
  }

  loading = false;
}

</script>


{#if loading}
<div style="display: flex; flex: 1; align-items: center; justify-content: center">
  <mwc-circular-progress indeterminate></mwc-circular-progress>
</div>
{:else if error}
<span>Error fetching the evaluation template: {error.data.data}</span>
{:else}

<div style="display: flex; flex-direction: column">
  <div style="display: flex; flex-direction: row">
    <span style="flex: 1"></span>
  </div>

</div>
{/if}

