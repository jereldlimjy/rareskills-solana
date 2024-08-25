import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day17 } from "../target/types/day_17";

describe("day_17", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day17 as Program<Day17>;

  // it("Is initialized!", async () => {
  //   const [myStorage, _bump] = anchor.web3.PublicKey.findProgramAddressSync([], program.programId);

  //   console.log("my storage address", myStorage);

  //   const tx = await program.methods.initialize().accounts({
  //     myStorage
  //   }).rpc();
  //   console.log("Your transaction signature", tx);
  // });

  // it("Sets new x", async () => {
  //   const [myStorage, _bump] = anchor.web3.PublicKey.findProgramAddressSync([], program.programId);

  //   const tx = await program.methods.set(new anchor.BN(5)).accounts({
  //     myStorage
  //   }).rpc();
  //   console.log("Your transaction signature", tx);
  // });

  it("Prints X", async () => {
    const [myStorage, _bump] = anchor.web3.PublicKey.findProgramAddressSync([], program.programId);

    await program.methods.print().accounts({
      myStorage
    }).rpc();
  });

  it("Increments X", async () => {
    const [myStorage, _bump] = anchor.web3.PublicKey.findProgramAddressSync([], program.programId);

    await program.methods.increment().accounts({
      myStorage
    }).rpc();
  });
});
