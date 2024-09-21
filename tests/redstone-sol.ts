import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { RedstoneSol } from "../target/types/redstone_sol";
import { requestRedstonePayload } from "@redstone-finance/sdk";
import { expect } from "chai";

const makeFeedIdBytes = (feedId: string) => {
  return Buffer.from(feedId.padEnd(32, "\0"));
};

const makePriceSeed = () => {
  return Buffer.from("price".padEnd(32, "\0"));
};

const makePayload = async (dataPackagesIds: Array<string>) => {
  const DATA_SERVICE_ID = "redstone-avalanche-prod";
  const UNIQUE_SIGNER_COUNT = 3;

  const res = await requestRedstonePayload(
    {
      dataPackagesIds,
      dataServiceId: DATA_SERVICE_ID,
      uniqueSignersCount: UNIQUE_SIGNER_COUNT,
    },
    "bytes"
  );

  const payload = Buffer.from(JSON.parse(res));

  console.log("Payload size:", payload.length);

  return payload;
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

// Utility function to print compute units used by a transaction
async function printComputeUnitsUsed(
  provider: anchor.AnchorProvider,
  txSignature: string
) {
  const maxRetries = 5;
  const cooldownMs = 200;

  for (let attempt = 1; attempt <= maxRetries; attempt++) {
    try {
      const tx = await provider.connection.getTransaction(txSignature, {
        maxSupportedTransactionVersion: 0,
        commitment: "confirmed",
      });
      if (tx && tx.meta && tx.meta.computeUnitsConsumed) {
        console.log(`Compute units used: ${tx.meta.computeUnitsConsumed}`);
        return; // Success, exit the function
      }
    } catch (error) {
      // pass
    }

    if (attempt < maxRetries) {
      await new Promise((resolve) => setTimeout(resolve, cooldownMs));
    }
  }

  console.log(`Failed to retrieve compute units after ${maxRetries} attempts`);
}

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
