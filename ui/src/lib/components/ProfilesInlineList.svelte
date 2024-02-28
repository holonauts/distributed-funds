<script lang="ts">
	import { type AgentPubKey, encodeHashToBase64 } from '@holochain/client';
	import ProfileInline from './ProfileInline.svelte';
	import { Button } from 'flowbite-svelte';
	import { goto } from '$app/navigation';

	export let agentPubKeys: AgentPubKey[];
</script>

<div class="flex flex-wrap space-x-2 space-y-2">
	{#each agentPubKeys as agentPubKey}
		<Button color="alternative" class="py-2" href={`/profiles/${encodeHashToBase64(agentPubKey)}`}>
			<ProfileInline {agentPubKey} />
		</Button>
	{/each}
</div>
