import { ACCEPTED_TOKEN_ADDRESS, RAIN_PARSER_ADDRESS } from "$lib/config";
import { RainLanguageServices, MetaStore, TextDocumentItem } from "@rainlanguage/dotrain";
import { writeContract } from '@wagmi/core';
import { config } from '$lib/utils/web3modal';
import { abi } from '$lib/abi/IParserV1.json';
import { toHex } from "viem";
import GRANTPOOL_FLOW_RAIN from '$lib/rain/grantpool-flow.rain?raw';

export async function prepareRainlangText(notaryAddress: string) {
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
  const rainlangText = await rainDocument.compose(["deposit", "close", "claim"]);

  return rainlangText;
}

export async function parseRainlangText(text: string): Promise<`0x${string}`> {
  const result = await writeContract(config, {
        address: RAIN_PARSER_ADDRESS,
        abi, 
        functionName: 'parse',
        args: [toHex(text)]
  });

  return result;
}
