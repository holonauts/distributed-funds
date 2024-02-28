<script lang="ts">
	import { Radio } from 'flowbite-svelte';
	import BaseBordered from '$lib/components/BaseBordered.svelte';

	export let value: string;
	export let group: string;
</script>

<BaseBordered>
	<div class="flex w-full items-start justify-start space-x-4">
		<Radio bind:group name="hash" {value} />
		<div class="flex-1">
			<slot />
		</div>
	</div>
</BaseBordered>
