<script lang="ts">
	import { Radio } from 'flowbite-svelte';

	export let value: string;
	export let group: string;
</script>

<div class="flex w-full items-start justify-start space-x-4">
	<Radio bind:group name="hash" {value} />
	<slot />
</div>
