<script lang="ts">
	import { onMount } from 'svelte';
	import { Radio } from 'flowbite-svelte';
	import { decode } from '@msgpack/msgpack';
	import { type Record, type ActionHash, encodeHashToBase64 } from '@holochain/client';
	import type { EvaluationTemplate, TimePeriod } from '../../../grant_pools/grants/types';
	import { holochainClient } from '$lib/stores/holochainClient';
	import TimePeriodListItem from '$lib/components/TimePeriodListItem.svelte';

	export let timePeriodHash: ActionHash;
	export let group: string;

	let loading = true;
	let record: Record | undefined;
	let timePeriod: TimePeriod | undefined;

	onMount(async () => {
		if (timePeriodHash === undefined) {
			throw new Error(
				`The timePeriodHash input is required for the EvaluationTemplateDetail element`
			);
		}
		await fetchEvaluationTemplate();
	});

	async function fetchEvaluationTemplate() {
		loading = true;
		record = undefined;
		timePeriod = undefined;

		try {
			record = await $holochainClient.client.callZome({
				cap_secret: null,
				role_name: 'grant_pools',
				zome_name: 'grants',
				fn_name: 'get_evaluation_template',
				payload: timePeriodHash
			});
			if (record) {
				timePeriod = decode((record.entry as any).Present.entry) as EvaluationTemplate;
			}
		} catch (e) {
			console.error(e);
		}

		loading = false;
	}
</script>

{#if loading}
	<div class="h-8 w-full animate-pulse rounded-md"></div>
{:else if record === undefined || timePeriod === undefined}
	<div>Time Period not found</div>
{:else}
	<div class="flex w-full items-start justify-start">
		<div class="p-4">
			<Radio bind:group name="hash" value={encodeHashToBase64(timePeriodHash)} />
		</div>
		<TimePeriodListItem {timePeriodHash} />
	</div>
{/if}
