This comprises of some popular solana programs from basic to advance

Required Versions:
```
anchor-cli 0.30.1
solana-cli 1.18.17
```

Set solana config to localhost with
- ``` solana config set --url localhost ```

Steps:
1) `anchor build`: Builds all programs
2) `anchor deploy`: Deploys all programs and generated program IDs for each.
3) Update (if not done automatically) the program IDs for each program with the corresponding generated program ID.
4) `anchor test --detach`: Runs all the tests.

Test by program name
- ```anchor test --program-name <PROGRAM-NAME> --detach```

If facing any issue, try deleting target/deploy dir and restart from step number 1

Look at program logs in `.anchor/program-logs`