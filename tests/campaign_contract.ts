import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CampaignContract } from "../target/types/campaign_contract";
import { LAMPORTS_PER_SOL, PublicKey, SystemProgram } from "@solana/web3.js";
import { assert } from "chai";

describe("campaign_contract", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.AnchorProvider.env();

  const program = anchor.workspace.campaignContract as Program<CampaignContract>;
  const admin = provider.wallet.publicKey;
  const user = provider.wallet.publicKey;


  // it("Create Campaign", async () => {

  //   const name = "Test Campaign";
  //   const targetAmount = new anchor.BN(1000000000);
  //   const lastDate = new anchor.BN(2735689600);
  //   const tx = await program.methods
  //     .createCampaign(name, targetAmount, lastDate)
  //     .accounts({
  //       admin: admin
  //     })
  //     .rpc();
  //   console.log("Your transaction signature", tx);

  //   const [campaignPda] = PublicKey.findProgramAddressSync(
  //     [Buffer.from("CAMPAIGN"), admin.toBuffer()],
  //     program.programId
  //   );

  //   const campaign = await program.account.crowdCampaign.fetch(campaignPda);

  //   assert.equal(campaign.campaignName, name);
  //   console.log("Campaign verified at:", campaignPda.toBase58());
  // });

  // it("Deposit SOL", async () => {

  //   const [campaignPda] = PublicKey.findProgramAddressSync(
  //     [Buffer.from("CAMPAIGN"), admin.toBuffer()],
  //     program.programId
  //   );

  //   const depositAmount = new anchor.BN(2000000000);
  //   const depositTx = await program.methods
  //     .deposited(depositAmount)
  //     .accounts({
  //       campaign: campaignPda,
  //       user: user
  //     })
  //     .rpc();
  //   console.log("Your transaction signature", depositTx);
  //   const campaignAfter = await program.account.crowdCampaign.fetch(campaignPda);
  //   console.log(campaignAfter.campaignAmountWithdrawn.toString());
  //   assert.equal(( await provider.connection.getBalance(campaignPda)).toString(), "2001719120");
  //   console.log("Campaign verified at:", campaignPda.toBase58());
  // });

  // it("Withdraw SOL", async () => {
  //   const [campaignPda] = PublicKey.findProgramAddressSync(
  //     [Buffer.from("CAMPAIGN"), admin.toBuffer()],  // admin = campaign_owner
  //     program.programId
  //   );

  //   const withdrawAmount = new anchor.BN(1000000000);

  //   const withdrawTx = await program.methods
  //     .withdraw(withdrawAmount)
  //     .accounts({       // ← MUST provide manually!
  //       user: user,
  //     })
  //     .rpc();

  //   console.log("Withdraw TX:", withdrawTx);

  //   const campaignAfter = await program.account.crowdCampaign.fetch(campaignPda);
  //   console.log(campaignAfter);

  //   assert.ok(campaignAfter.campaignAmountWithdrawn.toString() , withdrawAmount.toString());
  //   assert.ok((await provider.connection.getBalance(campaignPda)).toString(), new anchor.BN(1001719120).toString());
  //   console.log("✅ Withdraw successful. Withdrawn:", campaignAfter.campaignAmountWithdrawn.toString());
  // });


  // it("Cancel Campaign", async () => {
  //   const [campaignPda] = PublicKey.findProgramAddressSync(
  //     [Buffer.from("CAMPAIGN"), admin.toBuffer()],  // admin = campaign_owner
  //     program.programId
  //   );

  //   const cancelTx = await program.methods
  //     .cancelCampaign()
  //     .accounts({  
  //       user: user,
  //     })
  //     .rpc();

  //   console.log(" cancelTx : ", cancelTx);

  //   const campaignAfter = await program.account.crowdCampaign.fetch(campaignPda);
  //   console.log(campaignAfter);

  //   assert.ok(campaignAfter.campaignStatus.toString(), "1");
  // });

  // it("Refund", async () => {
  //   const [campaignPda] = PublicKey.findProgramAddressSync(
  //     [Buffer.from("CAMPAIGN"), admin.toBuffer()],  // admin = campaign_owner
  //     program.programId
  //   );
  //   const refund_account = anchor.web3.Keypair.generate();

  //   let initailBalance = await provider.connection.getBalance(refund_account.publicKey);

  //   const refundTx = await program.methods
  //     .refunds(new anchor.BN(1000000000))
  //     .accounts({
  //       user: user,
  //       toAccount: refund_account.publicKey
  //     })
  //     .rpc();


  //   console.log("Refund TX:", refundTx);
  //   const campaignAfter = await program.account.crowdCampaign.fetch(campaignPda);
  //   console.log(campaignAfter);
  //   let AfterBalance = await provider.connection.getBalance(refund_account.publicKey);
  //   assert.ok(AfterBalance > initailBalance , "Refund Successful");
  //   console.log("Refund successful.", campaignAfter);
  // });

    it("Claim Funds", async () => {
    const [campaignPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("CAMPAIGN"), admin.toBuffer()],  // admin = campaign_owner
      program.programId
    );

    let initailBalance = await provider.connection.getBalance(user);
    let pdaBalance = await provider.connection.getBalance(campaignPda);
    console.log("PDA Balance is : ", pdaBalance);
    console.log("Initial Balance : " , initailBalance);
    
    const ClaimTx = await program.methods
      .claimFunds()
      .accounts({
        user: user,
      })
      .rpc();


    console.log("Claim Funds TX:", ClaimTx);
    const campaignAfter = await program.account.crowdCampaign.fetch(campaignPda);
    console.log(campaignAfter);
    let AfterBalance = await provider.connection.getBalance(user);
     let AfterpdaBalance = await provider.connection.getBalance(campaignPda);
    console.log("PDA Balance is : ", AfterpdaBalance);
    console.log("After Balance is : ", AfterBalance);
    
    assert.ok(AfterBalance > initailBalance , "Claim Funds Successful");
    console.log("Claim Funds successful.", campaignAfter);
  });

});
