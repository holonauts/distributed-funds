<script lang="ts">
	import { Badge } from 'flowbite-svelte';
	import { StatusType, type Status } from '../../grant_pools/grants/types';

	export let status: Status;
</script>

{#if status.type === StatusType.Draft}
	<Badge large={true} color="none">Draft</Badge>
{:else if status.type === StatusType.Submitted}
	<Badge large={true} color="blue">Submitted</Badge>
{:else if status.type === StatusType.Evaluated}
	<Badge large={true} color="blue">Evaluated</Badge>
{:else if status.type === StatusType.Claimed}
	<Badge large={true} color="green">Funded</Badge>
{/if}
