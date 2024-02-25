
<script lang="ts">
import { onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import type { Link, ActionHash, EntryHash, AppAgentClient, Record, AgentPubKey, NewEntryAction } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { Evaluation } from './types';
import EvaluationDetail from './EvaluationDetail.svelte';

export let applicationHash: ActionHash;

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let links: Array<Link> | undefined;

let loading = true;
let error: any = undefined;

$: links, loading, error;

onMount(async () => {
  if (applicationHash === undefined) {
    throw new Error(`The applicationHash input is required for the EvaluationsForApplication element`);
  }

  try {
    links = await client.callZome({
      cap_secret: null,
      role_name: 'grant_pools',
      zome_name: 'grants',
      fn_name: 'get_evaluations_for_application',
      payload: applicationHash
    });
  } catch (e) {
    error = e;
  }
  loading = false;
});

</script>

{#if loading }
<div style="display: flex; flex: 1; align-items: center; justify-content: center">
  <mwc-circular-progress indeterminate></mwc-circular-progress>
</div>
{:else if error}
<span>Error fetching evaluations: ${error.data.data}.</span>
{:else if links.length === 0}
<span>No evaluations found for this application.</span>
{:else}
<div style="display: flex; flex-direction: column">
  {#each links as link}
    <div style="margin-bottom: 8px;">
      <EvaluationDetail evaluationHash={link.target}></EvaluationDetail>
    </div>
  {/each}
</div>
{/if}
