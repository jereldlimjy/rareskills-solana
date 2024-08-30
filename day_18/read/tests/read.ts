import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Read } from "../target/types/read";
import fs from 'fs';

describe("read", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Read as Program<Read>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });

  it("Reads the flag from other prog", async () => {
    const otherProgramId = new anchor.web3.PublicKey("4h8gf2dJLxgGdhK82HmnAZPS1ixyYpfLmJetPmWfWMEK");
    const idlJson = JSON.parse(fs.readFileSync("../other_prog/target/idl/other_prog.json", "utf-8"));

    const otherProgram = new anchor.Program(idlJson);

    const [trueFalseAcc, _bump] = anchor.web3.PublicKey.findProgramAddressSync([], otherProgramId);

    const otherProgramStorageStruct =  await otherProgram.account.myStorage.fetch(trueFalseAcc);
    console.log("the value of other prog flag is:", otherProgramStorageStruct.flag.toString());
  });
});
