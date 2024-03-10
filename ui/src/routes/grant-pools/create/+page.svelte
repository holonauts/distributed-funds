<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import {
		type Record,
		type ActionHash,
		type AgentPubKey,
		encodeHashToBase64
	} from '@holochain/client';
	import type { AmountRangeBigInt, GrantPool } from '../../../grant_pools/grants/types';
	import { Textarea, Input } from 'flowbite-svelte';
	import { toasts } from '$lib/stores/toast';
	import InputApplicationTemplate from './InputApplicationTemplate.svelte';
	import InputEvaluationTemplate from './InputEvaluationTemplate.svelte';
	import { holochainClient } from '$lib/stores/holochainClient';
	import BaseBreadcrumbs from '$lib/components/BaseBreadcrumbs.svelte';
	import SelectTimePeriod from './InputTimePeriod.svelte';
	import InputAgents from '$lib/components/InputAgents.svelte';
	import InputTokenAmountRange from '$lib/components/InputTokenAmountRange.svelte';
	import { bigintToU256 } from '$lib/utils/u256';
	import { goto } from '$app/navigation';
	import BaseLabelContent from '$lib/components/BaseLabelContent.svelte';
	import BaseHelper from '$lib/components/BaseHelper.svelte';
	import { config } from '$lib/utils/web3modal';
	import InputTokenAmount from '$lib/components/InputTokenAmount.svelte';
	import { ACCEPTED_TOKEN_SYMBOL, CHAIN } from '$lib/config';
	import { getAccount } from '@wagmi/core';
	import { createGrantPoolFlow, deposit } from '$lib/services/flow';
	import { approve } from '$lib/services/erc20';
	import BaseButtonLoading from '$lib/components/BaseButtonLoading.svelte';
	const dispatch = createEventDispatcher();

	export let name: string = '';
	export let purposeDescription: string = '';
	export let rulesDescription: string = '';
	export let timePeriod: ActionHash | undefined = undefined;
	export let amountRange: AmountRangeBigInt | undefined = undefined;
	export let applicationTemplate: ActionHash | undefined = undefined;
	export let evaluationTemplate: ActionHash | undefined = undefined;
	export let evaluators: AgentPubKey[] = [];
	export let depositAmount: bigint = 0n;
	let isCreating = false;

	$: isGrantPoolValid =
		name.length > 0 &&
		purposeDescription.length > 0 &&
		rulesDescription.length > 0 &&
		timePeriod !== undefined &&
		amountRange !== undefined &&
		applicationTemplate !== undefined &&
		evaluationTemplate !== undefined &&
		evaluators.length > 0 &&
		depositAmount !== undefined &&
		depositAmount > 0n;

	async function createGrantPool() {
		isCreating = true;

		try {
			// read wallet address
			const notaryEvmWallet = getAccount(config).address;
			if (notaryEvmWallet === undefined) {
				throw Error('Connect an EVM Wallert');
			}

			// deploy flow contract
			toasts.info('Transaction 1/3 (Deploy Grant Pool Contract): Awaiting signature');
			const flowEvm = await createGrantPoolFlow(notaryEvmWallet);

			// create grant pool entry
			const grantPoolEntry: GrantPool = {
				name: name!,
				purpose_description: purposeDescription!,
				rules_description: rulesDescription!,
				time_period: timePeriod!,
				application_template: applicationTemplate!,
				evaluation_template: evaluationTemplate!,
				amount_range: {
					min: bigintToU256(amountRange!.min),
					max: bigintToU256(amountRange!.max)
				},
				evaluators: evaluators!,
				notary_evm_wallet: notaryEvmWallet,
				flow_evm: flowEvm
			};
			console.log('grant pool entry is ', grantPoolEntry);

			const record: Record = await $holochainClient.client.callZome({
				cap_secret: null,
				role_name: 'grant_pools',
				zome_name: 'grants',
				fn_name: 'create_grant_pool',
				payload: grantPoolEntry
			});

			// call erc20 approve
			toasts.info(
				`Transaction 2/2 (Approve Spending ${ACCEPTED_TOKEN_SYMBOL}): Awaiting signature`
			);
			await approve(depositAmount, flowEvm.flow_clone_address);

			// call flow deposit
			toasts.info(`Transaction 3/3 (Deposit ${ACCEPTED_TOKEN_SYMBOL}): Awaiting signature`);
			const depositTxHash = await deposit(
				depositAmount,
				flowEvm.flow_clone_address,
				flowEvm.deposit_expression_address
			);

			// add grant pool <-> sponsor link
			await $holochainClient.client.callZome({
				cap_secret: null,
				role_name: 'grant_pools',
				zome_name: 'grants',
				fn_name: 'add_grant_pool_for_sponsor',
				payload: {
					base_sponsor: $holochainClient.client.myPubKey,
					target_grant_pool_hash: record.signed_action.hashed.hash,
					amount: bigintToU256(depositAmount),
					transaction_hash: depositTxHash
				}
			});

			dispatch('grant-pool-created', { grantPoolHash: record.signed_action.hashed.hash });
			goto(`/grant-pools/${encodeHashToBase64(record.signed_action.hashed.hash)}`);
		} catch (e) {
			toasts.error(`Error creating the grant pool: ${e}`);
		}

		isCreating = false;
	}
