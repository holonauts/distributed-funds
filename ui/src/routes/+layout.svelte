<script lang="ts">
	import '../app.pcss';
	import { onMount, setContext } from 'svelte';
	import { Spinner } from 'flowbite-svelte';
	import { clientContext } from '../contexts';
	import { holochainClient } from '$lib/stores/holochainClient';
	import ToastsList from '$lib/components/ToastsList.svelte';
	import Navbar from '$lib/components/Navbar.svelte';

	onMount(async () => {
		holochainClient.connect(new URL('ws://unused'), 'grant-funding');
	});

	setContext(clientContext, {
		getClient: () => $holochainClient.client
	});
</script>

{#if $holochainClient.isConnecting}
	<div class="flex h-screen w-full items-center justify-center">
		<Spinner class="h-14 w-14" />
	</div>
{:else}
	<div class="min-h-screen w-full bg-white dark:bg-gray-900">
		<Navbar />

		<main class="h-full w-full grow overflow-x-auto px-8">
			<slot />
		</main>

		<div class="fixed right-5 top-5 z-50 w-full max-w-md">
			<ToastsList />
		</div>
	</div>
{/if}
