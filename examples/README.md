# examples

> [!NOTE]
> The example uses the devnet cluster and program at `2tcbDvTs2LkKKx9xwizMHRBKxKgtWBihRnZoDnbxtc8k`

## Requirements:

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
solana airdrop 1 --url devnet --keypair example-keypair.json
```

Then create a `.env` file with the following content:

```sh
PRIVATE_KEY_PATH=./example-keypair.json
```

Then,

```bash
bun install
```

and

```bash
bun run index.ts
```
