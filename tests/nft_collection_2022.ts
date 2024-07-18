import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { NftCollection2022 } from "../target/types/nft_collection_2022";

describe("token", () => {
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .NftCollection2022 as Program<NftCollection2022>;
  const wallet = provider.wallet as anchor.Wallet;
  const groupMint = new anchor.web3.Keypair();
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
    const mint = new anchor.web3.Keypair();
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
});
