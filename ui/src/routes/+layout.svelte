<script lang="ts">
	import '../app.pcss';
	import { onMount, setContext } from 'svelte';
	import { DarkMode, Spinner } from 'flowbite-svelte';
	import { clientContext } from '../contexts';
	import { holochainClient } from '$lib/stores/holochainClient';
	import BaseToastsList from '$lib/components/BaseToastsList.svelte';
	import BaseNavbar from '$lib/components/BaseNavbar.svelte';

	onMount(() => {
		holochainClient.connect(new URL('ws://unused'), 'grant-funding');
	});

	setContext(clientContext, {
		getClient: () => holochainClient
	});
</script>

<div class="min-h-screen w-full bg-white dark:bg-gray-900">
	{#if $holochainClient.isConnecting}
		<div class="flex min-h-screen w-full items-center justify-center">
			<Spinner class="h-14 w-14" />

			<!-- Include DarkMode component to ensure theme is fetched and applied during loading -->
			<div class="hidden"><DarkMode /></div>
		</div>
	{:else}
		<BaseNavbar />

		<main class="flex h-full w-full grow justify-center overflow-x-auto">
			<div class="w-full max-w-screen-md py-8">
				<slot />
			</div>
		</main>

		<div class="fixed right-5 top-5 z-50 w-full max-w-md">
			<BaseToastsList />
		</div>
	{/if}
</div>
