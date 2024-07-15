import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Token } from "../target/types/token";

describe("token", () => {
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Token as Program<Token>;
  const wallet = provider.wallet as anchor.Wallet;
  const mint = new anchor.web3.Keypair();
  it("Created mint!", async () => {
    // Add your test here.
    const tx = await program.methods
      .createMint(0)
      .accounts({ mint: mint.publicKey, signer: wallet.publicKey })
      .signers([mint])
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
