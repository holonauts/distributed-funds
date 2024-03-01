import { writeContract } from '@wagmi/core'
import { config } from '$lib/utils/web3modal';
import { abi } from '$lib/abi/IERC20.json';
import { ACCEPTED_TOKEN_ADDRESS } from '$lib/config';

export async function approve(amount: bigint, flow_contract_address: string) {
  const txHash = await writeContract(config, {
    abi,
    address: ACCEPTED_TOKEN_ADDRESS,
    functionName: 'approve',
    args: [
      flow_contract_address,
      amount,
    ],
  });

  return txHash;
}