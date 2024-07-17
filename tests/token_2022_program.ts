import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Token2022Program } from "../target/types/token_2022_program";

describe("token", () => {
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .Token2022Program as Program<Token2022Program>;
  const wallet = provider.wallet as anchor.Wallet;
  it("Created mint with metadata pointer!", async () => {
    const tx = await program.methods
      .createTokenWithMetadataPointer(
        {
          name: "DEVIL",
          symbol: "DVL",
          uri: "https://media.istockphoto.com/id/1132736427/vector/devil-emoticon-isolated-on-white-background-emoji-smiley-vector-illustration.jpg?s=612x612&w=0&k=20&c=l4O1ujfaPcXV9zNdgPwHtC_wK-zgTrMbGmieRUf_T3A=",
        },
        9
      )
      .accounts({
        signer: wallet.publicKey,
      })
      .rpcAndKeys({ skipPreflight: true });
    console.log(tx);
  });
});
