<script lang="ts">
	import BaseBreadcrumbs from '$lib/components/BaseBreadcrumbs.svelte';
	import { PlusSolid } from 'flowbite-svelte-icons';
	import AllTimePeriods from './AllTimePeriods.svelte';
	import CreateTimePeriod from './CreateTimePeriod.svelte';
	import { Button, Modal } from 'flowbite-svelte';

	let showCreateModal = false;
</script>

<BaseBreadcrumbs title="Time Periods" />

<div class="mb-4 flex justify-end">
	<Button on:click={() => (showCreateModal = true)} color="green">
		<PlusSolid class="mr-2 h-4 w-4" />
		Create Time Period
	</Button>
</div>

<AllTimePeriods />

<Modal
	size="sm"
	outsideclose
	title="Create Time Period"
	bind:open={showCreateModal}
	class="overflow-visible"
>
	<CreateTimePeriod
		on:time-period-created={(e) => {
			showCreateModal = false;
		}}
	/>
</Modal>
