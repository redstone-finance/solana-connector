#!/bin/bash

echo "
anchor build

solana program write-buffer ./target/deploy/redstone_sol.so

solana program deploy --buffer <BUFFER_ADDRESS> --program-id <PROGRAM_ID> <PATH_TO_SO_FILE>
"
