import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Dex } from "../target/types/dex";

describe("init", () => {
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Dex as Program<Dex>;
  const wallet = provider.wallet as anchor.Wallet;

  it("Created Liquidity pool!", async () => {
    // Add your test here.
    const tx = await program.methods
      .createPool()
      .accounts({
        payer: wallet.publicKey,
      })
      .rpcAndKeys({
        skipPreflight: true,
      });
    console.log("Your transaction signature", tx);
  });
});
