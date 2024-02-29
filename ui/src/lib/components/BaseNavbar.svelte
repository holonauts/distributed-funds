<script lang="ts">
	import { Navbar, NavBrand, NavUl, NavLi, Button } from 'flowbite-svelte';
	import '@holochain-open-dev/profiles/dist/elements/agent-avatar.js';
	import { page } from '$app/stores';
	import { holochainClient } from '$lib/stores/holochainClient';
	import { encodeHashToBase64 } from '@holochain/client';

	$: activeUrl = $page.url.pathname;
</script>

<div class="relative flex items-start justify-center">
	<Navbar let:hidden>
		<NavBrand href="/">
			<span
				class="font-stencil self-center whitespace-nowrap text-4xl font-semibold text-black dark:text-slate-300"
			>
				Distributed Funds
			</span>
		</NavBrand>
		<NavUl {hidden} {activeUrl} class="flex items-center">
			<div class="flex items-center space-x-8">
				<NavLi href="/time-periods">Time Periods</NavLi>
				<NavLi href="/grant-pools">Grant Pools</NavLi>
				<NavLi href="/my-applications">My Applications</NavLi>
				<NavLi href="/my-assigned-evaluations">My Evaluations</NavLi>
				<Button
					color="alternative"
					size="xs"
					href={`/profiles/${encodeHashToBase64($holochainClient.client.myPubKey)}`}
				>
					<agent-avatar disable-copy disable-tooltip agentPubKey={$holochainClient.client.myPubKey}
					></agent-avatar>
				</Button>
			</div>
		</NavUl>
	</Navbar>
</div>
