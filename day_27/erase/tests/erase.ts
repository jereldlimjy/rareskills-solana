import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Erase } from "../target/types/erase";

describe("erase", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Erase as Program<Erase>;

  it("Is initialized!", async () => {
    const [pda, _bump] = anchor.web3.PublicKey.findProgramAddressSync([], program.programId);
    const tx = await program.methods.initialize().accounts({
      pda
    }).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Is erased!", async () => {
    const [pda, _bump] = anchor.web3.PublicKey.findProgramAddressSync([], program.programId);
    console.log("owner of pda is:", (await anchor.getProvider().connection.getAccountInfo(pda)).owner.toBase58());

    const pdaData = await program.account.myPdaData.fetch(pda);
    console.log("pda account:", pdaData);

    await program.methods.erase().accounts({
      myPda: pda
    }).rpc();
    console.log("data erased");

    console.log("owner of pda is:", (await anchor.getProvider().connection.getAccountInfo(pda)).owner.toBase58());
  });
});
