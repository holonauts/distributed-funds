<script lang="ts">
	import BaseBreadcrumbs from '$lib/components/BaseBreadcrumbs.svelte';
	import { Button } from 'flowbite-svelte';
	import AllGrantPools from './AllGrantPools.svelte';
	import { PlusSolid } from 'flowbite-svelte-icons';
</script>

<BaseBreadcrumbs title="Grant Pools" />

<div class="mb-4 flex justify-end">
	<Button href="/grant-pools/create" color="green">
		<PlusSolid class="mr-2 h-4 w-4" />
		Create Grant Pool
	</Button>
</div>

<AllGrantPools />
