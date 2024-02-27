<script lang="ts">
	import BaseBreadcrumbs from '$lib/components/BaseBreadcrumbs.svelte';
	import { Button } from 'flowbite-svelte';
	import AllGrantPools from './AllGrantPools.svelte';
	import { PlusSolid } from 'flowbite-svelte-icons';
</script>

<BaseBreadcrumbs title="Grant Pools" />

<div class="flex justify-end">
	<Button href="/grant-pools/create">
		<PlusSolid class="mr-2 h-4 w-4" />
		Create Grant Pool
	</Button>
</div>

<div class="h-full w-full">
	<AllGrantPools />
</div>
