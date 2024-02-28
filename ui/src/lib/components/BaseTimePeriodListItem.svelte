<script lang="ts">
	import { formatTimestampLocal } from '$lib/utils/date';
	import type { TimePeriod } from '../../grant_pools/grants/types';
	export let timePeriod: TimePeriod;
</script>

<div class="flex w-full items-center justify-between">
	<div class="w-full">
		<div class="text-xs dark:text-white">
			{formatTimestampLocal(timePeriod.start_at)} -
			{formatTimestampLocal(timePeriod.end_at)}
		</div>
	</div>
</div>
