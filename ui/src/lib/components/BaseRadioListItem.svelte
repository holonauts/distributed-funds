<script lang="ts">
	import { Radio } from 'flowbite-svelte';

	export let value: string;
	export let group: string;
</script>

<div
	class="relative w-full rounded-lg border-[1px] border-solid border-slate-900 px-2 py-4 dark:border-white"
>
	<div class="flex w-full items-start justify-start space-x-4">
		<Radio bind:group name="hash" {value} />
		<div class="flex-1">
			<slot />
		</div>
	</div>
</div>
