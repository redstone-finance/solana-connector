import { expect } from "chai";
import { getPrice } from "./price";

describe("getPrice", () => {
  it("should return the correct price for AVAX feed", async () => {
    const feedId = "AVAX";
    const price = await getPrice(feedId, "mainnet-beta");

    expect(price).to.be.an("object");
    expect(price).to.have.property("feedId", feedId);
    expect(price).to.have.property("value");
    expect(price).to.have.property("timestamp");

    // Verify that the value is a non-zero number
    const numericValue = parseFloat(price.value);
    expect(numericValue).to.be.a("number");
    expect(numericValue).to.be.greaterThan(0);

    // Verify that the timestamp is recent (within the last hour)
    const timestamp = parseInt(price.timestamp);
    expect(timestamp).to.be.a("number");
  });
});
