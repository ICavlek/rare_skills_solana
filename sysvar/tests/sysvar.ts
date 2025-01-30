import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Sysvar } from "../target/types/sysvar";

describe("sysvar", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Sysvar as Program<Sysvar>;
  
  const StakeHistory_PublicKey = new anchor.web3.PublicKey(
    "SysvarStakeHistory1111111111111111111111111"
  );

  let account = {
        stakeHistory: StakeHistory_PublicKey,
        recentBlockhashes: anchor.web3.SYSVAR_RECENT_BLOCKHASHES_PUBKEY,
        instructionSysvar: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
  };
  
  it("Get day of the week", async () => {
    const tx = await program.methods.getDayOfTheWeek()
      .accounts(account)
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Is initialized!", async () => {
    const tx = await program.methods
      .initialize(3)
      .accounts(account)
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
