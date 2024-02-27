<script lang="ts" generics="T">
	import { Spinner, Alert } from 'flowbite-svelte';

	export let loading = true;

	// eslint-disable-next-line no-undef
	export let hashes: T[] = [];
</script>

{#if loading}
	<div class="flex h-full min-h-64 w-full items-center justify-center">
		<Spinner class="h-8 w-8" />
	</div>
{:else if hashes.length === 0}
	<Alert class="my-8 text-center">None found.</Alert>
{:else}
	<div class="flex flex-col">
		{#each hashes as hash}
			<div class="mb-8">
				<slot {hash} />
			</div>
		{/each}
	</div>
{/if}
