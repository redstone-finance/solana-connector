import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";
import { requestRedstonePayload } from "@redstone-finance/sdk";
import {
  Connection,
  Keypair,
  PublicKey,
  sendAndConfirmTransaction,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";

const METHOD_DISCRIMINATOR = [49, 96, 127, 141, 118, 203, 237, 178];
const REDSTONE_SOL_PROGRAM_ID = "3oHtb7BCqjqhZt8LyqSAZRAubbrYy8xvDRaYoRghHB1T";
const DATA_SERVICE_ID = "redstone-avalanche-prod";
const FEED_ID = "AVAX";
const DATA_FEEDS = [FEED_ID];
const UNIQUE_SIGNER_COUNT = 3;

if (!process.env.PRIVATE_KEY_PATH) {
  throw new Error("PRIVATE_KEY env variable is required");
}

let network = "testnet";

if (process.env.MAINNET && process.env.MAINNET === "true") {
  network = "mainnet-beta";
  console.log(
    "using mainnet-beta, consider using RPC provider for production!",
  );
}
const RPC_URL = `https://api.${network}.solana.com`;

const connection = new Connection(RPC_URL, "confirmed");

console.log(`connected to ${RPC_URL}, slot: ${await connection.getSlot()}`);

// slice the first 32 bytes of the seed to get the private key
const signer = Keypair.fromSeed(
  Uint8Array.from(await Bun.file(process.env.PRIVATE_KEY_PATH).json()).slice(
    0,
    32,
  ),
);

console.log("using signer:", signer.publicKey.toBase58());

const makePayload = async () => {
  const res = await requestRedstonePayload(
    {
      dataPackagesIds: DATA_FEEDS,
      dataServiceId: DATA_SERVICE_ID,
      uniqueSignersCount: UNIQUE_SIGNER_COUNT,
    },
    "bytes",
  );
  return Uint8Array.from(JSON.parse(res));
};

const makeFeedIdBytes = (feedId: string) => {
  return Buffer.from(feedId.padEnd(32, "\0"));
};

const makePriceSeed = () => {
  return Buffer.from("price".padEnd(32, "\0"));
};

const seeds = [makePriceSeed(), makeFeedIdBytes(FEED_ID)];
const [priceAccount, _] = PublicKey.findProgramAddressSync(
  seeds,
  new PublicKey(REDSTONE_SOL_PROGRAM_ID),
);

const keys = [
  { pubkey: signer.publicKey, isSigner: true, isWritable: true },
  { pubkey: priceAccount, isSigner: false, isWritable: true },
  { pubkey: SYSTEM_PROGRAM_ID, isSigner: false, isWritable: false },
];

const makeInstructionData = async () => {
  const payload = await makePayload();

  const sizeIndicator = Buffer.alloc(4);
  sizeIndicator.writeUInt32LE(payload.length);

  const data = Buffer.concat([
    Uint8Array.from(METHOD_DISCRIMINATOR),
    Uint8Array.from(makeFeedIdBytes(FEED_ID)),
    // size indicator is a crucial param since using a dynamic Vec<u8> for payload
    Uint8Array.from(sizeIndicator),
    payload,
  ]);

  return data;
};

console.log("Pushing data in a loop");
while (true) {
  try {
    const transaction = new Transaction().add(
      new TransactionInstruction({
        keys,
        programId: new PublicKey(REDSTONE_SOL_PROGRAM_ID),
        data: await makeInstructionData(),
      }),
    );
    try {
      const signature = await sendAndConfirmTransaction(
        connection,
        transaction,
        [signer],
      );
      console.log(`${Date.now()}: ${signature}`);
    } catch (error) {
      console.error("Error sending transaction:", error);
    }
  } catch (error) {
    console.error("Error making transaction:", error);
  }

  await sleep(60 * 1000);
}

function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}
