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

  before(async () => {
    for (const feedId of feedIds) {
      pdas[feedId] = anchor.web3.PublicKey.findProgramAddressSync(
        [makePriceSeed(), makeFeedIdBytes(feedId)],
        program.programId
      )[0];
    }
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
          systemProgram: anchor.web3.SystemProgram.programId,
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
});
