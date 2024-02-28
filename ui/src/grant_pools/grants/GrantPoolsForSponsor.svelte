<script lang="ts">
import { onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import type { Record, EntryHash, ActionHash, AgentPubKey, AppAgentClient, NewEntryAction } from '@holochain/client';
import { clientContext } from '../../contexts';
import GrantPoolDetail from './GrantPoolDetail.svelte';
import type { GrantsSignal } from './types';

export let sponsor: AgentPubKey;

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let hashes: Array<ActionHash> | undefined;

let loading = true;
let error: any = undefined;

$: hashes, loading, error;

onMount(async () => {
  if (sponsor === undefined) {
    throw new Error(`The sponsor input is required for the GrantPoolsForSponsor element`);
  }

  try {
    const links = await client.callZome({
      cap_secret: null,
      role_name: 'grant_pools',
      zome_name: 'grants',
      fn_name: 'get_grant_pools_for_sponsor',
      payload: sponsor,
    });
    hashes = links.map(l => l.target);
  } catch (e) {
    error = e;
  }
  loading = false;

  client.on('signal', signal => {
    if (signal.zome_name !== 'grants') return;
    const payload = signal.payload as GrantsSignal;
    if (payload.type !== 'LinkCreated') return;
    if (payload.link_type !== 'SponsorToGrantPools') return;

    hashes = [...hashes, payload.action.hashed.content.target_address];
  });
});

</script>

{#if loading }
<div style="display: flex; flex: 1; align-items: center; justify-content: center">
  <mwc-circular-progress indeterminate></mwc-circular-progress>
</div>
{:else if error}
<span>Error fetching grant pools: {error.data.data}.</span>
{:else if hashes.length === 0}
<span>No grant pools found for this sponsor.</span>
{:else}
<div style="display: flex; flex-direction: column">
  {#each hashes as hash}
    <div style="margin-bottom: 8px;">
      <GrantPoolDetail grantPoolHash={hash}></GrantPoolDetail>
    </div>
  {/each}
</div>
{/if}
