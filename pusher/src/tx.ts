import bs58 from "bs58";
import { requestRedstonePayload } from "@redstone-finance/sdk";
import {
  Connection,
  Keypair,
  PublicKey,
  sendAndConfirmTransaction,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";
import {
  REDSTONE_SOL_PROGRAM_ID,
  SYSTEM_PROGRAM_ID,
  DATA_SERVICE_ID,
  UNIQUE_SIGNER_COUNT,
  METHOD_DISCRIMINATOR,
} from "./config.js";
import { makeFeedIdBytes, makePriceSeed } from "./util.js";

async function retry<T>(
  operation: () => Promise<T>,
  maxRetries: number = 2,
  initialBackoff: number = 100,
  backoffFactor: number = 2
): Promise<T> {
  let lastError: Error | undefined;
  for (let attempt = 0; attempt <= maxRetries; attempt++) {
    try {
      return await operation();
    } catch (error) {
      lastError = error as Error;
      if (attempt < maxRetries) {
        const backoffTime = initialBackoff * Math.pow(backoffFactor, attempt);
        await new Promise((resolve) => setTimeout(resolve, backoffTime));
      }
    }
  }
  throw lastError;
}

export async function makeTransaction(
  signer: Keypair,
  feedId: string
): Promise<Transaction> {
  const priceAccount = getPriceAccount(feedId);
  const keys = [
    { pubkey: signer.publicKey, isSigner: true, isWritable: true },
    { pubkey: priceAccount, isSigner: false, isWritable: true },
    {
      pubkey: new PublicKey(SYSTEM_PROGRAM_ID),
      isSigner: false,
      isWritable: false,
    },
  ];

  const instructionData = await makeInstructionData(feedId);

  return new Transaction().add(
    new TransactionInstruction({
      keys,
      programId: new PublicKey(REDSTONE_SOL_PROGRAM_ID),
      data: instructionData,
    })
  );
}

export async function sendTransaction(
  connection: Connection,
  transaction: Transaction,
  signer: Keypair
): Promise<string> {
  return await sendAndConfirmTransaction(connection, transaction, [signer]);
}

export async function sendTransactionWithJito(
  connection: Connection,
  transaction: Transaction,
  signer: Keypair
): Promise<string | undefined> {
  transaction.recentBlockhash = (
    await connection.getLatestBlockhash()
  ).blockhash;
  transaction.sign(signer);

  return await retry(async () => {
    const res = await fetch(
      "https://mainnet.block-engine.jito.wtf/api/v1/transactions",
      {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          jsonrpc: "2.0",
          id: 1,
          method: "sendTransaction",
          params: [bs58.encode(transaction.serialize())],
        }),
      }
    );
    if (!res.ok) {
      throw new Error(`HTTP error! status: ${res.status}`);
    }
    const data = await res.json();
    if (data.error) {
      throw new Error(`JSON-RPC error: ${JSON.stringify(data.error)}`);
    }
    return data?.result;
  });
}

async function makeInstructionData(feedId: string): Promise<Buffer> {
  const payload = await makePayload([feedId]);
  const sizeIndicator = Buffer.alloc(4);
  sizeIndicator.writeUInt32LE(payload.length);

  return Buffer.concat([
    Uint8Array.from(METHOD_DISCRIMINATOR),
    Uint8Array.from(makeFeedIdBytes(feedId)),
    Uint8Array.from(sizeIndicator),
    payload,
  ]);
}

async function makePayload(dataFeeds: string[]): Promise<Uint8Array> {
  const res = await requestRedstonePayload(
    {
      dataPackagesIds: dataFeeds,
      dataServiceId: DATA_SERVICE_ID,
      uniqueSignersCount: UNIQUE_SIGNER_COUNT,
    },
    "bytes"
  );
  return Uint8Array.from(JSON.parse(res));
}

function getPriceAccount(feedId: string): PublicKey {
  const seeds = [makePriceSeed(), makeFeedIdBytes(feedId)];
  const [priceAccount] = PublicKey.findProgramAddressSync(
    seeds,
    new PublicKey(REDSTONE_SOL_PROGRAM_ID)
  );
  return priceAccount;
}
