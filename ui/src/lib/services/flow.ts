import { writeContract } from '@wagmi/core';
import { config } from '$lib/utils/web3modal';
import FLOW_ABI from '$lib/abi/Flow.json';
import { cloneFlow } from "$lib/services/clonefactory";
import { prepareRainlangText } from '$lib/services/rain';
import { parseRainlangText } from '$lib/services/rain';

export async function createGrantPoolFlow(notaryAddress: `0x${string}`) {
  const rainlang = await prepareRainlangText(notaryAddress);
  const bytecode: Uint8Array = await parseRainlangText(rainlang);
  const flowAddress = await cloneFlow(bytecode);

  return flowAddress as `0x${string}`;
}

export async function deposit(amount: bigint, flow_contract_address: `0x${string}`) {
  const txHash = await writeContract(config, {
    abi: FLOW_ABI.abi,
    address: flow_contract_address,
    functionName: 'flow',
    args: [
      [amount]
    ],
  });

  return txHash;
}

export async function close(flow_contract_address: `0x${string}`) {
  await writeContract(config, {
    abi: FLOW_ABI.abi,
    address: flow_contract_address,
    functionName: 'close',
    args: [],
  });
}

export async function claim(flow_contract_address: `0x${string}`, signer: `0x${string}`, context: any[]) {
  await writeContract(config, {
    abi: FLOW_ABI.abi,
    address: flow_contract_address,
    functionName: 'close',
    args: [
      [],
      [signer],
      context
    ],
  });
}
