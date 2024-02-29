<script lang="ts">
	import { Label } from 'flowbite-svelte';
	import BaseHelper from '$lib/components/BaseHelper.svelte';

	export let label: string;
	export let helper: string = '';
</script>

<div class="mb-2" {...$$props}>
	<Label
		class="block text-lg font-bold uppercase text-slate-400 rtl:text-right dark:text-slate-600"
	>
		{label}
	</Label>

	<div class="text-md block font-medium text-gray-900 rtl:text-right dark:text-gray-300">
		<slot />
	</div>

	{#if helper.length > 0}
		<div class="mt-2">
			<BaseHelper>{helper}</BaseHelper>
		</div>
	{/if}
</div>
