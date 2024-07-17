This comprises of some popular solana programs from basic to advance

Required Versions:
```
anchor-cli 0.30.1
solana-cli 1.18.17
```

Set solana config to localhost with
- ``` solana config set --url localhost ```

## Building
1) `anchor build`: Builds all programs
2) Update the program IDs for each program with the corresponding generated program ID.

## Running Tests
Test by program name
- ```anchor test --detach --program-name <PROGRAM-NAME>```

## Logging
Look at program logs in `.anchor/program-logs`
Or use `solana logs`
Also set skipPreflight to false to see logs in case of failures in sending transaction.

If facing any issue, try deleting target dir and revert any changes to cargo.lock and retry, else feel free to create an issue to discuss.