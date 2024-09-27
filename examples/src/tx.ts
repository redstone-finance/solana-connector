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

export async function makeTransaction(
  signer: Keypair,
  feedId: string,
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
    }),
  );
}

export async function sendTransaction(
  connection: Connection,
  transaction: Transaction,
  signer: Keypair,
): Promise<string> {
  return await sendAndConfirmTransaction(connection, transaction, [signer]);
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
    "bytes",
  );
  return Uint8Array.from(JSON.parse(res));
}

function getPriceAccount(feedId: string): PublicKey {
  const seeds = [makePriceSeed(), makeFeedIdBytes(feedId)];
  const [priceAccount] = PublicKey.findProgramAddressSync(
    seeds,
    new PublicKey(REDSTONE_SOL_PROGRAM_ID),
  );
  return priceAccount;
}
