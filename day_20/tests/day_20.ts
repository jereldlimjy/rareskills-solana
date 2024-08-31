import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day20 } from "../target/types/day_20";

describe("day_20", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day20 as Program<Day20>;
  const [storageAccountAddress, _bump] = anchor.web3.PublicKey.findProgramAddressSync([], program.programId);

  // it("Is initialized!", async () => {
  //   // Add your test here.
  //   console.log("program address:", storageAccountAddress.toBase58());
  //   const tx = await program.methods.initialize().accounts({
  //     storageAccount: storageAccountAddress
  //   }).rpc();
  //   console.log("Your transaction signature", tx);
  // });

  it("Has increased size!", async () => {
    // Add your test here.
    console.log("program address:", storageAccountAddress.toBase58());
    const tx = await program.methods.increaseAccountSize().accounts({
      storageAccount: storageAccountAddress
    }).rpc();
    console.log("Your transaction signature", tx);
  });
});
