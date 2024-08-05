import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Dex } from "../target/types/dex";
import { Token2022Program } from "../target/types/token_2022_program";
import { TOKEN_2022_PROGRAM_ID } from "@solana/spl-token";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";

describe("init", async () => {
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Dex as Program<Dex>;
  const token_2022_program = anchor.workspace
    .Token2022Program as Program<Token2022Program>;
  const wallet = provider.wallet as anchor.Wallet;
  let usdc_mint: anchor.web3.PublicKey;
  let weth_mint: anchor.web3.PublicKey;
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
  it("Create mint and fund main wallet tokens", async () => {
    const { mint } = await create_mint_tokens(
      "USDC",
      2,
      token_2022_program,
      wallet
    );
    const { mint: wEthMint } = await create_mint_tokens(
      "wETH",
      5,
      token_2022_program,
      wallet
    );
    usdc_mint = mint;
    weth_mint = wEthMint;
  });
  it("Fund USDC", async () => {
    const tx = await program.methods
      .fundPool(new anchor.BN(100))
      .accounts({
        mint: usdc_mint,
        signer: wallet.publicKey,
      })
      .rpcAndKeys({
        skipPreflight: true,
      });
    console.log(tx);
  });
  it("Fund wETH", async () => {
    const tx = await program.methods
      .fundPool(new anchor.BN(100))
      .accounts({
        mint: weth_mint,
        signer: wallet.publicKey,
      })
      .rpcAndKeys({
        skipPreflight: true,
      });
    console.log(tx);
    // total funds in pool is 100 * 100 = 10_000
  });
  it("Exchange", async () => {
    let buyer_wallet = new anchor.web3.Keypair();
    await create_buyer_account_and_mint_tokens(
      provider,
      usdc_mint,
      token_2022_program,
      wallet,
      buyer_wallet
    );
    // With explorer check that total funded amount to pool is constant or not after swap
    // Also try changing decimal of mints and check if the rule is not violated
    const tx = await program.methods
      .swap(new anchor.BN(100))
      .accounts({
        mintBase: weth_mint,
        mintQuote: usdc_mint,
        signer: buyer_wallet.publicKey,
      })
      .signers([buyer_wallet])
      .rpcAndKeys({ skipPreflight: true });
    // total funds in pool is 50 * 200 = 10_000 , 50 base was give to user, at the price of 100 quote
    // since the product is constant it works
    console.log(tx);
  });
});

async function create_mint_tokens(
  name: string,
  decimal: number,
  token_2022_program: anchor.Program<Token2022Program>,
  wallet: anchor.Wallet
) {
  const keys = await token_2022_program.methods
    .createToken(name, decimal)
    .accounts({
      tokenProgram: TOKEN_2022_PROGRAM_ID,
      signer: wallet.publicKey,
    })
    .rpcAndKeys({ skipPreflight: true });
  console.log(keys);
  await token_2022_program.methods
    .createAssociatedTokenAccount()
    .accounts({
      mint: keys.pubkeys.mint,
      signer: wallet.publicKey,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
      authority: wallet.publicKey,
    })
    .rpcAndKeys({ skipPreflight: true });
  await token_2022_program.methods
    .mintToken(new anchor.BN(1000))
    .accounts({
      mint: keys.pubkeys.mint,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
      signer: wallet.publicKey,
      reciever: wallet.publicKey,
    })
    .rpcAndKeys({
      skipPreflight: true,
    });
  return {
    mint: keys.pubkeys.mint,
  };
}

async function create_buyer_account_and_mint_tokens(
  provider: anchor.AnchorProvider,
  mint: anchor.web3.PublicKey,
  token_2022_program: anchor.Program<Token2022Program>,
  wallet: anchor.Wallet,
  buyer: anchor.web3.Keypair
) {
  const create_buyer_account = anchor.web3.SystemProgram.createAccount({
    fromPubkey: wallet.publicKey,
    lamports: anchor.web3.LAMPORTS_PER_SOL * 10,
    newAccountPubkey: buyer.publicKey,
    programId: SYSTEM_PROGRAM_ID,
    space: 0,
  });
  let create_buyer_account_tx = new anchor.web3.Transaction().add(
    create_buyer_account
  );
  // create buyer account
  await anchor.web3.sendAndConfirmTransaction(
    provider.connection,
    create_buyer_account_tx,
    [wallet.payer, buyer]
  );
  // create ata for mint e.g., (USDC)
  await token_2022_program.methods
    .createAssociatedTokenAccount()
    .accounts({
      mint: mint,
      signer: wallet.publicKey,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
      authority: buyer.publicKey,
    })
    .rpcAndKeys({ skipPreflight: true });
  // Mint 1000 USDC to buyer
  await token_2022_program.methods
    .mintToken(new anchor.BN(100))
    .accounts({
      mint: mint,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
      signer: wallet.publicKey,
      reciever: buyer.publicKey,
    })
    .rpcAndKeys({
      skipPreflight: true,
    });
}
