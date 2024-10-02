import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { RedstoneSol } from "../target/types/redstone_sol";
import { expect } from "chai";
import {
  printComputeUnitsUsed,
  makePayload,
  makePriceSeed,
  makeFeedIdBytes,
  deserializePriceData,
} from "./util";
import { SIGNERS } from "../migrations/signers";

describe("redstone-sol", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.RedstoneSol as Program<RedstoneSol>;

  const feedIds = [
    "AVAX",
    "BAL",
    "BAL_ggAVAX_AVAX",
    "BAL_sAVAX_AVAX",
    "BAL_yyAVAX_AVAX",
    "BTC",
    "CAI",
    "CRV",
    "DAI",
    "ETH",
    "EUROC",
    "GLP",
    "GMX",
    "GM_AVAX_WAVAX",
    "GM_AVAX_WAVAX_USDC",
    "GM_BTC_BTCb",
    "GM_BTC_BTCb_USDC",
    "GM_ETH_WETHe",
    "GM_ETH_WETHe_USDC",
    "GM_SOL_SOL_USDC",
    "IB01.L",
    "JOE",
    "LINK",
    "PNG",
    "PNG_AVAX_ETH_LP",
    "PNG_AVAX_USDC_LP",
    "PNG_AVAX_USDT_LP",
    "PRIME",
    "QI",
    "SHLB_GMX-AVAX_B",
    "SOL",
    "TJ_AVAX_USDC_AUTO",
    "USDC",
    "USDT",
    "WOMBAT_ggAVAX_AVAX_LP_AVAX",
    "WOMBAT_ggAVAX_AVAX_LP_ggAVAX",
    "WOMBAT_sAVAX_AVAX_LP_AVAX",
    "WOMBAT_sAVAX_AVAX_LP_sAVAX",
    "XAVA",
    "YYAV3SA1",
    "YY_AAVE_AVAX",
    "YY_GLP",
    "YY_PNG_AVAX_ETH_LP",
    "YY_PNG_AVAX_USDC_LP",
    "crvUSDBTCETH",
    "ggAVAX",
    "gmdAVAX",
    "gmdBTC",
    "gmdETH",
    "gmdUSDC",
    "sAVAX",
    "yyAVAX",
  ];

  let pdas = {};

  let configAccount: anchor.web3.PublicKey;

  const systemProgram = anchor.web3.SystemProgram.programId;

  before(async () => {
    for (const feedId of feedIds) {
      pdas[feedId] = anchor.web3.PublicKey.findProgramAddressSync(
        [makePriceSeed(), makeFeedIdBytes(feedId)],
        program.programId
      )[0];
    }

    configAccount = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("config")],
      program.programId
    )[0];
  });

  it("initializes correctly", async () => {
    await program.methods
      .initialize(
        SIGNERS,
        3, // signer_count_threshold
        new anchor.BN(15 * 60 * 1000), // max_timestamp_delay_ms (15 minutes)
        new anchor.BN(3 * 60 * 1000) // max_timestamp_ahead_ms (3 minutes)
      )
      .accountsStrict({
        owner: anchor.getProvider().publicKey,
        systemProgram,
        configAccount,
      })
      .rpc();

    console.log("Config initialized at:", configAccount.toString());
  });

  async function testFeedIdPush(feedId: string) {
    it(`Updates correctly for ${feedId} feed`, async () => {
      const payload = await makePayload([feedId]);
      const feedIdBytes = makeFeedIdBytes(feedId);
      const priceAccount = pdas[feedId];
      const tx = await program.methods
        .processRedstonePayload(Array.from(feedIdBytes), payload)
        .accountsStrict({
          user: provider.wallet.publicKey,
          priceAccount,
          configAccount,
          systemProgram,
        })
        .rpc({ skipPreflight: true });

      await printComputeUnitsUsed(provider, tx);

      const priceAccountData = deserializePriceData(
        (await provider.connection.getAccountInfo(priceAccount)).data
      );

      expect(priceAccountData.feedId).to.equal(feedId);
      expect(priceAccountData.value).to.not.equal("0");

      console.log(`${feedId}: ${JSON.stringify(priceAccountData)}`);
    });
  }

  feedIds.forEach(testFeedIdPush);

  describe("Config updates", () => {
    it("Owner can update the config", async () => {
      const newSigners = SIGNERS.slice(0, 5); // Use first 5 signers
      const newThreshold = 2;
      const newMaxDelay = new anchor.BN(20 * 60 * 1000); // 20 minutes
      const newMaxAhead = new anchor.BN(5 * 60 * 1000); // 5 minutes

      await program.methods
        .updateConfig(newSigners, newThreshold, newMaxDelay, newMaxAhead)
        .accountsStrict({
          owner: provider.wallet.publicKey,
          configAccount,
        })
        .rpc();

      const updatedConfig = await program.account.configAccount.fetch(
        configAccount
      );

      expect(updatedConfig.signers).to.deep.equal(newSigners);
      expect(updatedConfig.signerCountThreshold).to.equal(newThreshold);
      expect(updatedConfig.maxTimestampDelayMs.toString()).to.equal(
        newMaxDelay.toString()
      );
      expect(updatedConfig.maxTimestampAheadMs.toString()).to.equal(
        newMaxAhead.toString()
      );
    });

    it("Non-owner cannot update the config", async () => {
      const nonOwnerWallet = anchor.web3.Keypair.generate();
      await provider.connection.requestAirdrop(
        nonOwnerWallet.publicKey,
        1000000000
      );

      try {
        await program.methods
          .updateConfig(null, 4, null, null)
          .accountsStrict({
            owner: nonOwnerWallet.publicKey,
            configAccount,
          })
          .signers([nonOwnerWallet])
          .rpc();
        expect.fail("Expected error but transaction succeeded");
      } catch (error) {
        expect(error.toString()).to.include(
          "A has one constraint was violated"
        );
      }
    });

    it("Partial update of config is possible", async () => {
      const originalConfig = await program.account.configAccount.fetch(
        configAccount
      );
      const newMaxDelay = new anchor.BN(25 * 60 * 1000); // 25 minutes

      await program.methods
        .updateConfig(null, null, newMaxDelay, null)
        .accountsStrict({
          owner: provider.wallet.publicKey,
          configAccount,
        })
        .rpc();

      const updatedConfig = await program.account.configAccount.fetch(
        configAccount
      );

      expect(updatedConfig.signers).to.deep.equal(originalConfig.signers);
      expect(updatedConfig.signerCountThreshold).to.equal(
        originalConfig.signerCountThreshold
      );
      expect(updatedConfig.maxTimestampDelayMs.toNumber()).to.equal(
        newMaxDelay.toNumber()
      );
      expect(updatedConfig.maxTimestampAheadMs.toNumber()).to.equal(
        originalConfig.maxTimestampAheadMs.toNumber()
      );
    });
  });
});
