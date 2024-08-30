import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { OtherProg } from "../target/types/other_prog";

describe("other_prog", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.OtherProg as Program<OtherProg>;

  // it("Is initialized!", async () => {
  //   // Add your test here.
  //   const tx = await program.methods.initialize().rpc();
  //   console.log("Your transaction signature", tx);
  // });

  it("Sets flag", async () => {
    const [myStorage, _bump] = anchor.web3.PublicKey.findProgramAddressSync([], program.programId);

    const tx = await program.methods.set(true).accounts({
      myStorage
    }).rpc();
    console.log("my tx signature is:", tx);
  });
});
