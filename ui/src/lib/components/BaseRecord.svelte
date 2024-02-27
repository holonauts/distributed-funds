<script lang="ts" generics="T">
	import type { Record } from '@holochain/client';

	export let loading = true;
	// eslint-disable-next-line no-undef
	export let entry: T | undefined = undefined;
	export let record: Record | undefined = undefined;
</script>

{#if loading}
	<div style="w-full h-8 rounded-lg animate-pulse bg-gray-200 dark:bg-gray-700"></div>
{:else if record === undefined || entry === undefined}
	<div>Not Found</div>
{:else}
	<slot {entry} {record} />
{/if}
