import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Crowdfund } from "../target/types/crowdfund";

describe("crowdfund", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Crowdfund as Program<Crowdfund>;
  const defaultPublickey = new anchor.web3.PublicKey("Hj68YqdDaaTPw5hffSMV3hAXxGpijWswEMEcaR3bQmRB");

  // it("Is initialized!", async () => {
  //   // initialise pda account
  //   const [pda, _bump] = anchor.web3.PublicKey.findProgramAddressSync([], program.programId);

  //   await program.methods.initializePda().accounts({
  //     pda
  //   }).rpc();

  //   console.log("owner of pda account is:", (await anchor.getProvider().connection.getAccountInfo(pda)).owner);
  //   console.log(`pda account has: ${(await anchor.getProvider().connection.getAccountInfo(pda)).lamports} lamports`);

  //   // donate some sol
  //   await program.methods.donate(new anchor.BN(0.1 * anchor.web3.LAMPORTS_PER_SOL)).accounts({
  //     pda
  //   }).rpc();

  //   console.log(`pda account has: ${(await anchor.getProvider().connection.getAccountInfo(pda)).lamports} lamports`);

  //   // withdraw the sol
  //   await program.methods.withdraw(new anchor.BN(0.05 * anchor.web3.LAMPORTS_PER_SOL)).accounts({
  //     pda
  //   }).rpc();

  //   console.log(`pda account has: ${(await anchor.getProvider().connection.getAccountInfo(pda)).lamports} lamports`);
  // });

  // test withdrawing full balance
  it("Tries to withdraw full lamport balance", async () => {
    const [pda, _bump] = anchor.web3.PublicKey.findProgramAddressSync([], program.programId);

    console.log("pre initialise");

    console.log(`pda account has: ${(await anchor.getProvider().connection.getAccountInfo(pda))} lamports`);

    await program.methods.initializePda().accounts({
      pda
    }).rpc();

    console.log("post initialise");

    console.log(`pda account has: ${((await anchor.getProvider().connection.getAccountInfo(pda)).lamports)} lamports`);

    await program.methods.withdraw(new anchor.BN(((await anchor.getProvider().connection.getAccountInfo(pda)).lamports))).accounts({
      pda
    }).rpc();

    console.log("post withdraw");

    console.log(`pda account has: ${(await anchor.getProvider().connection.getAccountInfo(pda))} lamports`);
  })
});
