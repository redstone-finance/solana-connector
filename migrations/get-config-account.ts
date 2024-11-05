import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { RedstoneSol } from "../target/types/redstone_sol";

async function getConfigAccount() {
  // Setup provider
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // Get program
  const program = anchor.workspace.RedstoneSol as Program<RedstoneSol>;

  // Get config account address
  const configAccount = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("config")],
    program.programId
  )[0];

  try {
    // Fetch config account data
    const configData = await program.account.configAccount.fetch(configAccount);

    console.log("Config Account Address:", configAccount.toString());
    console.log("Config Account Data:");
    console.log({
      signers: configData.signers,
      signerCountThreshold: configData.signerCountThreshold,
      maxTimestampDelayMs: configData.maxTimestampDelayMs.toString(),
      maxTimestampAheadMs: configData.maxTimestampAheadMs.toString(),
      owner: configData.owner.toString(),
    });
  } catch (error) {
    console.error("Error fetching config account:", error);
    throw error;
  }
}

// Run the function
getConfigAccount()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
