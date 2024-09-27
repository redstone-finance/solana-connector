import { getConnection, getSigner, setupProgram } from "./config.js";
import { getPrice } from "./price.js";
import { makeTransaction, sendTransaction } from "./tx.js";
import { sleep } from "./util.js";

async function main() {
  const program = setupProgram();
  const options = program.opts();

  if (options.checkPrice) {
    const price = await getPrice(options.checkPrice, options.network);
    console.log(price);
    return;
  }

  if (!options.privateKey || !options.feedId || !options.network) {
    console.log(program.helpInformation());
    process.exit(1);
  }

  const connection = await getConnection(options.network);
  const signer = await getSigner(options.privateKey);

  console.log("Using signer:", signer.publicKey.toBase58());

  console.log("Pushing data in a loop");
  while (true) {
    try {
      const transaction = await makeTransaction(signer, options.feedId);
      const signature = await sendTransaction(connection, transaction, signer);
      console.log(`${Date.now()}: ${signature}`);
    } catch (error) {
      console.error("Error:", error);
    }

    await sleep(60 * 1000);
  }
}

main().catch(console.error);
