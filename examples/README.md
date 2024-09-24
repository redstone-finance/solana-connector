# examples

> [!NOTE]
> The example uses the testnet cluster and program at `2tcbDvTs2LkKKx9xwizMHRBKxKgtWBihRnZoDnbxtc8k`

## Requirements

- `bun` v1.1.21. [Bun](https://bun.sh)
- Keypair generation and airdrop:
  - `solana` CLI ^1.18.17
    [Solana](https://docs.solana.com/cli/install-solana-cli-tools)
  - `jq` for parsing JSON

## Usage

You need to create a keypair

```sh
solana-keygen new -o example-keypair.json
```

Grab some SOL from the faucet

```sh
solana airdrop 1 --url testnet --keypair example-keypair.json
```

For mainnet use, the keypair has to hold some real SOL, pushing a single feed
ID with 3 signers costs like 0.000005 SOL

To run for multiple feeds, start a separate process for each, max 1 feed ID per
process

Then,

```bash
bun install
```

### Push Data

- Testnet

```bash
bun run index.ts \
  --private-key ./example-keypair.json \
  --network testnet \
  --feed-id AVAX # or BTC, ETH, etc., any feed from redstone avax prod service
```

- Mainnet

```bash
bun run index.ts \
  --private-key $HOME/.config/solana/id.json \
  --network mainnet-beta \
  --feed-id AVAX # or BTC, ETH, etc., any feed from redstone avax prod service
```

### Check Price

- Testnet

```bash
bun run index.ts \
  --check-price AVAX \
  --network testnet
```

- Mainnet

```bash
bun run index.ts \
  --check-price AVAX \
  --network mainnet
```
