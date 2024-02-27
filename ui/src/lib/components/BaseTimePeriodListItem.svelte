<script lang="ts">
	import { formatTimestampHumanized } from '$lib/utils/date';
	import type { TimePeriod } from '../../grant_pools/grants/types';
	export let timePeriod: TimePeriod;
</script>

<div class="flex w-full items-center justify-between">
	<div class="w-full">
		<div class="text-xs dark:text-white">
			{formatTimestampHumanized(timePeriod.start_at)} -
			{formatTimestampHumanized(timePeriod.end_at)}
		</div>
	</div>
</div>
