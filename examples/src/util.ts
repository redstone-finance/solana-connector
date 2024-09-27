export function makeFeedIdBytes(feedId: string): Buffer {
  return Buffer.from(feedId.padEnd(32, "\0"));
}

export function makePriceSeed(): Buffer {
  return Buffer.from("price".padEnd(32, "\0"));
}

export function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

export interface PriceData {
  feedId: string;
  value: string;
  timestamp: string;
}

export const deserializePriceData = (data: Buffer): PriceData => {
  if (data.length !== 64) {
    // 8 (discriminator) + 32 + 16 + 8 bytes
    throw new Error("Invalid data length for PriceData " + data.length);
  }

  const feedIdBuffer = data.subarray(8, 40);
  const feedId = feedIdBuffer.toString("utf8").replace(/\0+$/, "");
  const valueLow = data.readBigUInt64LE(40);
  const valueHigh = data.readBigUInt64LE(48);
  const value = valueLow + (valueHigh << BigInt(64));
  const timestamp = data.readBigUInt64LE(56);

  return {
    feedId: feedId,
    value: value.toString(),
    timestamp: timestamp.toString(),
  };
};
