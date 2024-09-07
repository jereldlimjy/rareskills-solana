import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day27 } from "../target/types/day_27";

describe("day_27", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day27 as Program<Day27>;

  const airdrop = async (address: anchor.web3.PublicKey) => {
    const tx = await anchor.getProvider().connection.requestAirdrop(address, 1 * anchor.web3.LAMPORTS_PER_SOL);
    const latestBlockhash = await anchor.getProvider().connection.getLatestBlockhash();
    await anchor.getProvider().connection.confirmTransaction({
      blockhash: latestBlockhash,
      lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
      signature: tx
    });
  }


  // it("It increments!", async () => {
  //   const [pda, _bump] = anchor.web3.PublicKey.findProgramAddressSync([], program.programId);

  //   // create pda account and set counter to 1
  //   await program.methods.increment().accounts({
  //     pda
  //   }).rpc();

  //   // fetch pda account data
  //   const pdaData = await program.account.myPdaData.fetch(pda);

  //   console.log("my counter value is:", pdaData.counter);
  // });


  it("It initialises!", async () => {
    const [pda, _bump] = anchor.web3.PublicKey.findProgramAddressSync([], program.programId);

    await program.methods.initialise().accounts({
      myPda: pda
    }).rpc();
    console.log("initialised");

    await airdrop(pda);

    // owner of pda should now be program
    const pdaAccountInfo = await anchor.getProvider().connection.getAccountInfo(pda);
    console.log("owner of pda account is:", pdaAccountInfo.owner.toBase58());

    // // drain lamports
    await program.methods.drainLamport().accounts({
      myPda: pda
    }).rpc();

    // owner should be null
    console.log("owner of pda account is:", await anchor.getProvider().connection.getAccountInfo(pda));

    // initialise again
    await program.methods.initialise().accounts({
      myPda: pda
    }).rpc();
    console.log("initialised");

    // assign to system program
    await program.methods.giveToSystemProgram().accounts({
      myPda: pda
    }).rpc();
    console.log("owner of pda account is:", (await anchor.getProvider().connection.getAccountInfo(pda)).owner.toBase58());
  });
});
