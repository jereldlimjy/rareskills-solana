import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day29 } from "../target/types/day_29";

describe("day_29", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day29 as Program<Day29>;

  it("Is initialized!", async () => {
    const [pda, _bump] = anchor.web3.PublicKey.findProgramAddressSync([], program.programId);

    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);

    console.log(`program: ${program.programId.toBase58()}`);
    console.log(`storage account: ${pda.toBase58()}`);
  });
});
