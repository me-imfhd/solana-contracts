This comprises of some popular solana programs from basic to advance

Required Versions:
```
anchor-cli 0.30.1
solana-cli 1.18.17
```

Set solana config to localhost with
- ``` solana config set --url localhost ```

Run a local validator in a seperate terminal to test programs locally with
- ``` solana-test-validator ```

Steps:
1) `anchor build`: Builds all programs
2) `anchor deploy`: Deploys all programs and generated program IDs for each.
3) Update the program IDs for each program with the corresponding generated program ID.
4) `anchor test --detach --skip-local-validator`: Runs all the tests on the locally running validator.

Test by program name
- ```anchor test --program-name <PROGRAM-NAME> --detach --skip-local-validator```

