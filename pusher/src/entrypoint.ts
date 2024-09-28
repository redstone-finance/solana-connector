import {
  getSignerFromPrivateKey,
  makeTransaction,
  sendTransaction,
  getConnectionFromRpcUrl,
} from "./";

const FEED_ID = "AVAX";

export async function entrypoint() {
  const rpcUrl = process.env.RPC_URL;
  const privateKey = process.env.PRIVATE_KEY;

  if (!rpcUrl || !privateKey) {
    console.error("Missing RPC_URL or PRIVATE_KEY");
    return;
  }

  try {
    const connection = await getConnectionFromRpcUrl(rpcUrl);
    const signer = getSignerFromPrivateKey(privateKey);

    console.log("Using signer:", signer.publicKey.toBase58());
    try {
      const transaction = await makeTransaction(signer, FEED_ID);
      const signature = await sendTransaction(connection, transaction, signer);
      console.log(`${Date.now()}: Transaction sent: ${signature}`);
    } catch (error) {
      console.error("Error in transaction:", error);
    }
  } catch (error) {
    console.error("Fatal error:", error);
  }
}

entrypoint();
