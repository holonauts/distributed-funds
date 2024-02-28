import { pad, toBytes } from 'viem';

export const bigintToU256 = (val: bigint) => pad(toBytes(val));
