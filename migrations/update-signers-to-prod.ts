import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { RedstoneSol } from "../target/types/redstone_sol";
import { PRIMARY_SIGNERS } from "./signers";

// Setup function to initialize the program and get the config account
async function setup() {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.RedstoneSol as Program<RedstoneSol>;

  const configAccount = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("config")],
    program.programId
  )[0];

  return { program, provider, configAccount };
}

async function updateSigners() {
  const { program, provider, configAccount } = await setup();

  try {
    // Update the signers while keeping other configuration values unchanged
    const tx = await program.methods
      .updateConfig(
        PRIMARY_SIGNERS, // New signers array
        null, // Keep existing threshold
        null, // Keep existing max delay
        null // Keep existing max ahead
      )
      .accountsStrict({
        owner: provider.wallet.publicKey,
        configAccount,
      })
      .rpc();

    console.log("Transaction signature:", tx);
    console.log("Successfully updated signers");
  } catch (error) {
    console.error("Error updating signers:", error);
    throw error;
  }
}

updateSigners()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
