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
	class="h-[30rem] overflow-visible"
	dialogClass="fixed top-0 start-0 end-0 h-modal md:inset-0 md:h-full z-50 w-full p-4 flex overflow-visible"
>
	<CreateTimePeriod
		on:time-period-created={(e) => {
			showCreateModal = false;
		}}
	/>
</Modal>
