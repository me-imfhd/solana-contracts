import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Token } from "../target/types/token";
import {
  getAccount,
  getAssociatedTokenAddress,
  getAssociatedTokenAddressSync,
  getMint,
  getOrCreateAssociatedTokenAccount,
} from "@solana/spl-token";
import { assert } from "chai";
describe("token", () => {
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Token as Program<Token>;
  const wallet = provider.wallet as anchor.Wallet;
  const mint = new anchor.web3.Keypair();
  it("Created mint!", async () => {
    const tx = await program.methods
      .createMint(0)
      .accounts({ mint: mint.publicKey, signer: wallet.publicKey })
      .signers([mint])
      .rpc();
    console.log("Your transaction signature", tx);
  });
  it("Attached Metadata!", async () => {
    const tx = await program.methods
      .attachMetadata({
        name: "Devil",
        symbol: "DVL",
        uri: "https://media.istockphoto.com/id/1132736427/vector/devil-emoticon-isolated-on-white-background-emoji-smiley-vector-illustration.jpg?s=612x612&w=0&k=20&c=l4O1ujfaPcXV9zNdgPwHtC_wK-zgTrMbGmieRUf_T3A=",
      })
      .accounts({ mint: mint.publicKey, signer: wallet.publicKey })
      .rpc();
    console.log("Your transaction signature", tx);
  });
  it("Create Token Accounts!", async () => {
    const tx = await program.methods
      .createUserTokenAccount()
      .accounts({ mint: mint.publicKey, signer: wallet.publicKey })
      .rpc();
    console.log("Your transaction signature", tx);
  });
  it("Mint Tokens to Myself!", async () => {
    // Creates token account if does not already exist
    const tx = await program.methods
      .mintTokens(100)
      .accounts({ mint: mint.publicKey, signer: wallet.publicKey })
      .rpc();
    console.log("Your transaction signature", tx);
    let mint_after = await getMint(provider.connection, mint.publicKey);
    assert("100" == mint_after.supply.toString());
  });
  it("Transfer Tokens!", async () => {
    // Creates the reciver's token account if does not already exist
    let reciever = new anchor.web3.Keypair();
    const tx = await program.methods
      .transferToken(50)
      .accounts({
        mint: mint.publicKey,
        signer: wallet.publicKey,
        recieverAccount: reciever.publicKey,
      })
      .rpc();
    let reciever_token_address = getAssociatedTokenAddressSync(
      mint.publicKey,
      reciever.publicKey
    );
    let reciever_token_account = await getAccount(
      provider.connection,
      reciever_token_address
    );
    let sender_token_address = getAssociatedTokenAddressSync(
      mint.publicKey,
      wallet.publicKey
    );
    let sender_token_account = await getAccount(
      provider.connection,
      sender_token_address
    );
    assert("50" == reciever_token_account.amount.toString());
    assert("50" == sender_token_account.amount.toString());
    let mint_after = await getMint(provider.connection, mint.publicKey);
    assert("100" == mint_after.supply.toString());
    console.log("Your transaction signature", tx);
  });
});
