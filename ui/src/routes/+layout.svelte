<script lang="ts">
	import '@holochain-open-dev/profiles/dist/elements/profiles-context.js';
	import '@holochain-open-dev/profiles/dist/elements/create-profile.js';
	import '@holochain-open-dev/profiles/dist/elements/update-profile.js';
	import '@holochain-open-dev/profiles/dist/elements/profile-detail.js';
	import '@holochain-open-dev/profiles/dist/elements/search-agent.js';
	import '@shoelace-style/shoelace/dist/themes/light.css';
	import '@fontsource/saira-stencil-one';
	import '../app.pcss';
	import { onMount, setContext } from 'svelte';
	import { DarkMode, Spinner } from 'flowbite-svelte';
	import { clientContext } from '../contexts';
	import { holochainClient } from '$lib/stores/holochainClient';
	import BaseToastsList from '$lib/components/BaseToastsList.svelte';
	import BaseNavbar from '$lib/components/BaseNavbar.svelte';

	onMount(async () => {
		await holochainClient.connect(new URL('ws://unused'), 'grant-funding');
	});

	setContext(clientContext, {
		getClient: () => holochainClient
	});
</script>

<profiles-context store={$holochainClient.profilesStore}>
	<div class="flex min-h-screen w-full flex-col bg-white dark:bg-gray-900">
		{#if $holochainClient.isConnecting}
			<div class="flex h-screen w-full items-center justify-center">
				<Spinner class="h-14 w-14" />

				<!-- Include DarkMode component to ensure theme is fetched and applied during loading -->
				<div class="hidden"><DarkMode /></div>
			</div>
		{:else}
			<BaseNavbar />

			<main class="flex w-full flex-grow grow justify-center overflow-x-auto">
				<div class="h-full w-full max-w-screen-md pb-16">
					<slot />
				</div>
			</main>

			<div class="fixed right-5 top-5 z-50 w-full max-w-md">
				<BaseToastsList />
			</div>

			<div class="fixed bottom-1 right-1">
				<DarkMode
					btnClass="rounded text-gray-700 hover:bg-gray-100 md:hover:bg-transparent md:border-0 md:hover:text-primary-700 dark:text-gray-400 md:dark:hover:text-white dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent"
				/>
			</div>
		{/if}
	</div>
</profiles-context>
