import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day2 } from "../target/types/day_2";

describe("day_2", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day2 as Program<Day2>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize(new anchor.BN(777), new anchor.BN(888), "hello").rpc();
    console.log("Your transaction signature", tx);
  });

  it("Array test", async () => {
    const tx = await program.methods.array([new anchor.BN(777), new anchor.BN(888)]).rpc();
    console.log("Your transation signature", tx);
  })

  it("Add test", async () => {
    const tx = await program.methods.add(new anchor.BN(1), new anchor.BN(2)).rpc();
    console.log("Your transation signature", tx);
  })

  it("Subtract test", async () => {
    const tx = await program.methods.subtract(new anchor.BN(5), new anchor.BN(4)).rpc();
    console.log("Your transation signature", tx);
  })

  it("Multiply test", async () => {
    const tx = await program.methods.multiply(new anchor.BN(2), new anchor.BN(3)).rpc();
    console.log("Your transation signature", tx);
  })

  it("Divide test", async () => {
    const tx = await program.methods.divide(new anchor.BN(10), new anchor.BN(2)).rpc();
    console.log("Your transation signature", tx);
  })
});
