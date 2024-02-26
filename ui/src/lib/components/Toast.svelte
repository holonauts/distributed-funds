<script lang="ts">
	import { Toast } from 'flowbite-svelte';
	import {
		CheckCircleSolid,
		CloseSolid,
		ExclamationCircleSolid,
		InfoCircleSolid
	} from 'flowbite-svelte-icons';
	import type { ToastData } from '$lib/stores/toast';

	export let toast: ToastData;
</script>

<div class="mt-2">
	{#if toast.message_type === 'Success'}
		<Toast color="green">
			<CheckCircleSolid slot="icon" class="h-5 w-5" />
			<span class={toast.break_text ? 'break-all' : ''}>{toast.text}</span>
		</Toast>
	{:else if toast.message_type === 'Error'}
		<Toast color="red">
			<CloseSolid slot="icon" class="h-5 w-5" />
			<span class={toast.break_text ? 'break-all' : ''}>{toast.text}</span>
		</Toast>
	{:else if toast.message_type === 'Warning'}
		<Toast color="yellow">
			<ExclamationCircleSolid slot="icon" class="h-5 w-5" />
			<span class={toast.break_text ? 'break-all' : ''}>{toast.text}</span>
		</Toast>
	{:else}
		<Toast color="blue">
			<InfoCircleSolid slot="icon" class="h-5 w-5" />
			<span class={toast.break_text ? 'break-all' : ''}>{toast.text}</span>
		</Toast>
	{/if}
</div>
