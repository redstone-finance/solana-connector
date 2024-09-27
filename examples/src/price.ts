import { PublicKey } from "@solana/web3.js";
import { getConnection, REDSTONE_SOL_PROGRAM_ID } from "./config.js";
import {
  makeFeedIdBytes,
  makePriceSeed,
  deserializePriceData,
  type PriceData,
} from "./util.js";

export async function getPrice(
  feedId: string,
  network: string,
): Promise<PriceData> {
  const connection = await getConnection(network);
  const [priceAccount] = PublicKey.findProgramAddressSync(
    [makePriceSeed(), makeFeedIdBytes(feedId)],
    new PublicKey(REDSTONE_SOL_PROGRAM_ID),
  );
  const acc = await connection.getAccountInfo(priceAccount);
  if (!acc) {
    throw new Error("Price account not found");
  }
  return deserializePriceData(acc.data);
}
