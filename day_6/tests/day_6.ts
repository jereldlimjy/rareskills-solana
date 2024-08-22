import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day6 } from "../target/types/day_6";

describe("day_6", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day6 as Program<Day6>;

  it("Age checker", async () => {
    let tx = await program.methods.ageChecker(new anchor.BN(35)).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Prints person", async () => {
    let tx = await program.methods.initialize("Bob", new anchor.BN(30)).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Prints even numbers", async () => {
    let tx = await program.methods.exercise([new anchor.BN(1), new anchor.BN(2), new anchor.BN(3), new anchor.BN(4), new anchor.BN(5), new anchor.BN(6)]).rpc();
    console.log("Your transaction signature", tx);
  });
});
