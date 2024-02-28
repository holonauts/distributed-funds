<script lang="ts">
	import { Label } from 'flowbite-svelte';

	export let label: string;
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
</div>
