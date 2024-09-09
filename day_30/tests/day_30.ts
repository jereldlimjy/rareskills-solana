import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day30 } from "../target/types/day_30";

describe("day_30", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day30 as Program<Day30>;

  const requestAirdrop = async (address: anchor.web3.PublicKey) => {
    const connection = anchor.getProvider().connection;
    const airdropTx = await connection.requestAirdrop(address, 1 * anchor.web3.LAMPORTS_PER_SOL);
    
    // wait for txn completion
    const latestBlockhash = await connection.getLatestBlockhash();
    await connection.confirmTransaction({
      blockhash: latestBlockhash.blockhash,
      lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
      signature: airdropTx
    });
  }

  it("Is a huge test!", async () => {
    // generate keypair
    const newKeypair = anchor.web3.Keypair.generate();

    // request airdrop
    await requestAirdrop(newKeypair.publicKey);

    // transfer SOL to another wallet
    const bob = anchor.web3.Keypair.generate();

    const transferTx = new anchor.web3.Transaction().add(
      anchor.web3.SystemProgram.transfer({
        toPubkey: bob.publicKey,
        fromPubkey: newKeypair.publicKey,
        lamports: 0.5 * anchor.web3.LAMPORTS_PER_SOL
      })
    );

    console.log(`my keypair has ${(await anchor.getProvider().connection.getAccountInfo(newKeypair.publicKey)).lamports} lamports`);
    await anchor.web3.sendAndConfirmTransaction(anchor.getProvider().connection, transferTx, [newKeypair]); // should pass

    console.log("sent 0.5 sol");
    console.log(`my keypair has ${(await anchor.getProvider().connection.getAccountInfo(newKeypair.publicKey)).lamports} lamports`);

    // initialise account from program
    await program.methods.initialize().accounts({
      myKeypairAccount: newKeypair.publicKey
    }).signers([newKeypair]).rpc();

    // try transferring sol directly from keypair (should fail)
    try {
      const transferTx = new anchor.web3.Transaction().add(
        anchor.web3.SystemProgram.transfer({
          toPubkey: bob.publicKey,
          fromPubkey: newKeypair.publicKey,
          lamports: 0.5 * anchor.web3.LAMPORTS_PER_SOL
        })
      );

      await anchor.web3.sendAndConfirmTransaction(anchor.getProvider().connection, transferTx, [newKeypair]);
    } catch (err) {
      console.log("transfer 0.5 sol fails");
    }

    // close account
    await program.methods.close().accounts({
      myKeypairAccount: newKeypair.publicKey
    }).rpc();

    // account info should be null
    console.log("my keypair account info:", await anchor.getProvider().connection.getAccountInfo(newKeypair.publicKey));

    // try calling set value (should fail)
    try {
      await program.methods.set(new anchor.BN(5)).accounts({
        myKeypairAccount: newKeypair.publicKey
      }).rpc();
    } catch (err) {
      console.log("set value fails");
    }

    // request airdrop
    await requestAirdrop(newKeypair.publicKey);

    // set value should still fail
    try {
      await program.methods.set(new anchor.BN(5)).accounts({
        myKeypairAccount: newKeypair.publicKey
      }).rpc();
    } catch (err) {
      console.log("set value fails again");
    }

    // transfer should pass now
    console.log(`my keypair has ${(await anchor.getProvider().connection.getAccountInfo(newKeypair.publicKey)).lamports} lamports`);
    await anchor.web3.sendAndConfirmTransaction(anchor.getProvider().connection, transferTx, [newKeypair]); // should pass

    console.log("sent 0.5 sol");
    console.log(`my keypair has ${(await anchor.getProvider().connection.getAccountInfo(newKeypair.publicKey)).lamports} lamports`);
  });
});
