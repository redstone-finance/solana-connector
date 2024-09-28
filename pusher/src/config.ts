import bs58 from "bs58";
import { Connection, Keypair } from "@solana/web3.js";
import { Command } from "commander";
import { readFile } from "fs/promises";

export const SYSTEM_PROGRAM_ID = "11111111111111111111111111111111";
export const METHOD_DISCRIMINATOR = [49, 96, 127, 141, 118, 203, 237, 178];
export const REDSTONE_SOL_PROGRAM_ID =
  "3oHtb7BCqjqhZt8LyqSAZRAubbrYy8xvDRaYoRghHB1T";
export const DATA_SERVICE_ID = "redstone-avalanche-prod";
export const UNIQUE_SIGNER_COUNT = 3;

export function setupProgram(): Command {
  return new Command()
    .option("-k, --private-key <path>", "Path to the private key file")
    .option(
      "-n, --network <network>",
      "Network to use (testnet or mainnet-beta)",
      "testnet",
    )
    .option("-f, --feed-id <id>", "Feed ID to use", "AVAX")
    .option("-c, --check-price <id>", "Check price for a given feed ID")
    .parse(process.argv);
}

export async function getConnection(network: string): Promise<Connection> {
  let RPC_URL = `https://api.${network}.solana.com`;
  if (network === "mainnet-beta" && process.env.RPC_URL) {
    RPC_URL = process.env.RPC_URL;
  }

  const connection = new Connection(RPC_URL, "confirmed");
  console.log(`Connected to ${RPC_URL}, slot: ${await connection.getSlot()}`);
  return connection;
}

export async function getConnectionFromRpcUrl(
  rpcUrl: string,
): Promise<Connection> {
  const connection = new Connection(rpcUrl, "confirmed");
  console.log(`Connected to ${rpcUrl}, slot: ${await connection.getSlot()}`);
  return connection;
}

export async function getSigner(privateKeyPath: string): Promise<Keypair> {
  return Keypair.fromSeed(
    Uint8Array.from(JSON.parse(await readFile(privateKeyPath, "utf-8"))).slice(
      0,
      32,
    ),
  );
}

export function getSignerFromPrivateKey(privateKey: string): Keypair {
  const decodedPrivateKey = bs58.decode(privateKey);
  return Keypair.fromSecretKey(Uint8Array.from(decodedPrivateKey));
}
