<script lang="ts">
	import { page } from '$app/stores';
	import CreateApplication from '$lib/components/CreateApplication.svelte';
	import { decodeHashFromBase64 } from '@holochain/client';

	$: actionHash = decodeHashFromBase64($page.params.actionHashB64);
</script>

<CreateApplication {actionHash} />
