import { expect } from "chai";
import { getSignerFromPrivateKey } from "./config";

describe("config", () => {
  test("get signer from bs58 private key works", async () => {
    const privateKey =
      "5YJDF5Spe555QvbT6ouQSAyvXY1tox69YZjvJs8U2fZpYuddqHjZofHhwU1DnnvueDftLSQnBZYBYte63ucd5ULr";
    const pubkey = "7V7t14pFJUPnk8qHtLfik8cjjDoMhxZfqGJ7SVE14G98";
    const signer = getSignerFromPrivateKey(privateKey);
    expect(signer.publicKey.toString()).to.equal(pubkey);
  });
});
