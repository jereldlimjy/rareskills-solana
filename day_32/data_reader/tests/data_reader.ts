import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DataReader } from "../target/types/data_reader";

describe("data_reader", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.DataReader as Program<DataReader>;

  it("Is initialized!", async () => {
    const otherAccountAddress = "CtiWxFVRaEbuZ9PVV7GQqgdKUwT18LWcr3VcsZ8VASaP";

    const tx = await program.methods.readOtherAccount().accounts({
      otherAccount: new anchor.web3.PublicKey(otherAccountAddress)
    }).rpc();
    console.log("Your transaction signature", tx);
  });
});
