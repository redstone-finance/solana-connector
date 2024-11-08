import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { RedstoneSol } from "../target/types/redstone_sol";
import { PRIMARY_SIGNERS } from "./signers";

async function main() {
  // Local env by default, ammend Anchor.toml for prod
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.RedstoneSol as Program<RedstoneSol>;

  const [configAccount, _] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("config")],
    program.programId
  );

  await program.methods
    .initialize(
      PRIMARY_SIGNERS,
      1, // signer_count_threshold
      new anchor.BN(15 * 60 * 1000), // max_timestamp_delay_ms (15 minutes)
      new anchor.BN(3 * 60 * 1000) // max_timestamp_ahead_ms (3 minutes)
    )
    .accountsStrict({
      owner: anchor.getProvider().publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
      configAccount,
    })
    .rpc();

  console.log("Config initialized at:", configAccount.toString());
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
