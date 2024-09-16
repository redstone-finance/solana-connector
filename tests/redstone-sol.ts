import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { RedstoneSol } from "../target/types/redstone_sol";

const samplePayload = Buffer.from([
  69, 84, 72, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0, 53, 34, 200, 98, 183, 1, 145, 252, 191, 33, 48, 0, 0,
  0, 32, 0, 0, 1, 216, 103, 23, 5, 228, 117, 189, 252, 234, 151, 120, 207, 25,
  242, 15, 33, 124, 124, 110, 151, 99, 229, 225, 207, 115, 84, 115, 138, 69, 86,
  231, 198, 93, 34, 192, 61, 40, 34, 203, 68, 178, 178, 243, 137, 168, 138, 189,
  32, 248, 70, 96, 75, 177, 219, 188, 114, 42, 168, 58, 202, 2, 46, 204, 182,
  27, 69, 84, 72, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0, 0, 53, 34, 223, 68, 251, 1, 145, 252, 191, 33, 48, 0,
  0, 0, 32, 0, 0, 1, 30, 60, 221, 79, 57, 197, 206, 33, 243, 68, 100, 220, 52,
  237, 254, 209, 6, 15, 156, 156, 35, 138, 186, 16, 123, 8, 121, 155, 112, 142,
  142, 63, 116, 232, 223, 118, 30, 67, 54, 94, 215, 216, 72, 150, 250, 251, 5,
  222, 233, 216, 250, 139, 73, 138, 137, 129, 130, 214, 28, 52, 142, 100, 162,
  124, 27, 69, 84, 72, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 53, 34, 253, 200, 1, 1, 145, 252, 191, 33,
  48, 0, 0, 0, 32, 0, 0, 1, 250, 189, 170, 158, 79, 207, 211, 161, 225, 202, 79,
  64, 35, 231, 207, 158, 214, 125, 158, 248, 99, 149, 127, 162, 187, 159, 111,
  66, 192, 42, 211, 202, 70, 150, 1, 23, 101, 8, 218, 77, 213, 103, 126, 150,
  98, 35, 74, 44, 90, 52, 96, 97, 213, 137, 80, 66, 12, 124, 51, 77, 92, 184,
  30, 231, 28, 66, 84, 67, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 66, 64, 4, 61, 0, 1, 145, 252, 191, 33,
  48, 0, 0, 0, 32, 0, 0, 1, 66, 122, 39, 181, 18, 179, 108, 80, 105, 245, 79,
  202, 70, 148, 196, 203, 112, 119, 131, 28, 206, 109, 173, 29, 235, 242, 118,
  93, 158, 252, 203, 212, 83, 113, 41, 162, 6, 129, 69, 88, 84, 152, 25, 39,
  247, 232, 149, 111, 131, 250, 149, 195, 168, 218, 169, 68, 244, 117, 59, 106,
  1, 162, 69, 206, 27, 66, 84, 67, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 66, 64, 4, 61, 0, 1, 145, 252,
  191, 33, 48, 0, 0, 0, 32, 0, 0, 1, 113, 235, 232, 82, 254, 133, 36, 95, 235,
  36, 214, 92, 163, 49, 139, 77, 210, 49, 185, 181, 173, 112, 16, 211, 199, 203,
  1, 211, 207, 30, 72, 117, 8, 41, 38, 219, 159, 24, 20, 215, 160, 62, 165, 36,
  91, 108, 116, 252, 159, 203, 98, 122, 88, 229, 152, 72, 97, 113, 124, 161,
  120, 156, 168, 105, 27, 66, 84, 67, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 66, 64, 4, 61, 0, 1, 145, 252,
  191, 33, 48, 0, 0, 0, 32, 0, 0, 1, 126, 115, 33, 159, 116, 108, 112, 204, 53,
  98, 201, 97, 82, 237, 245, 221, 130, 138, 124, 184, 236, 30, 13, 160, 177,
  160, 197, 186, 186, 107, 58, 124, 92, 170, 180, 224, 197, 230, 21, 203, 213,
  107, 213, 240, 236, 213, 224, 81, 153, 143, 234, 246, 119, 57, 146, 15, 55,
  62, 202, 32, 40, 143, 103, 228, 27, 0, 6, 0, 0, 0, 0, 0, 2, 237, 87, 1, 30, 0,
  0,
]);

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

  let ethPriceAccount: anchor.web3.PublicKey;
  let btcPriceAccount: anchor.web3.PublicKey;
  let cbix: anchor.web3.TransactionInstruction;

  before(async () => {
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
        .processRedstonePayload(samplePayload)
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
        .processRedstonePayload(samplePayload)
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
