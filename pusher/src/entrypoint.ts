import { serve } from "bun";
import {
  getSignerFromPrivateKey,
  makeTransaction,
  sendTransaction,
  getConnectionFromRpcUrl,
} from "./";

const FEED_ID = "AVAX";

async function pushData() {
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

// Schedule data push every 10 seconds
setInterval(pushData, 10000);

const port = parseInt(process.env.PORT || "8080", 10);

serve({
  port,
  fetch(req) {
    const url = new URL(req.url);

    if (url.pathname === "/") {
      return new Response("Service is running");
    }

    if (url.pathname === "/push") {
      // Trigger a manual push
      pushData();
      return new Response("Data push triggered");
    }

    return new Response("Not Found", { status: 404 });
  },
});
