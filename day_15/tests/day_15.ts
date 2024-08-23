import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day15 } from "../target/types/day_15";

describe("day_15", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day15 as Program<Day15>;

  let keyPair = new anchor.web3.PublicKey("Hj68YqdDaaTPw5hffSMV3hAXxGpijWswEMEcaR3bQmRB");

  it("Is initialized!", async () => {
    // Add your test here.
    let balanceBefore = await program.provider.connection.getBalance(keyPair);
    console.log("balance before:", balanceBefore);

    const tx = await program.methods.initialize().rpc();

    let balanceAfter = await program.provider.connection.getBalance(keyPair);
    console.log("balance after:", balanceBefore);

    console.log("diff:", BigInt(balanceAfter.toString()) - BigInt(balanceBefore.toString()));
  });
});
