import bs58 from "bs58";

export const base58ToBytes = (base58: string): Uint8Array => {
  return bs58.decode(base58);
};

export const bytesToBase58 = (bytes: Uint8Array): string => {
  return bs58.encode(bytes);
};
