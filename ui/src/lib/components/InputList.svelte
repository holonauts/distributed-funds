<script lang="ts" generics="T">
	import { Button, Listgroup, ListgroupItem } from 'flowbite-svelte';
	import { CloseOutline, PlusOutline } from 'flowbite-svelte-icons';
	import { createEventDispatcher } from 'svelte';
	const dispatch = createEventDispatcher();

	// eslint-disable-next-line svelte/valid-compile, no-undef
	export let items: T[] = [];
	export let createIsValid = false;

	let showCreate = true;

	function remove(index: number) {
		items = items.filter((a, i) => i !== index);
	}
</script>

{#if items.length === 0}
	<p>None</p>
{:else}
	<Listgroup>
		{#each items as item, index}
			<ListgroupItem class="w-full py-1">
				<div class="flex items-center justify-between">
					<div class="flex-grow">
						<slot name="item" {item} {index} />
					</div>

					<Button class="p-1" color="alternative" on:click={() => remove(index)}>
						<CloseOutline class="h-4 w-4" />
					</Button>
				</div>
			</ListgroupItem>
		{/each}
	</Listgroup>
{/if}

{#if showCreate}
	<div class="mt-8 rounded-lg bg-black/15 p-4">
		<div class="mb-4">
			<slot name="create" />
		</div>
		<div class="flex justify-end space-x-4">
			<Button size="xs" class="p-2" color="alternative" on:click={() => (showCreate = false)}>
				Cancel
			</Button>
			<Button
				size="xs"
				class="p-2"
				color="green"
				disabled={!createIsValid}
				on:click={() => {
					dispatch('add');
					showCreate = false;
				}}>Save</Button
			>
		</div>
	</div>
{:else}
	<div class="mt-4 flex justify-end">
		<Button size="xs" class="p-2" color="green" on:click={() => (showCreate = true)}>
			<PlusOutline class="mr-2 h-4 w-4" /> New
		</Button>
	</div>
{/if}
