import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { RedstoneSol } from "../target/types/redstone_sol";
import { requestRedstonePayload } from "@redstone-finance/sdk";

const makePayload = async () => {
  const DATA_SERVICE_ID = "redstone-avalanche-prod";
  const DATA_FEEDS = ["ETH", "BTC"];
  const UNIQUE_SIGNER_COUNT = 3;

  const res = await requestRedstonePayload(
    {
      dataPackagesIds: DATA_FEEDS,
      dataServiceId: DATA_SERVICE_ID,
      uniqueSignersCount: UNIQUE_SIGNER_COUNT,
    },
    "bytes"
  );

  return Buffer.from(JSON.parse(res));
};

function deserializePriceData(data: Buffer): PriceData {
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
}

interface PriceData {
  feedId: string;
  value: string;
  timestamp: string;
}

describe("redstone-sol", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.RedstoneSol as Program<RedstoneSol>;

  let payload: Buffer;
  let ethPriceAccount: anchor.web3.PublicKey;
  let btcPriceAccount: anchor.web3.PublicKey;
  let cbix: anchor.web3.TransactionInstruction;

  before(async () => {
    payload = await makePayload();
    // Derive price account addresses
    [ethPriceAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("price"), Buffer.from("ETH\0\0")],
      program.programId
    );

    [btcPriceAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("price"), Buffer.from("BTC\0\0")],
      program.programId
    );

    // Set up compute budget instruction
    cbix = anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({
      units: 10 ** 6,
    });
  });

  it("Processes Redstone payload successfully", async () => {
    try {
      // Process the payload
      const tx = await program.methods
        .processRedstonePayload(payload)
        .preInstructions([cbix])
        .accounts({
          user: provider.wallet.publicKey,
        })
        .rpc();

      console.log("Transaction signature:", tx);
    } catch (error) {
      console.error("Error processing payload:", error);
      throw error; // Re-throw the error to fail the test
    }
  });

  it("Updates price accounts correctly", async () => {
    try {
      // Process the payload
      const tx = await program.methods
        .processRedstonePayload(payload)
        .preInstructions([cbix])
        .accounts({
          user: provider.wallet.publicKey,
        })
        .rpc();

      console.log("Transaction signature:", tx);

      // fetch the accounts with provider
      const ethPriceData = await provider.connection.getAccountInfo(
        ethPriceAccount
      );
      const btcPriceData = await provider.connection.getAccountInfo(
        btcPriceAccount
      );
      console.log(
        "ETH Price Account Data:",
        deserializePriceData(ethPriceData!.data)
      );
      console.log(
        "BTC Price Account Data:",
        deserializePriceData(btcPriceData!.data)
      );
    } catch (error) {
      console.error(
        "Error processing payload and verifying price accounts:",
        error
      );
      throw error;
    }
  });
});
