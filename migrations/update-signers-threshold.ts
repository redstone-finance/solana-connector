import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { RedstoneSol } from "../target/types/redstone_sol";

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

async function updateSignerThreshold(newThreshold: number) {
  const { program, provider, configAccount } = await setup();

  try {
    // Update only the signer threshold, keeping other values the same
    const tx = await program.methods
      .updateConfig(
        null, // Keep existing signers
        newThreshold, // New threshold
        null, // Keep existing max delay
        null // Keep existing max ahead
      )
      .accountsStrict({
        owner: provider.wallet.publicKey,
        configAccount,
      })
      .rpc();

    console.log("Transaction signature:", tx);
    console.log(`Successfully updated signer threshold to ${newThreshold}`);
  } catch (error) {
    console.error("Error updating signer threshold:", error);
    throw error;
  }
}

// Get the threshold from command line arguments
const newThreshold = parseInt(process.argv[2]);

if (isNaN(newThreshold)) {
  console.error("Please provide a valid number for the new signer threshold");
  process.exit(1);
}

updateSignerThreshold(newThreshold)
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
