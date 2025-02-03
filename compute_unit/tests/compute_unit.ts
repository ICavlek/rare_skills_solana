import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ComputeUnit } from "../target/types/compute_unit";

describe("compute_unit", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.ComputeUnit as Program<ComputeUnit>;
  const defaultKeyPair = new anchor.web3.PublicKey("GGWSVKHQ525GDKwjqbWe3dv6bemYcv6ps5WXCi1TkiHU");
  let myKeypair = anchor.web3.Keypair.generate();
  
  it("Is initialized!", async () => {
    // Add your test here.
    let bal_before = await program.provider.connection.getBalance(
      defaultKeyPair
    );
    console.log("before:", bal_before);

    // call the initialize function of our program
    const tx = await program.methods.initialize().accounts({
      signer1: program.provider.publicKey,
      singer2: myKeypair.publicKey,
    }).signers([myKeypair]).rpc();

    // log the keypair's balance after
    let bal_after = await program.provider.connection.getBalance(
      defaultKeyPair
    );
    console.log("after:", bal_after);

    // log the difference
    console.log(
      "diff:",
      BigInt(bal_before.toString()) - BigInt(bal_after.toString())
    );
    // Summary: cca 5000 lamports per signer, max 12 signers in 200 000
    // program compute units
  });
});
