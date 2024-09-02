import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day21 } from "../target/types/day_21";

describe("day_21", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day21 as Program<Day21>;

  it("Is initialized!", async () => {
    const publicKey = new anchor.web3.PublicKey("Hj68YqdDaaTPw5hffSMV3hAXxGpijWswEMEcaR3bQmRB");
    const tx = await program.methods.initialize().accounts({
      accountToCheck: publicKey
    }).rpc();
    console.log("Your transaction signature", tx);
  });
});
