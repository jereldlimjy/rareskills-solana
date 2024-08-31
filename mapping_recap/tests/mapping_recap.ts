import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MappingRecap } from "../target/types/mapping_recap";

describe("mapping_recap", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.MappingRecap as Program<MappingRecap>;

  // only dealing with 1 account
  const key = new anchor.BN(98);
  const seeds = [key.toArrayLike(Buffer, "le", 8)];
  const [mapAccountAddress, _bump] = anchor.web3.PublicKey.findProgramAddressSync(seeds, program.programId);

  it("Is initialized!", async () => {
    const tx = await program.methods.initialize(key).accounts({
      mapAccount: mapAccountAddress
    }).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Sets a word in the map account", async () => {
    const tx = await program.methods.set(key, "hello world").accounts({
      mapAccount: mapAccountAddress
    }).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Reads the map account", async () => {
    const mapAccountData = await program.account.mapAccountData.fetch(mapAccountAddress);
    console.log("map account data:", mapAccountData);
  });
});
