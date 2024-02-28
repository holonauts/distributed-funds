<script lang="ts">
	import { type ActionHash } from '@holochain/client';
	import RecordDetail from './RecordDetail.svelte';
	import { Badge } from 'flowbite-svelte';
	import dayjs from 'dayjs';

	export let timePeriodHash: ActionHash;
	export let activeText = 'Active';

	let now = dayjs().valueOf();
</script>

<RecordDetail
	callZomeRequest={{
		cap_secret: null,
		role_name: 'grant_pools',
		zome_name: 'grants',
		fn_name: 'get_time_period',
		payload: timePeriodHash
	}}
>
	<svelte:fragment let:entry>
		{#if now < entry.start_at}
			<Badge large={true} color="blue">Upcoming</Badge>
		{:else if now > entry.start_at && now < entry.end_at}
			<Badge large={true} color="green">{activeText}</Badge>
		{:else}
			<Badge large={true} color="none">Completed</Badge>
		{/if}
	</svelte:fragment>
</RecordDetail>
