<script lang="ts">
	import { encodeHashToBase64, type AgentPubKey } from '@holochain/client';
	import '@holochain-open-dev/profiles/dist/elements/agent-avatar.js';
	import { holochainClient } from '$lib/stores/holochainClient';

	export let agentPubKey: AgentPubKey;

	$: ({ profiles } = $holochainClient.profilesStore);

	function formatAgentPubKeyShort(key: AgentPubKey) {
		let keyB64 = encodeHashToBase64(key);
		return `${keyB64.slice(0, 6)}...${keyB64.slice(-6)}`;
	}
</script>

<div class="flex items-center justify-start space-x-2">
	<agent-avatar {agentPubKey}></agent-avatar>
	<div>{profiles.get(agentPubKey)?.name || formatAgentPubKeyShort(agentPubKey)}</div>
</div>
