import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day13 } from "../target/types/day_13";

describe("day_13", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day13 as Program<Day13>;

  it("Is initialized!", async () => {
    // Add your test here.
    const myEventListener = program.addEventListener("myEvent", (event, slot) => {
      console.log(`my event ${event} occurred at slot ${slot}`);
    });
  
    const stringEventListener = program.addEventListener("stringEvent", (event, slot) => {
      console.log(`my event ${event} occurred at slot ${slot}`);
    });

    await program.methods.initialize(new anchor.BN(777), "Hello World").rpc();
    // console.log("Your transaction signature", tx);

    await new Promise((resolve) => setTimeout(resolve, 10000));

    program.removeEventListener(myEventListener);
    program.removeEventListener(stringEventListener);
  });
});
