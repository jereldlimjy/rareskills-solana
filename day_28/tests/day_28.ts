import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day28 } from "../target/types/day_28";

describe("day_28", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day28 as Program<Day28>;

  it("Is a multicall!", async () => {
    const wallet = anchor.workspace.Day28.provider.wallet.payer;
    const [pda, _bump] = anchor.web3.PublicKey.findProgramAddressSync([], program.programId);

    let transaction = new anchor.web3.Transaction();

    transaction.add(await program.methods.initialize().accounts({ myAccount: pda }).transaction());
    transaction.add(await program.methods.set(5).accounts({ myAccount: pda }).transaction());

    await anchor.web3.sendAndConfirmTransaction(anchor.getProvider().connection, transaction, [wallet]);

    const accountData = await program.account.accountData.fetch(pda);
    console.log("account value is:", accountData.value);
  });
});
