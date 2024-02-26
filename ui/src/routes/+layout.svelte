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
		getClient: () => client
	});
</script>

{#if loading}
	<div class="flex h-screen w-full items-center justify-center">
		<Spinner class="h-14 w-14" />
	</div>
{:else}
	<main>
		<slot />
	</main>
{/if}
