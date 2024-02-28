<script lang="ts">
	import { type AgentPubKey } from '@holochain/client';
	import { isEqual } from 'lodash';
	import ProfileInline from './ProfileInline.svelte';
	import { Badge, Button } from 'flowbite-svelte';
	import { CloseOutline } from 'flowbite-svelte-icons';
	export let value: AgentPubKey[] = [];

	function add(agentPubKey: AgentPubKey) {
		if (value.filter((v) => isEqual(v, agentPubKey)).length > 0) return;
		value = [...value, agentPubKey];
	}

	function remove(agentPubKey: AgentPubKey) {
		value = value.filter((v) => !isEqual(v, agentPubKey));
	}
</script>

<div class="input-agents-wrapper">
	<search-agent
		class="relative -top-4"
		field-label=" "
		clear-on-select
		include-myself={false}
		on:agent-selected={(e) => add(e.detail.agentPubKey)}
	></search-agent>

	<div class="mt-2 flex flex-wrap space-x-2 space-y-2">
		{#each value as agentPubKey}
			<Badge color="none" class="py-2">
				<div class="flex items-center justify-between space-x-4">
					<ProfileInline {agentPubKey} />
					<Button class="p-0" size="xs" color="alternative">
						<CloseOutline class="h-4 w-4" on:click={() => remove(agentPubKey)} />
					</Button>
				</div>
			</Badge>
		{/each}
	</div>
</div>

<style>
	.input-agents-wrapper :global(textarea) {
		border-radius: 0.5rem !important;
	}

	.input-agents-wrapper :global(input) {
		border-radius: 0.5rem !important;
	}

	.input-agents-wrapper :global(label) {
		--tw-text-opacity: 1;
		color: rgb(156 163 175 / var(--tw-text-opacity)) !important;
	}
</style>
