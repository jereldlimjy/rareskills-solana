import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ChangeOwner } from "../target/types/change_owner";

describe("change_owner", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.ChangeOwner as Program<ChangeOwner>;

  const airdrop = async (address: anchor.web3.PublicKey) => {
    const tx = await anchor.getProvider().connection.requestAirdrop(address, 1 * anchor.web3.LAMPORTS_PER_SOL);
    const latestBlockHash = await anchor.getProvider().connection.getLatestBlockhash();
    await anchor.getProvider().connection.confirmTransaction({
      blockhash: latestBlockHash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: tx
    });
  }

  const getAccountInfo = async (address: anchor.web3.PublicKey) => {
    return await anchor.getProvider().connection.getAccountInfo(address);
  }

  it("Is initialized!", async () => {
    // pda account
    const [pda, _bump] = await anchor.web3.PublicKey.findProgramAddressSync([], program.programId);

    console.log("the owner of the pda account is:", await getAccountInfo(pda));

    // initialise pda account
    await program.methods.initializePda().accounts({
      pda
    }).rpc();

    console.log("the owner of the pda account is:", (await getAccountInfo(pda)).owner);

    // keypair account
    const newKeypair = anchor.web3.Keypair.generate();

    console.log("keypair address:", newKeypair.publicKey.toBase58());
    console.log("the owner of the keypair account is:", await getAccountInfo(newKeypair.publicKey));

    // initialise keypair account
    await program.methods.initializeKeypair().accounts({
      keypair: newKeypair.publicKey
    }).signers([newKeypair]).rpc();

    console.log("the owner of the keypair account is:", (await getAccountInfo(newKeypair.publicKey)).owner);

    // change owner of keypair account
    await program.methods.changeOwner().accounts({
      keypair: newKeypair.publicKey
    }).rpc();

    console.log("the owner of the keypair account is:", (await getAccountInfo(newKeypair.publicKey)).owner);
  });
});
