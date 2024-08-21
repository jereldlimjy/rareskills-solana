import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorFunctionTutorial } from "../target/types/anchor_function_tutorial";

describe("anchor-function-tutorial", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AnchorFunctionTutorial as Program<AnchorFunctionTutorial>;

  it("Calls boaty mcboatface", async () => {
    // Add your test here.
    const tx = await program.methods.boatyMcBoatface(new anchor.BN(1)).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Should add", async () => {
    const tx = await program.methods.add(new anchor.BN(1), new anchor.BN(2)).rpc();
    console.log("Your transaction signature", tx);
  });
  
  it("Should sub", async () => {
    const tx = await program.methods.sub(
    new anchor.BN(10),
    new anchor.BN(3)).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Should multiply", async () => {
    const tx = await program.methods.mul(
    new anchor.BN(10),
    new anchor.BN(2)).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Should divide", async () => {
    const tx = await program.methods.div(
    new anchor.BN(10),
    new anchor.BN(5)).rpc();
    console.log("Your transaction signature", tx);
  });
});
