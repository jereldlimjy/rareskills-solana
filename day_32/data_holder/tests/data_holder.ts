import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DataHolder } from "../target/types/data_holder";

describe("data_holder", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.DataHolder as Program<DataHolder>;

  it("Is initialized!", async () => {
    const [pda, _bump] = anchor.web3.PublicKey.findProgramAddressSync([], program.programId);
    const tx = await program.methods.initialize(new anchor.BN(9)).accounts({
      otherAccount: pda
    }).rpc();
    console.log("Your transaction signature", tx);
    console.log("storage account address:", pda.toBase58());
  });
});
