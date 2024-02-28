<script lang="ts">
	import { type ActionHash, encodeHashToBase64 } from '@holochain/client';
	import TimePeriodListItem from '$lib/components/TimePeriodListItem.svelte';
	import BaseRadioListItem from '$lib/components/BaseRadioListItem.svelte';

	export let timePeriodHash: ActionHash;
	export let group: string;
</script>

<BaseRadioListItem bind:group value={encodeHashToBase64(timePeriodHash)}>
	<TimePeriodListItem {timePeriodHash} />
</BaseRadioListItem>
