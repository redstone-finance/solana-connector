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
  if (data.length !== 80) {
    // 8 (discriminator) + 32 + 32 + 8 bytes
    throw new Error("Invalid data length for PriceData " + data.length);
  }

  const feedIdBuffer = data.subarray(8, 40);
  const feedId = feedIdBuffer.toString("utf8").replace(/\0+$/, "");
  const valueBuffer = data.subarray(40, 72);
  const value = BigInt(`0x${valueBuffer.toString("hex")}`);
  const timestamp = data.readBigUInt64LE(72);

  return {
    feedId: feedId,
    value: value.toString(),
    timestamp: timestamp.toString(),
  };
};
