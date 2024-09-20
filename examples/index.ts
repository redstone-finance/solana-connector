import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import { requestRedstonePayload } from "@redstone-finance/sdk";
import {
  ComputeBudgetProgram,
  Connection,
  Keypair,
  PublicKey,
  sendAndConfirmTransaction,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";

if (!process.env.PRIVATE_KEY) {
  throw new Error("PRIVATE_KEY env variable is required");
}

const RPC_URL = "https://api.testnet.solana.com";

const connection = new Connection(RPC_URL, "confirmed");

console.log(`connected to ${RPC_URL}, slot: ${await connection.getSlot()}`);

const signer = Keypair.fromSecretKey(
  Uint8Array.from(bs58.decode(process.env.PRIVATE_KEY))
);

console.log("Using signer:", signer.publicKey.toBase58());

const METHOD_DISCRIMINATOR = [49, 96, 127, 141, 118, 203, 237, 178];
const REDSTONE_SOL_PROGRAM_ID = "redumH9C5NCb4bMUcf5SjE3ANkLSLMTx8L1WPmuHbAR";

const DATA_SERVICE_ID = "redstone-avalanche-prod";
const DATA_FEEDS = ["ETH"];
const UNIQUE_SIGNER_COUNT = 1; // testnet uses 1 but in prod subject to change

const makePayload = async () => {
  const res = await requestRedstonePayload(
    {
      dataPackagesIds: DATA_FEEDS,
      dataServiceId: DATA_SERVICE_ID,
      uniqueSignersCount: UNIQUE_SIGNER_COUNT,
    },
    "bytes"
  );

  const payload = Uint8Array.from(JSON.parse(res));

  console.log(`payload size: ${payload.length} bytes`);
  return payload;
};

const [ethPriceAccount, _] = PublicKey.findProgramAddressSync(
  [Buffer.from("price"), Buffer.from("ETH\0\0")],
  new PublicKey(REDSTONE_SOL_PROGRAM_ID)
);

const transaction = new Transaction()
  .add(ComputeBudgetProgram.setComputeUnitLimit({ units: 300000 }))
  .add(
    new TransactionInstruction({
      keys: [{ pubkey: ethPriceAccount, isSigner: false, isWritable: true }],
      programId: new PublicKey(REDSTONE_SOL_PROGRAM_ID),
      data: Buffer.concat([
        Uint8Array.from(METHOD_DISCRIMINATOR),
        await makePayload(),
      ]),
    })
  );

try {
  const signature = await sendAndConfirmTransaction(connection, transaction, [
    signer,
  ]);
  console.log("Transaction sent successfully. Signature:", signature);
} catch (error) {
  console.error("Error sending transaction:", error);
}
