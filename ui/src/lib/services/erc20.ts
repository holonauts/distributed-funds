import { getPublicClient, getWalletClient, writeContract } from '@wagmi/core'
import { config } from '$lib/utils/web3modal';
import { abi } from '$lib/abi/IERC20.json';
import { ACCEPTED_TOKEN_ADDRESS } from '$lib/config';
import { getContract } from 'viem';

export async function approve(amount: bigint, flow_contract_address: string) {
  const erc20Contract = await getErc20Contract(ACCEPTED_TOKEN_ADDRESS);
  const client = await getWalletClient(config);
  const currentAllowance = await erc20Contract.read.allowance([
    client.account.address,
    flow_contract_address,
  ]);
  
  const additionalAllowance = amount - currentAllowance;
  if(additionalAllowance === 0n) return;

  const txHash = await erc20Contract.write.approve([
    flow_contract_address,
    additionalAllowance,
  ]);
  console.log('erc20 approve txhash', txHash);

  return txHash;
}

const getErc20Contract = async (address: `0x${string}`) => getContract({
  address,
  abi,
  client: {
    wallet: await getWalletClient(config),
    public: await getPublicClient(config)
  }
});