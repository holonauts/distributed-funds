<script lang="ts">
	import { Badge } from 'flowbite-svelte';
	import dayjs from 'dayjs';
	import type { TimePeriod } from '../../grant_pools/grants/types';

	export let timePeriod: TimePeriod;
	export let activeText = 'Active';

	let now = dayjs().valueOf();
</script>

{#if now < timePeriod.start_at}
	<Badge large={true} color="blue">Upcoming</Badge>
{:else if now > timePeriod.start_at && now < timePeriod.end_at}
	<Badge large={true} color="green">{activeText}</Badge>
{:else}
	<Badge large={true} color="none">Completed</Badge>
{/if}
