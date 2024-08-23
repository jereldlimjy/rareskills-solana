import * as anchor from "@coral-xyz/anchor";
import { Program, AnchorError } from "@coral-xyz/anchor";
import { Day14 } from "../target/types/day_14";
import { assert } from "chai";

describe("day_14", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day14 as Program<Day14>;

  // single signer
  // it("Is initialized!", async () => {
  //   // Add your test here.
  //   const tx = await program.methods.initialize().accounts({
  //     signer1: program.provider.publicKey
  //   }).rpc();
  //   console.log("The signer1: ", program.provider.publicKey.toBase58());
  // });

  // multiple signers

  // generate a signer to call our function
  let myKeypair = anchor.web3.Keypair.generate();
  let myOtherKeypair = anchor.web3.Keypair.generate();

  it("Has 2 signers", async () => {
    const tx = await program.methods.initialize().accounts({
      signer1: program.provider.publicKey,
      signer2: myKeypair.publicKey,
    }).signers([
      myKeypair
    ]).rpc()

    console.log("tx signature:", tx);
    console.log("The signer1: ", program.provider.publicKey.toBase58());
    console.log("The signer2: ", myKeypair.publicKey.toBase58());
  })


  it("Has 3 signers", async () => {
    await program.methods.threeSigners().accounts({
      signer1: program.provider.publicKey,
      signer2: myKeypair.publicKey,
      signer3: myOtherKeypair.publicKey
    }).signers([
      myKeypair,
      myOtherKeypair
    ]).rpc();

    console.log("The signer1: ", program.provider.publicKey.toBase58());
    console.log("The signer2: ", myKeypair.publicKey.toBase58());
    console.log("The signer3: ", myOtherKeypair.publicKey.toBase58());
  })

  it("Allows owner", async () => {
    const tx = await program.methods.onlyOwner().accounts({
      signer_account: program.provider.publicKey
    }).rpc();
    console.log("tx signature:", tx);
  })

  it("Is Not called by owner", async () => {
    try {
      const tx = await program.methods.onlyOwner().accounts({
        signer_account: myKeypair.publicKey
      }).signers([myKeypair]).rpc();
      console.log("tx signature:", tx);
    } catch (_err) {
      // TODO: not working as expected
      assert.isTrue(_err instanceof AnchorError);
      const err: AnchorError = _err;
      const errMsg =
        "Only owner can call this function!";
      assert.strictEqual(err.error.errorMessage, errMsg);
      console.log("Error number:", err.error.errorCode.number);
    }
  })
});
