<script lang="ts">
	import '../app.pcss';
	import { onMount, setContext } from 'svelte';
  import type { ActionHash, AppAgentClient } from '@holochain/client';
	import { AppAgentWebsocket } from '@holochain/client';
	import { Spinner, Input } from 'flowbite-svelte';
  import { clientContext } from '../contexts';

  let client: AppAgentClient | undefined;
  let loading = true; 

  onMount(async () => {
    // We pass an unused string as the url because it will dynamically be replaced in launcher environments
    client = await AppAgentWebsocket.connect(new URL('https://UNUSED'), 'grant-funding');

    loading = false;
  });

  setContext(clientContext, {
    getClient: () => client,
  });
</script>

{#if loading}
  <div class="w-full h-screen flex justify-center items-center">
    <Spinner class="w-14 h-14" />
  </div>
{:else}
  <main>
    <slot></slot>
  </main>
{/if}
