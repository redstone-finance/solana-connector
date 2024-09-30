# examples

> [!NOTE]
> The example uses the testnet cluster and program at
> `3oHtb7BCqjqhZt8LyqSAZRAubbrYy8xvDRaYoRghHB1T`

## Requirements

- Keypair generation and airdrop:
  - node and npm, used 20.15.0 and 10.7.0
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
npm install && npm run build
```

### Push Data

- Testnet

```bash
npm run start -- \
  --private-key ./example-keypair.json \
  --network testnet \
  --feed-id AVAX # or BTC, ETH, etc., any feed from redstone avax prod service
```

- Mainnet

```bash
npm run start -- \
  --private-key $HOME/.config/solana/id.json \
  --network mainnet-beta \
  --feed-id AVAX # or BTC, ETH, etc., any feed from redstone avax prod service
```

### Check Price

- Testnet

```bash
npm run start -- \
  --check-price AVAX \
  --network testnet
```

- Mainnet

```bash
npm run start -- \
  --check-price AVAX \
  --network mainnet-beta
```

### Push data continously from Google Cloud Run

> [!WARNING]
> Don't use a key with more than 0.1-0.2 SOL, the key is stored as environment
> variable by Cloud Run, in production setting it should be pulled from Google
> Cloud Secret Manager on every trigger

Set up `.env` as per `.env.example`:

```txt
PRIVATE_KEY=[base58-encoded-private-key]
RPC_URL=[mainnet-rpc-url]
```

Update the `PROJECT_ID` in the `./hack/ci-cd.sh` script, ensure you have cloud
build, cloud run GCP APIs enabled, then run

```sh
./hack/ci-cd.sh
```

The script parses the local env vars from `.env` and sets them in the Cloud Run
service, Cloud Build runs the build and deploys a new revision
