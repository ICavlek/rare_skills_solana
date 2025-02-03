import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day14 } from "../target/types/day_14";

describe("day_14", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day14 as Program<Day14>;

  let myKeypair = anchor.web3.Keypair.generate();

  it("Is signed by multiple signers", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().accounts({
      signer1: program.provider.publicKey,
      signer2: myKeypair.publicKey
    })
    .signers([myKeypair])
    .rpc();
    console.log("Signer1: ", program.provider.publicKey.toBase58());
    console.log("Signer2: ", myKeypair.publicKey.toBase58());
  });

  it("Is called by the owner", async () => {
    const tx = await program.methods
      .calledByOwner()
      .accounts({
        signerAccount: program.provider.publicKey,
      })
      .rpc();

    console.log("Transaction hash:", tx);
  })
  
  it("Is NOT called by the owner", async () => {
    const tx = await program.methods
      .calledByOwner()
      .accounts({
        signerAccount: myKeypair.publicKey,
      })
      .signers([myKeypair])
      .rpc();

    console.log("Transaction hash:", tx);
  })
});
