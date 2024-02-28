import { pad, toBytes, fromBytes} from 'viem';

export const bigintToU256 = (val: bigint) => pad(toBytes(val));

export const u256ToBigint = (val: Uint8Array) => fromBytes(val, "bigint");
