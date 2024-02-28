<script lang="ts">
	import { formatRecordTimestampHumanized, formatTimestampHumanized } from '$lib/utils/date';
	import type { Record } from '@holochain/client';
	import { Badge } from 'flowbite-svelte';

	export let record: Record;
</script>

<Badge color="blue">{formatRecordTimestampHumanized(record)}</Badge>
