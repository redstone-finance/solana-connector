export function makeFeedIdBytes(feedId: string): Buffer {
  return Buffer.from(feedId.padEnd(32, "\0"));
}

export function makePriceSeed(): Buffer {
  return Buffer.from("price".padEnd(32, "\0"));
}

export function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}
