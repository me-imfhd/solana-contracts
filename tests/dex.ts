import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Dex } from "../target/types/dex";
import { Token2022Program } from "../target/types/token_2022_program";
import { TOKEN_2022_PROGRAM_ID } from "@solana/spl-token";

describe("init", () => {
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Dex as Program<Dex>;
  const token_2022_program = anchor.workspace
    .Token2022Program as Program<Token2022Program>;
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
  it("Fund Pool", async () => {
    const { mint, ata: funderTokenAccount } = await create_mint_tokens(
      "USDC",
      token_2022_program,
      wallet
    );
    const tx = await program.methods
      .fundPool(new anchor.BN(100))
      .accounts({
        mint: mint,
        signer: wallet.publicKey,
        funderTokenAccount,
      })
      .rpcAndKeys({
        skipPreflight: true,
      });
    console.log(tx);
  });
});

async function create_mint_tokens(
  name: string,
  token_2022_program: anchor.Program<Token2022Program>,
  wallet: anchor.Wallet
) {
  const keys = await token_2022_program.methods
    .createToken(name)
    .accounts({
      tokenProgram: TOKEN_2022_PROGRAM_ID,
      signer: wallet.publicKey,
    })
    .rpcAndKeys({ skipPreflight: true });
  console.log(keys);
  const ata_keys = await token_2022_program.methods
    .createAssociatedTokenAccount()
    .accounts({
      mint: keys.pubkeys.mint,
      signer: wallet.publicKey,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
    })
    .rpcAndKeys({ skipPreflight: true });
  await token_2022_program.methods
    .mintToken(new anchor.BN(1000))
    .accounts({
      mint: keys.pubkeys.mint,
      receiver: ata_keys.pubkeys.tokenAccount,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
      signer: wallet.publicKey,
    })
    .rpcAndKeys({
      skipPreflight: true,
    });
  return {
    mint: keys.pubkeys.mint,
    ata: ata_keys.pubkeys.tokenAccount,
  };
}
