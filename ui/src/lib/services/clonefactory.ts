import { RAIN_DEPLOYER_ADDRESS, RAIN_FLOW_ADDRESS, RAIN_PARSER_ADDRESS } from "$lib/config";
import { config } from '$lib/utils/web3modal';
import { abi } from '$lib/abi/CloneFactory.json';
import FLOW_ABI from '$lib/abi/Flow.json';

import { writeContract } from "@wagmi/core";
import { encodeAbiParameters } from "viem";

export async function cloneFlow(bytecodeHex: `0x${string}`) {  
  const encodedData = encodeAbiParameters(
    [
      {
        "name": "config",
        "type": "tuple[]",
        "components": [
          {
            "name": "deployer",
            "type": "address"
          },
          {
            "name": "bytecode",
            "type": "bytes"
          },
          {
            "name": "constants",
            "type": "uint256[]"
          }
        ],
      }
    ],
    [
      [
        {
          deployer: RAIN_DEPLOYER_ADDRESS, 
          bytecode: bytecodeHex, 
          constants: []
        }
      ]
    ]);

  const result = await writeContract(config, {
      address: RAIN_PARSER_ADDRESS,
      abi, 
      functionName: 'clone',
      args: [
        RAIN_FLOW_ADDRESS,
        encodedData
      ]
  });

  console.log("clone result", result);

  return result;
}