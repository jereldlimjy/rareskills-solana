import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day24 } from "../target/types/day_24";

describe("day_24", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day24 as Program<Day24>;

  const airdrop = async (address: anchor.web3.PublicKey) => {
    const tx = await anchor.getProvider().connection.requestAirdrop(address, 1 * anchor.web3.LAMPORTS_PER_SOL);
    await confirmTransaction(tx);
  }

  const confirmTransaction = async (tx: anchor.web3.TransactionSignature) => {
    const latestBlockHash = await anchor.getProvider().connection.getLatestBlockhash();
    await anchor.getProvider().connection.confirmTransaction({
      blockhash: latestBlockHash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: tx
    });
  }

  // for alice and bob exercise
  // it("Is initialized!", async () => {
  //   // get address of storage account
  //   const [myStorageAddress, _bumps] = anchor.web3.PublicKey.findProgramAddressSync([], program.programId);

  //   // generate new keyPair
  //   const keyPair = anchor.web3.Keypair.generate();

  //   await airdrop(keyPair.publicKey);

  //   const tx = await program.methods.initialize().accounts({
  //     myStorage: myStorageAddress,
  //     signer: keyPair.publicKey
  //   }).signers([keyPair]).rpc();

  //   console.log("Your transaction signature", tx);
  // });

  // it("Updates storage!", async () => {
  //   // get address of storage account
  //   const [myStorageAddress, _bumps] = anchor.web3.PublicKey.findProgramAddressSync([], program.programId);

  //   // generate new keyPair
  //   const keyPair = anchor.web3.Keypair.generate();

  //   await airdrop(keyPair.publicKey);

  //   const tx = await program.methods.updateValue(new anchor.BN(7)).accounts({
  //     myStorage: myStorageAddress,
  //     signer: keyPair.publicKey
  //   }).signers([keyPair]).rpc();

  //   console.log("Your transaction signature", tx);

  //   const myStorageData = await program.account.myStorageData.fetch(myStorageAddress);

  //   console.log("the value of x is now:", myStorageData.x);
  // });

  // for proto-ERC20 exercise
    it("Is initialized!", async () => {
    // generate new keyPairs
    const tom = anchor.web3.Keypair.generate();
    const dick = anchor.web3.Keypair.generate();

    const tomSeeds = [tom.publicKey.toBytes()];
    const dickSeeds = [dick.publicKey.toBytes()];

    // get address of PDAs
    const [tokenAccount, _] = anchor.web3.PublicKey.findProgramAddressSync(tomSeeds, program.programId);
    const [receiverTokenAccount, _bumps] =  anchor.web3.PublicKey.findProgramAddressSync(dickSeeds, program.programId);

    // fund tom and dick
    await airdrop(tom.publicKey);
    await airdrop(dick.publicKey);

    // initialize tom and dick accounts
    await program.methods.initialize().accounts({
      tokenAccount,
      authority: tom.publicKey
    }).signers([tom]).rpc();
    await program.methods.initialize().accounts({
      tokenAccount: receiverTokenAccount,
      authority: dick.publicKey
    }).signers([dick]).rpc();

    // transfer 5 points from tom to dick
    await program.methods.transferPoints(new anchor.BN(5)).accounts({
      tokenAccount,
      receiverTokenAccount,
      authority: tom.publicKey
    }).signers([tom]).rpc();

    // fetch balances
    const tomAccount = await program.account.tokenAccountData.fetch(tokenAccount);
    const dickAccount = await program.account.tokenAccountData.fetch(receiverTokenAccount);

    console.log(`tom has ${tomAccount.amount.toString()} tokens`);
    console.log(`dick has ${dickAccount.amount.toString()} tokens`);
  });
});
