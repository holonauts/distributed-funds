<script lang="ts">
	import { DateInput } from 'date-picker-svelte';
	import { Button, Label } from 'flowbite-svelte';
	import type { TimePeriod } from '../../grant_pools/grants/types';
	import { holochainClient } from '$lib/stores/holochainClient';
	import { toasts } from '$lib/stores/toast';
	import { createEventDispatcher } from 'svelte';
	import { type Record } from '@holochain/client';
	import dayjs from 'dayjs';
	const dispatch = createEventDispatcher();

	let startAt: Date | undefined;
	let endAt: Date;

	$: minEndAt = startAt ? startAt : new Date();
	$: isValid = startAt && endAt && endAt > startAt;

	async function createTimePeriod() {
		const timePeriodEntry: TimePeriod = {
			start_at: dayjs(startAt).valueOf()!,
			end_at: dayjs(endAt).valueOf()!
		};

		try {
			const record: Record = await $holochainClient.client.callZome({
				cap_secret: null,
				role_name: 'grant_pools',
				zome_name: 'grants',
				fn_name: 'create_time_period',
				payload: timePeriodEntry
			});
			dispatch('time-period-created', { timePeriodHash: record.signed_action.hashed.hash });
		} catch (e) {
			toasts.error(`Error creating the time period: ${e}`);
		}
	}
</script>

<div class="flex flex-col">
	<Label class="mb-1 text-lg">Application Period</Label>

	<div class="flex items-center justify-start space-x-8">
		<div>
			<DateInput min={new Date()} bind:value={startAt} />
			<Label>Starts At</Label>
		</div>

		<div>
			<DateInput min={minEndAt} bind:value={endAt} />
			<Label>Ends At</Label>
		</div>
	</div>

	<div class="flex justify-end">
		<Button color="green" on:click={createTimePeriod} disabled={!isValid}>Create</Button>
	</div>
</div>