</script>

<BaseBreadcrumbs title="Create" />

<BaseLabelContent label="Title" class="mb-8">
	<Input id="name" bind:value={name} required placeholder="Coral Reef Renewal" />
</BaseLabelContent>

<BaseLabelContent label="Purpose" class="mb-8">
	<Textarea
		id="purpose-description"
		class="h-48"
		bind:value={purposeDescription}
		required
		placeholder="To support projects that are effectively improving the health of coral reefs in the south pacific."
	/>
	<BaseHelper>The mission of this grants pool, and what it aims to accomplish.</BaseHelper>
</BaseLabelContent>

<BaseLabelContent label="Rules & Eligability Criteria" class="mb-8">
	<Textarea
		id="rules-description"
		class="h-48"
		bind:value={rulesDescription}
		required
		placeholder="All projects must be based in the east. Have no more than 500K revenue per year."
	/>
	<BaseHelper>The rules and eligability requirements for grant applications.</BaseHelper>
</BaseLabelContent>

<div class="mb-8">
	<InputApplicationTemplate bind:value={applicationTemplate} />
</div>

<div class="mb-8">
	<InputEvaluationTemplate bind:value={evaluationTemplate} />
</div>

<BaseLabelContent label="Grant Funding Range" class="mb-8">
	<InputTokenAmountRange bind:value={amountRange} />
	<BaseHelper>The amount of funding that may be awarded in a single grant.</BaseHelper>
</BaseLabelContent>

<div class="mb-8">
	<SelectTimePeriod bind:value={timePeriod} />
</div>

<div class="mb-8">
	<BaseLabelContent label="Evaluators">
		<InputAgents bind:value={evaluators} />
	</BaseLabelContent>
	<BaseHelper>The people invited to evaluate and score grant applications.</BaseHelper>
</div>

<BaseLabelContent
	label="Initial Deposit"
	helper={`The amount of ${ACCEPTED_TOKEN_SYMBOL} to deposit into the grant pool before creating. Additional funding can be contributed after grant pool is created.`}
	class="mb-8"
>
	<InputTokenAmount bind:value={depositAmount} />
</BaseLabelContent>

<BaseLabelContent
	label="Wallet"
	helper={`The EVM wallet address that will manage the grant pool vault. Grant pool vaults are deployed to ${CHAIN.name} and must have sufficient ${CHAIN.nativeCurrency.name} to pay gas fees.`}
	class="mb-8"
>
	<w3m-button />
</BaseLabelContent>

<div class="flex justify-end">
	<BaseButtonLoading
		disabled={!isGrantPoolValid}
		on:click={createGrantPool}
		color="green"
		loading={isCreating}
	>
		Create
	</BaseButtonLoading>
</div>
