# solana-connector

Maintainers: [Redstone](https://redstone.finance), piotrostr@

## Deployments

- Mainnet: `3oHtb7BCqjqhZt8LyqSAZRAubbrYy8xvDRaYoRghHB1T`
- Devnet: `3oHtb7BCqjqhZt8LyqSAZRAubbrYy8xvDRaYoRghHB1T`
- Testnet: `3oHtb7BCqjqhZt8LyqSAZRAubbrYy8xvDRaYoRghHB1T`

Anchor IDL: `./target/idl/redstone_sol.json`

`process_redstone_payload` method discriminator: `[49, 96, 127, 141, 118, 203, 237, 178]`

## Requirements
Before use you will need:
* rust
* solana-cli
* anchor-cli
* node

All instalation steps are described [here](https://solana.com/docs/intro/installation#install-dependencies)

install node packages:
```
npm install
```

## Build & Test
```
anchor build # just build the project
anchor test # builds and test the project
```

in case you see warnings during build try appending anchor with `RUSTUP_TOOLCHAIN="nightly-2024-11-19"` like so:
```
RUSTUP_TOOLCHAIN="nightly-2024-11-19" anchor test
```

See file `Anchor.toml` for more anchor scripts you can run.


## Tools versions
* anchor-cli 0.30.1
* solana-cli 2.1.4 (src:024d047e; feat:288566304, client:Agave)
* npm 10.9.0
* node v23.3.0

rust version managed by `rust-toolchain.toml`.

## Examples

Check out the `./pusher` directory for pushing data on-chain through
a serverless service

## License

BSL
