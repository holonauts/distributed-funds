<script lang="ts">
	import { Input, Label } from 'flowbite-svelte';
	import type { NumberRange } from '../../grant_pools/grants/types';

	export let value: NumberRange = { min: 0, max: 10 };
</script>

<div class="flex items-center justify-start space-x-4">
	<div>
		<input
			type="number"
			id="min"
			bind:value={value.min}
			placeholder="0"
			class="focus:border-primary-500 focus:ring-primary-500 dark:focus:border-primary-500 dark:focus:ring-primary-500 block w-full rounded-lg border-gray-300 bg-gray-50 p-2.5 text-sm text-gray-900 disabled:cursor-not-allowed disabled:opacity-50 rtl:text-right dark:border-gray-500 dark:bg-gray-600 dark:text-white dark:placeholder-gray-400"
		/>
		<Label for="min" class="text-xs">Minimum</Label>
	</div>
	<div>
		<input
			type="number"
			id="max"
			bind:value={value.max}
			placeholder="100"
			class="focus:border-primary-500 focus:ring-primary-500 dark:focus:border-primary-500 dark:focus:ring-primary-500 block w-full rounded-lg border-gray-300 bg-gray-50 p-2.5 text-sm text-gray-900 disabled:cursor-not-allowed disabled:opacity-50 rtl:text-right dark:border-gray-500 dark:bg-gray-600 dark:text-white dark:placeholder-gray-400"
		/>
		<Label for="max" class="text-xs">Maximum</Label>
	</div>
</div>
