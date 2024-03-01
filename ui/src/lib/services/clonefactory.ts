import { FLOW_ADDRESS, RAIN_PARSER_ADDRESS } from "$lib/config";
import { config } from '../utils/web3modal';
import { abi } from '$lib/abi/CloneFactory.json';
import { writeContract } from "@wagmi/core";


export async function cloneFlow(bytecode: Uint8Array) {
  const result = await writeContract(config, {
      address: RAIN_PARSER_ADDRESS,
      abi, 
      functionName: 'clone',
      args: [
        FLOW_ADDRESS,
        bytecode
      ]
  });

  return result[0];
}