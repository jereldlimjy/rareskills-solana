import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day25 } from "../target/types/day_25";

describe("day_25", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day25 as Program<Day25>;

  const airdrop = async (address: anchor.web3.PublicKey) => {
    const airdropTx = await anchor.getProvider().connection.requestAirdrop(address, 1 * anchor.web3.LAMPORTS_PER_SOL);
    const latestBlockHash = await anchor.getProvider().connection.getLatestBlockhash();
    await anchor.getProvider().connection.confirmTransaction({
      blockhash: latestBlockHash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: airdropTx
    });
  }

  // it("Is initialized!", async () => {
  //   const newKeypair = anchor.web3.Keypair.generate();

  //   // await airdrop(newKeypair.publicKey);

  //   await program.methods.initialize().accounts({
  //     keypairAccount: newKeypair.publicKey
  //   }).signers([newKeypair]).rpc();
  // });

  // it("Fails to initialize without private key!", async () => {
  //   const newKeypair = anchor.web3.Keypair.generate();

  //   // await airdrop(newKeypair.publicKey);

  //   await program.methods.initialize().accounts({
  //     keypairAccount: newKeypair.publicKey
  //   }).rpc();
  // });


  // it("Fails with random PDA", async () => {
  //   const seeds = [];
  //   const [pda, _bump] = anchor.web3.PublicKey.findProgramAddressSync(seeds, program.programId);

  //   await program.methods.initialize().accounts({
  //     keypairAccount: pda
  //   }).rpc();
  // });

  it("Fails to spend SOL after initialising keypair account in program", async () => {
    const newKeypair = anchor.web3.Keypair.generate();
    const receiverKeypair = anchor.web3.Keypair.generate();

    // airdrop sol to account
    await airdrop(newKeypair.publicKey);
    const accountInfoBefore = await anchor.getProvider().connection.getAccountInfo(newKeypair.publicKey);
		console.log(`initial keypair account owner is ${accountInfoBefore.owner}`);

    const transferTx = new anchor.web3.Transaction().add(
      anchor.web3.SystemProgram.transfer({
        fromPubkey: newKeypair.publicKey,
        toPubkey: receiverKeypair.publicKey,
        lamports: 0.1 * anchor.web3.LAMPORTS_PER_SOL
      })
    );

    // this should pass
    await anchor.web3.sendAndConfirmTransaction(anchor.getProvider().connection, transferTx, [newKeypair]);
    console.log('sent 0.1 sol');

    // initialise keypair account in program
    await program.methods.initialize().accounts({
      keypairAccount: newKeypair.publicKey
    }).signers([newKeypair]).rpc();

    // get account owner after initialization
		const accountInfoAfter = await anchor.getProvider().connection.getAccountInfo(newKeypair.publicKey);
		console.log(`new keypair account owner is ${accountInfoAfter.owner}`);

    // this should fail
    try {
      await anchor.web3.sendAndConfirmTransaction(anchor.getProvider().connection, transferTx, [newKeypair]);
    } catch (err) {
      console.log('sending 0.1 sol failed now :(');
    }
  });
});
