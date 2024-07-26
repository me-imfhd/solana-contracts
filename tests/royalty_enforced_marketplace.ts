import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { RoyaltyEnforcedMarketplace } from "../target/types/royalty_enforced_marketplace";
import { EnforcedTransferHook } from "../target/types/enforced_transfer_hook";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";

describe("token", () => {
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .RoyaltyEnforcedMarketplace as Program<RoyaltyEnforcedMarketplace>;
  const transferHookProgram = anchor.workspace
    .EnforcedTransferHook as Program<EnforcedTransferHook>;
  const wallet = provider.wallet as anchor.Wallet;
  const groupMint = new anchor.web3.Keypair();
  const mint = new anchor.web3.Keypair();
  const creator_1 = new anchor.web3.Keypair();
  const creator_2 = new anchor.web3.Keypair();
  const buyer = new anchor.web3.Keypair();
  const create_buyer_account = anchor.web3.SystemProgram.createAccount({
    fromPubkey: wallet.publicKey,
    lamports: anchor.web3.LAMPORTS_PER_SOL * 10,
    newAccountPubkey: buyer.publicKey,
    programId: SYSTEM_PROGRAM_ID,
    space: 0,
  });
  it("Mint Collection", async () => {
    const tx = await program.methods
      .collectionMint({
        name: "DEVIL Collection",
        symbol: "DVL",
        uri: "collection_uri",
        maxSize: 10, // max 10 nfts can be added
      })
      .accounts({
        payer: wallet.publicKey,
        groupMint: groupMint.publicKey,
        mintTo: wallet.publicKey,
      })
      .signers([groupMint])
      .rpcAndKeys({ skipPreflight: true });
    console.log(tx);
  });
  it("Mint Nft", async () => {
    const tx = await program.methods
      .mintNft({
        name: "DEVIL #1",
        symbol: "DVL",
        uri: "nft_uri",
      })
      .accounts({
        payer: wallet.publicKey,
        mint: mint.publicKey,
        mintTo: wallet.publicKey,
        groupMint: groupMint.publicKey,
      })
      .signers([mint])
      .rpcAndKeys({ skipPreflight: true });
    console.log(tx);
  });
  it("Adds royalties and initailized extra meta accounts", async () => {
    const tx = await program.methods
      .addRoyalties({
        creators: [
          { address: creator_1.publicKey, share: 40 },
          { address: creator_2.publicKey, share: 60 },
        ],
        royaltyBasisPoints: 1000,
      })
      .accounts({
        mint: mint.publicKey,
        payer: wallet.publicKey,
      })
      .rpcAndKeys({
        skipPreflight: true,
      });
    console.log(tx);
    const tx_2 = await transferHookProgram.methods
      .initializeExtraAccountMetaList()
      .accounts({ mint: mint.publicKey, payer: wallet.publicKey })
      .rpcAndKeys({
        skipPreflight: true,
      });
    console.log(tx_2);
  });
  it("List Nft", async () => {
    const tx = await program.methods
      .listNft({ price: new anchor.BN(1000) })
      .accounts({
        mint: mint.publicKey,
        seller: wallet.publicKey,
      })
      .rpcAndKeys({
        skipPreflight: true,
      });
    console.log(tx);
  });
  it("Buy Nft", async () => {
    let create_buyer_account_tx = new anchor.web3.Transaction().add(
      create_buyer_account
    );
    let t = await anchor.web3.sendAndConfirmTransaction(
      provider.connection,
      create_buyer_account_tx,
      [wallet.payer, buyer]
    );
    console.log(t);
    const extraMetasAccount = anchor.web3.PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode("extra-account-metas"),
        mint.publicKey.toBuffer(),
      ],
      transferHookProgram.programId
    )[0];
    console.log(extraMetasAccount);
    const tx = await program.methods
      .buyNft()
      .accounts({
        mint: mint.publicKey,
        seller: wallet.publicKey,
        buyer: buyer.publicKey,
        transferHookProgram: transferHookProgram.programId,
        extraMetasAccount,
      })
      .signers([buyer])
      .rpcAndKeys({
        skipPreflight: true,
      });
    console.log(tx);
  });
});
