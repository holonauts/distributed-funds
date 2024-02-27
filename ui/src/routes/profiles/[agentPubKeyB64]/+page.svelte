<script lang="ts">
	import { page } from '$app/stores';
	import { holochainClient } from '$lib/stores/holochainClient';
	import '@holochain-open-dev/profiles/dist/elements/create-profile.js';
	import '@holochain-open-dev/profiles/dist/elements/update-profile.js';
	import '@holochain-open-dev/profiles/dist/elements/profile-detail.js';
	import '@shoelace-style/shoelace/dist/themes/light.css';
	import { decodeHashFromBase64 } from '@holochain/client';
	import { isEqual } from 'lodash';
	import BaseBreadcrumbs from '$lib/components/BaseBreadcrumbs.svelte';
	import { Card } from 'flowbite-svelte';

	$: agentPubKey = decodeHashFromBase64($page.params.agentPubKeyB64);
	$: ({ myProfile } = $holochainClient.profilesStore);
</script>

<BaseBreadcrumbs title="Agent Profile" />

<div class="flex justify-center">
	<div>
		{#if isEqual(agentPubKey, $holochainClient.client.myPubKey)}
			{#if $myProfile.value !== undefined}
				<Card>
					<update-profile></update-profile>
				</Card>
			{:else}
				<create-profile></create-profile>
			{/if}
		{:else}
			<profile-detail {agentPubKey} />
		{/if}
	</div>
</div>
