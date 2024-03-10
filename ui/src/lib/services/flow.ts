import { prepareRainlang, parseRainlang, cloneFlow } from '$lib/services/rain';
import { RAIN_INTERPRETER_ADDRESS, RAIN_STORE_ADDRESS } from '$lib/config';
import { FlowCloneEvm } from '../../grant_pools/grants/types';
import { RAIN_DEPLOYER_ADDRESS, RAIN_FLOW_ADDRESS, CLONE_FACTORY_ADDRESS } from "$lib/config";
import { getPublicClient, getWalletClient, waitForTransactionReceipt } from '@wagmi/core';
import { config } from '$lib/utils/web3modal';
import CLONE_FACTORY_ABI from '$lib/abi/CloneFactory.json';
import FLOW_ABI from '$lib/abi/Flow.json';
import RAIN_DEPLOYER_ABI from '$lib/abi/RainterpreterExpressionDeployerNPE2.json';
import { encodeAbiParameters, getContract, pad, parseEventLogs, toHex } from "viem";
import { RainlangParsed } from './rain';

export async function createGrantPoolFlow(notaryAddress: `0x${string}`): Promise<FlowCloneEvm> {
  // compose dotrain
  const { deposit, close, claim } = await prepareRainlang(notaryAddress);
  
  // parse rainlang
  const depositParsed = await parseRainlang(deposit);
  const closeParsed = await parseRainlang(close);
  const claimParsed = await parseRainlang(claim);

  // clone contract
  const flowCloneEvm = await cloneFlow(depositParsed, closeParsed, claimParsed);

  return flowCloneEvm;
}

export async function deposit(amount: bigint, flow_clone_address: `0x${string}`, deposit_expression_address: `0x${string}`): Promise<`0x${string}`> {
  const flowContract = await getFlowContract(flow_clone_address);
  const res = await flowContract.simulate.flow([
    [
      RAIN_INTERPRETER_ADDRESS,
      RAIN_STORE_ADDRESS,
      deposit_expression_address,
    ],
    [pad(toHex(amount))],
    [],
  ]);
  console.log('simulation result', res);

  const txHash = await flowContract.write.flow([
    [
      RAIN_INTERPRETER_ADDRESS,
      RAIN_STORE_ADDRESS,
      deposit_expression_address,
    ],
    [pad(toHex(amount))],
    [],
  ]);
  return txHash as `0x${string}`;
}

export async function close(flow_clone_address: `0x${string}`, close_expression_address: `0x${string}`) {
  const flowContract = await getFlowContract(flow_clone_address);
  const txHash = await flowContract.write.flow([
    [
      RAIN_INTERPRETER_ADDRESS,
      RAIN_STORE_ADDRESS,
      close_expression_address,
    ],
    [],
    []
  ]);
  return txHash as `0x${string}`;
}

export async function claim(flow_clone_address: `0x${string}`, claim_expression_address: `0x${string}`, signer: `0x${string}`, context: any[], signature: `0x${string}`) {
  const flowContract = await getFlowContract(flow_clone_address);
  const txHash = await flowContract.write.flow([
    [
      RAIN_INTERPRETER_ADDRESS,
      RAIN_STORE_ADDRESS,
      claim_expression_address,
    ],
    [],
    [
      [signer, context, signature]
    ],
  ]);
  return txHash as `0x${string}`;
}

const getFlowContract = async (address: `0x${string}`) => getContract({
  address,
  abi: FLOW_ABI.abi,
  client: {
    wallet: await getWalletClient(config),
    public: await getPublicClient(config)
  }
});


export async function cloneFlow(depositParsed: RainlangParsed, closeParsed: RainlangParsed, claimParsed: RainlangParsed): Promise<FlowCloneEvm> {  
  const cloneFactoryContract = getContract({
    address: CLONE_FACTORY_ADDRESS,
    abi: CLONE_FACTORY_ABI.abi,
    client: {
      wallet: await getWalletClient(config),
      public: await getPublicClient(config)
    }
  });

  const encodedData = encodeAbiParameters(
    FLOW_ABI.abi[1].inputs,
    [
      [
          {
            deployer: RAIN_DEPLOYER_ADDRESS, 
            ...depositParsed
          },
          {
            deployer: RAIN_DEPLOYER_ADDRESS, 
            ...closeParsed
          },
          {
            deployer: RAIN_DEPLOYER_ADDRESS, 
            ...claimParsed
          }
      ]
    ]
  );

  const { result: flowCloneAddress } = await cloneFactoryContract.simulate.clone([RAIN_FLOW_ADDRESS, encodedData]);
  const txHash = await cloneFactoryContract.write.clone([RAIN_FLOW_ADDRESS, encodedData]);

  // Get addresses of deployed expressions  
  const txReceipt = await waitForTransactionReceipt(config, {hash: txHash});

  const deployedExpressions = parseEventLogs({
      abi: RAIN_DEPLOYER_ABI.abi,
      logs: txReceipt.logs
    })
    .filter((event) =>  event.eventName === 'DeployedExpression')
    .map((event) => event.args.expression);
  if(deployedExpressions.length !== 3) throw Error("Flow cloning did not deploy 3 expressions");

  return {
    flow_clone_address: flowCloneAddress,
    deposit_expression_address: deployedExpressions[0],
    close_expression_address: deployedExpressions[1],
    claim_expression_address: deployedExpressions[2],
  };
}