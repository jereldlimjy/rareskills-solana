import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day19 } from "../target/types/day_19";

describe("day_19", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day19 as Program<Day19>;

  it("Initialise and set value", async () => {
    // Add your test here.
    const key = new anchor.BN(3);
    const value = new anchor.BN(777);

    const seeds = [key.toArrayLike(Buffer, "le", 8)];

    const [valueAccount, _bump] = anchor.web3.PublicKey.findProgramAddressSync(seeds, program.programId);

    // const tx = await program.methods.initialize(key).accounts({
    //   myMapping: valueAccount
    // }).rpc();

    // console.log("Your transaction signature", tx);

    const setTx = await program.methods.set(key, value).accounts({
      myStorage: valueAccount
    }).rpc();

    console.log("Your set transaction signature", setTx);

    const storageAccount = await program.account.myStorage.fetch(valueAccount);
    console.log(`storage account value: ${storageAccount.val}`);
  });
});
