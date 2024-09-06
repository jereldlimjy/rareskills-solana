import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { StorageRecap } from "../target/types/storage_recap";

describe("storage_recap", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.StorageRecap as Program<StorageRecap>;
  const [storageAccountAddress, _bump] = anchor.web3.PublicKey.findProgramAddressSync([], program.programId);

  // it("Is initialized!", async () => {
  //   const tx = await program.methods.initialize().accounts({
  //     storageAccount: storageAccountAddress
  //   }).rpc();
  //   console.log("Your transaction signature", tx);
  // });

  it("Sets num and is_happy", async () => {
    const tx = await program.methods.set(new anchor.BN(777), true).accounts({
      storageAccount: storageAccountAddress
    }).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Reads num and is_happy", async () => {
    // queries the blockchain to retrieve account data at `storageAccountAddress`
    // deserialises the data into a TS object using the schema defined by the `storageAccountData` struct
    const storageAccountData = await program.account.storageAccountData.fetch(storageAccountAddress);
    console.log("the storage account data obj is:", storageAccountData);
  })
});
