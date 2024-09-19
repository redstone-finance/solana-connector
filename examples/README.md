# examples

This project was created using `bun init` in bun v1.1.21. [Bun](https://bun.sh) is a fast all-in-one JavaScript runtime.

Create `.env` with a `PRIVATE_KEY` set to a base58 encoded private key.

You can create one with a wallet like Phantom or Solflare or generate with
`solana-keygen generate`.

To get some Tesnet SOL, you can use the [Solana
Faucet](https://solfaucet.com/), or `solana airdrop 1 --url testnet`.

Then, you can

```bash
bun install
```

and

```bash
bun run index.ts
```
