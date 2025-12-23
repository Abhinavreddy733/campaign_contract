use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("85rgQQX1eVG8VEMK6PMWjQgrhU8LoLjrJxMGZFfQXsL4");

#[program]
pub mod campaign_contract {
    use super::*;

    pub fn create_campaign(
        ctx: Context<CreateCampaign>,
        name: String,
        target_amount: u64,
        last_date: i64
    ) -> ProgramResult {
        let new_campaign = &mut ctx.accounts.campaign;
        new_campaign.campaign_owner = *ctx.accounts.admin.key;
        new_campaign.campaign_name = name;
        new_campaign.campaign_target_amount = target_amount;
        new_campaign.campaign_last_date = last_date;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateCampaign<'info> {
    #[account(
        init,
        payer = admin,
        space = 8 + CrowdCampaign::INIT_SPACE,
        seeds = [b"CAMPAIGN".as_ref() , admin.key().as_ref()],
        bump
    )]
    pub campaign : Account<'info , CrowdCampaign>,
    #[account(mut)]
    pub admin : Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct CrowdCampaign {
    pub campaign_owner: Pubkey,
    #[max_len(50)]
    pub campaign_name: String,
    pub campaign_target_amount: u64,
    pub campaign_last_date: i64,
    pub campaign_amount_collected: u64,
    pub campaign_amount_withdrawn: u64,
} 
