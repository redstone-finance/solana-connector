import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { RedstoneSol } from "../target/types/redstone_sol";
import * as fs from "fs";
import * as path from "path";

describe("redstone-sol", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.RedstoneSol as Program<RedstoneSol>;

  it("Processes Redstone payload successfully", async () => {
    // Read the payload from the file
    const payloadPath = path.join(__dirname, "..", "sample-payload.hex");
    const payloadHex = fs.readFileSync(payloadPath, "utf8").trim();
    const payload = Buffer.from(payloadHex, "hex");
    console.log("Payload:", payload.length, payload.byteLength);

    try {
      // Process the payload
      const tx = await program.methods
        .processRedstonePayload(payload.subarray(0, 800))
        .accounts({
          user: anchor.AnchorProvider.env().wallet.publicKey,
        })
        .rpc();

      console.log("Transaction signature:", tx);

      // If we reach this point without throwing an error, the test passed
      console.log("Payload processed successfully");
    } catch (error) {
      console.error("Error processing payload:", error);
      throw error; // Re-throw the error to fail the test
    }
  });
});
