import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day26 } from "../target/types/day_26";

describe("day_26", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day26 as Program<Day26>;

  const airdrop = async (address: anchor.web3.PublicKey) => {
    const tx = await anchor.getProvider().connection.requestAirdrop(address, 1 * anchor.web3.LAMPORTS_PER_SOL);

    const latestBlockhash = await anchor.getProvider().connection.getLatestBlockhash();

    await anchor.getProvider().connection.confirmTransaction({
      blockhash: latestBlockhash,
      lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
      signature: tx
    });
  }

  it("Is initialized!", async () => {
    // initialise PDA account
    const [pda, _bump] = anchor.web3.PublicKey.findProgramAddressSync([], program.programId);

    // get owner of pda
    console.log("owner of pda account is:", (await anchor.getProvider().connection.getAccountInfo(pda)));

    const pdaTx = await program.methods.initializePda().accounts({
      myAccount: pda
    }).rpc();

    console.log("Your transaction signature", pdaTx);
    console.log("owner of pda account is:", (await anchor.getProvider().connection.getAccountInfo(pda)).owner);

    // Initialise Keypair account
    const newKeypair = anchor.web3.Keypair.generate();

    console.log("owner of keypair account is:", (await anchor.getProvider().connection.getAccountInfo(newKeypair.publicKey)));

    // airdrop sol
    await airdrop(newKeypair.publicKey);
    console.log("airdropped sol");

    console.log("owner of keypair account is:", (await anchor.getProvider().connection.getAccountInfo(newKeypair.publicKey)).owner);

    const keypairTx = await program.methods.initializeKeypair().accounts({
      myAccount: newKeypair.publicKey
    }).signers([newKeypair]).rpc();

    console.log("Your transaction signature", keypairTx);
    console.log("owner of keypair account is:", (await anchor.getProvider().connection.getAccountInfo(newKeypair.publicKey)).owner);
  });
});
