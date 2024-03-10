import { ACCEPTED_TOKEN_ADDRESS, RAIN_PARSER_ADDRESS } from "$lib/config";
import { RainLanguageServices, MetaStore, TextDocumentItem } from "@rainlanguage/dotrain";
import { getPublicClient, getWalletClient } from '@wagmi/core';
import { config } from '$lib/utils/web3modal';
import PARSER_ABI from '$lib/abi/IParserV1.json';
import { getContract, toHex } from "viem";
import GRANTPOOL_FLOW_RAIN from '$lib/rain/grantpool-flow.rain?raw';

export interface FlowRainlangSources {
  deposit: string,
  close: string,
  claim: string
};

export interface RainlangParsed {
  bytecode: `0x${string}`,
  constants: `0x${string}`[],
}

export async function prepareRainlang(notaryAddress: string): Promise<FlowRainlangSources>{
  // instantiate a MetaStore which is a in-memory CAS for Rain metadata
  const metaStore = new MetaStore();

  // some text document
  const textDocument = TextDocumentItem.create(
    "file:///grantpool-flow.rain",
    "rainlang",
    0,
    GRANTPOOL_FLOW_RAIN,
  );

  // initiating the services (metaStore is optional)
  const langServices = new RainLanguageServices(metaStore);

  // instantiate a new RainDocument
  const rainDocument = await langServices.newRainDocument(textDocument, [
    ['notary-address', notaryAddress],
    ['payment-token', ACCEPTED_TOKEN_ADDRESS]
  ])

  // composing a RainDocument to get rainlang string
  const rainlangSources = {
    deposit: await rainDocument.compose(["deposit"]),
    close: await rainDocument.compose(["close"]),
    claim: await rainDocument.compose(["claim"])
  };

  return rainlangSources;
}

export async function parseRainlang(text: string): Promise<RainlangParsed> {
  const parserContract = getContract({
    address: RAIN_PARSER_ADDRESS,
    abi: PARSER_ABI.abi,
    client: {
      wallet: await getWalletClient(config),
      public: await getPublicClient(config)
    }
  });
  const [bytecode, constants] = await parserContract.read.parse([toHex(text)]);
  
  return {
    bytecode,
    constants
  }
}
