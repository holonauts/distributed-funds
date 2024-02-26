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

<main class="flex w-full h-full">
  {#if loading || true }
    <div class="flex w-full h-screen justify-center items-center">
      <Spinner class="w-8 h-8" />
			<Input />
    </div>
  {:else}
		<slot></slot>
  {/if}
</main>
