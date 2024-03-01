import { ACCEPTED_TOKEN_ADDRESS, RAIN_PARSER_ADDRESS } from "$lib/config";
import { RainLanguageServices, MetaStore, TextDocumentItem } from "@rainlanguage/dotrain";
import { readContracts } from '@wagmi/core';
import { config } from '$lib/utils/web3modal';
import PARSER_ABI from '$lib/abi/IParserV2.json';
import { toBytes } from "viem";

export async function prepareRainlangText(notaryAddress: string) {
  const res = await fetch('/rain/grantpool-flow.rain')
  const grantPoolFlow = await res.text();

  // instantiate a MetaStore which is a in-memory CAS for Rain metadata
  const metaStore = new MetaStore();

  // some text document
  const textDocument = TextDocumentItem.create(
    "file:///grantpool-flow.rain",
    "rainlang",
    0,
    grantPoolFlow,
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

export async function parseRainlangText(text: string): Promise<Uint8Array> {
  const result = await readContracts(config, {
    contracts: [
      {
        address: RAIN_PARSER_ADDRESS,
        abi: PARSER_ABI, 
        functionName: 'iParser',
        args: [toBytes(text)]
      },
    ],
  });

  return result[0].result;
}
