import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day23 } from "../target/types/day_23";

describe("day_23", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day23 as Program<Day23>;

  const printAccountBalance = async (account: anchor.web3.PublicKey) => {
    const balance = await anchor.getProvider().connection.getBalance(account);
    console.log(`${account} has balance of ${balance / anchor.web3.LAMPORTS_PER_SOL} SOL`);
  }

  it("Sends sol!", async () => {
    // generate new wallets
    let receiver = anchor.web3.Keypair.generate();
  
    await printAccountBalance(new anchor.web3.PublicKey("Hj68YqdDaaTPw5hffSMV3hAXxGpijWswEMEcaR3bQmRB"));

    const tx = await program.methods.sendSol(new anchor.BN(1 * anchor.web3.LAMPORTS_PER_SOL)).accounts({
      receiver: receiver.publicKey
    }).rpc();
    console.log("Your transaction signature", tx);

    await printAccountBalance(new anchor.web3.PublicKey("Hj68YqdDaaTPw5hffSMV3hAXxGpijWswEMEcaR3bQmRB"));
  });

  it("Splits sol!", async () => {
    // generate new wallets
    let receiver_1 = anchor.web3.Keypair.generate();
    let receiver_2 = anchor.web3.Keypair.generate();
  
    await printAccountBalance(receiver_1.publicKey);
    await printAccountBalance(receiver_2.publicKey);
    const tx = await program.methods.splitSol(new anchor.BN(1 * anchor.web3.LAMPORTS_PER_SOL)).accounts({
      receiver1: receiver_1.publicKey,
      receiver2: receiver_2.publicKey
    }).rpc();
    console.log("Your transaction signature", tx);

    await printAccountBalance(receiver_1.publicKey);
    await printAccountBalance(receiver_2.publicKey);
  });

  it("Split SOL arb", async () => {
    const recipient1 = anchor.web3.Keypair.generate();
    const recipient2 = anchor.web3.Keypair.generate();
    const recipient3 = anchor.web3.Keypair.generate();
    const recipient4 = anchor.web3.Keypair.generate();

    await printAccountBalance(recipient1.publicKey);
    await printAccountBalance(recipient2.publicKey);
    await printAccountBalance(recipient3.publicKey);
    await printAccountBalance(recipient4.publicKey);

    const accountMeta1 = {pubkey: recipient1.publicKey, isWritable: true, isSigner: false};
    const accountMeta2 = {pubkey: recipient2.publicKey, isWritable: true, isSigner: false};
    const accountMeta3 = {pubkey: recipient3.publicKey, isWritable: true, isSigner: false};
    const accountMeta4 = {pubkey: recipient4.publicKey, isWritable: true, isSigner: false};

    let amount = new anchor.BN(1 * anchor.web3.LAMPORTS_PER_SOL);
    await program.methods.splitSolArb(amount)
      .remainingAccounts([accountMeta1, accountMeta2, accountMeta3, accountMeta4])
      .rpc();

    await printAccountBalance(recipient1.publicKey);
    await printAccountBalance(recipient2.publicKey);
    await printAccountBalance(recipient3.publicKey);
    await printAccountBalance(recipient4.publicKey);
  });
});
