import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Counter } from "../target/types/counter";

describe("counter", () => {
  // Configure the client to use the local cluster.
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Counter as Program<Counter>;
  const wallet = provider.wallet as anchor.Wallet;
  const counter_account = new anchor.web3.Keypair();
  it("Create Counter", async () => {
    // Add your test here.
    const tx = await program.methods
      .createCounter()
      .accounts({ counter: counter_account.publicKey, user: wallet.publicKey })
      .signers([wallet.payer, counter_account])
      .rpc();

    console.log("Your transaction signature", tx);
  });
  it("Fetch a counter!", async () => {
    const counter = await program.account.counter.fetch(
      counter_account.publicKey
    );
    console.log("Your counter", counter);
  });
  it("Increment a counter!", async () => {
    const tx_1 = await program.methods
      .incrementCounter()
      .accounts({ counter: counter_account.publicKey })
      .rpc();
    const tx_2 = await program.methods
      .incrementCounter()
      .accounts({ counter: counter_account.publicKey })
      .rpc();
    const tx_3 = await program.methods
      .incrementCounter()
      .accounts({ counter: counter_account.publicKey })
      .rpc();

    const counterUpdated = await program.account.counter.fetch(
      counter_account.publicKey
    );
    console.log("Your counter count is: ", counterUpdated.count);
  });
  it("Decrement a counter!", async () => {
    const tx_1 = await program.methods
      .decrementCounter()
      .accounts({ counter: counter_account.publicKey })
      .rpc();

    const counterUpdated = await program.account.counter.fetch(
      counter_account.publicKey
    );
    console.log("Your counter count is: ", counterUpdated.count);
  });
});
