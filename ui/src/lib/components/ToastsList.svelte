<script lang="ts">
	import Toast from '$lib/components/Toast.svelte';
	import { toastsList } from '$lib/stores/toast';
</script>

{#each $toastsList as toast}
	<div class="flex justify-end">
		<Toast {toast} />
	</div>
{/each}
