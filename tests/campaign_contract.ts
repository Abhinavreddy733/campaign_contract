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

  it("Create Campaign", async () => {

    const name = "Test Campaign";
    const targetAmount = new anchor.BN(1000000000);
    const lastDate = new anchor.BN(1735689600);
    const tx = await program.methods
      .createCampaign(name, targetAmount, lastDate)
      .accounts({
        admin: admin
      })
      .rpc();
    console.log("Your transaction signature", tx);

    const [campaignPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("CAMPAIGN"), admin.toBuffer()],
      program.programId
    );

    const campaign = await program.account.crowdCampaign.fetch(campaignPda);

    assert.equal(campaign.campaignName, name);
    console.log("Campaign verified at:", campaignPda.toBase58());
  });
});
