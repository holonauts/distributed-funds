<script lang="ts" generics="T">
	import { decode } from '@msgpack/msgpack';
	import { type Record, type AppAgentCallZomeRequest } from '@holochain/client';
	import { holochainClient } from '$lib/stores/holochainClient';
	import { Card } from 'flowbite-svelte';

	export let callZomeRequest: AppAgentCallZomeRequest;
	let loading = true;
	let record: Record;
	// eslint-disable-next-line no-undef
	let entry: T;

	$: callZomeRequest, fetch();

	async function fetch() {
		loading = true;

		try {
			record = await $holochainClient.client.callZome(callZomeRequest);
			if (record) {
				// eslint-disable-next-line no-undef
				entry = decode(record.entry.Present.entry) as T;
			}
		} catch (e) {
			console.error(e);
		}

		loading = false;
	}
</script>

{#if loading}
	<div class="h-12 w-full animate-pulse rounded-md dark:bg-slate-800"></div>
{:else if record === undefined || entry === undefined}
	<div>Not found</div>
{:else}
	<slot {record} {entry} />
{/if}
