use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
// use anchor_lang::system_program::{self, Transfer};

declare_id!("85rgQQX1eVG8VEMK6PMWjQgrhU8LoLjrJxMGZFfQXsL4");

#[program]
pub mod campaign_contract {
    use super::*;

    pub fn create_campaign(
        ctx: Context<CreateCampaign>,
        name: String,
        target_amount: u64,
        last_date: i64,
    ) -> ProgramResult {
        let new_campaign = &mut ctx.accounts.campaign;
        let now = Clock::get()?.unix_timestamp;
        if last_date <= now + 86400 {
            msg!("The campaign end date must be at least 24 hours in the future.");
            return Err(ProgramError::InvalidArgument);
        }
        new_campaign.campaign_owner = *ctx.accounts.admin.key;
        new_campaign.campaign_name = name;
        new_campaign.campaign_target_amount = target_amount;
        new_campaign.campaign_last_date = last_date;
        new_campaign.campaign_status = 0; // campaign is active
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> ProgramResult {
        let campaign = &mut ctx.accounts.campaign;
        let user = &mut ctx.accounts.user;

        if campaign.campaign_owner != *user.key {
            return Err(ProgramError::IllegalOwner);
        }

        // let rent = Rent::get()?;
        // let minimum_balance = rent.minimum_balance(campaign.to_account_info().data_len());
        // let available_balance = campaign.to_account_info().lamports() - minimum_balance;

        // if available_balance < amount {
        //     return Err(ProgramError::InsufficientFunds);
        // }

        // let bump = ctx.bumps.campaign;

        // let seeds: &[&[u8]] = &[b"CAMPAIGN", campaign.campaign_owner.as_ref(), &[bump]];
        // let signer_seeds = &[&seeds[..]];

        // let cpi_accounts = Transfer {
        //     from: campaign.to_account_info(),
        //     to: user.to_account_info(),
        // };

        // let cpi_ctx = CpiContext::new_with_signer(
        //     ctx.accounts.system_program.to_account_info(),
        //     cpi_accounts,
        //     signer_seeds,
        // );

        // system_program::transfer(cpi_ctx, amount)?;

        // ctx.accounts.campaign.campaign_amount_withdrawn += amount;

        let minimum_balance = Rent::get()?.minimum_balance(campaign.to_account_info().data_len());

        if **campaign.to_account_info().lamports.borrow() - minimum_balance < amount {
            return Err(ProgramError::InsufficientFunds);
        }

        **campaign.to_account_info().try_borrow_mut_lamports()? -= amount;
        **user.try_borrow_mut_lamports()? += amount;

        campaign.campaign_amount_withdrawn += amount;

        Ok(())

    }

    pub fn deposited(ctx: Context<Deposite>, amount: u64) -> ProgramResult {
        let campaign = &mut ctx.accounts.campaign;

        if campaign.campaign_status != 0 {
            msg!("Cannot deposit to a non-active campaign.");
            return Err(ProgramError::InvalidArgument);
        }

        let ix = system_instruction::transfer(
            &ctx.accounts.user.key(),
            &ctx.accounts.campaign.key(),
            amount,
        );

        // Store the result of the invoke function call
        let result = anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.campaign.to_account_info(),
            ],
        );

        // Check if the invoke operation was successful
        if let Err(e) = result {
            return Err(e.into()); // Convert the error to a ProgramResult
        }

        Ok(())
    }

    pub fn claim_funds(ctx: Context<ClaimFunds>) -> ProgramResult {
        let campaign = &mut ctx.accounts.campaign;
        let user = &mut ctx.accounts.user;

        if campaign.campaign_owner != *user.key {
            return Err(ProgramError::IllegalOwner);
        }

        // let deadline = campaign.campaign_last_date;
        // let now = Clock::get()?.unix_timestamp;

        // if now < deadline {
        //     return Err(ProgramError::InvalidArgument);
        // }

        let amount = campaign.to_account_info().lamports();
        let minimum_balance = Rent::get()?.minimum_balance(campaign.to_account_info().data_len());
        let claimable_amount = amount - minimum_balance;

        **campaign.to_account_info().try_borrow_mut_lamports()? -= claimable_amount;
        **user.try_borrow_mut_lamports()? += claimable_amount;

        campaign.campaign_status = 2; // mark campaign as successful
        campaign.campaign_amount_withdrawn = claimable_amount;

        Ok(())

    }

    pub fn cancel_campaign(ctx: Context<CancelCampaign>) -> ProgramResult {
        let campaign = &mut ctx.accounts.campaign;
        let user = &mut ctx.accounts.user;

        if campaign.campaign_owner != *user.key {
            return Err(ProgramError::IllegalOwner);
        }
        campaign.campaign_last_date = Clock::get()?.unix_timestamp; // set deadline to now
        campaign.campaign_status = 1; // mark campaign as cancelled

        Ok(())

    }

    pub fn refunds(ctx: Context<Refunds>, amount: u64) -> ProgramResult {
        let campaign = &mut ctx.accounts.campaign;
        let user = &mut ctx.accounts.user;
        let to = &mut ctx.accounts.to_account;

        if campaign.campaign_owner != *user.key {
            return Err(ProgramError::IllegalOwner);
        }

        let minimum_balance = Rent::get()?.minimum_balance(campaign.to_account_info().data_len());

        if **campaign.to_account_info().lamports.borrow() - minimum_balance < amount {
            return Err(ProgramError::InsufficientFunds);
        }

        campaign.campaign_amount_withdrawn += amount;

        **campaign.to_account_info().try_borrow_mut_lamports()? -= amount;
        **to.try_borrow_mut_lamports()? += amount;

        // **ctx
        //     .accounts
        //     .campaign
        //     .to_account_info()
        //     .lamports
        //     .borrow_mut() -= amount;
        // **ctx.accounts.to.to_account_info().lamports.borrow_mut() += amount;

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
    pub campaign: Account<'info, CrowdCampaign>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(
        mut,
        seeds = [b"CAMPAIGN", user.key().as_ref()],
        bump
    )]
    pub campaign: Account<'info, CrowdCampaign>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposite<'info> {
    #[account(mut)]
    pub campaign: Account<'info, CrowdCampaign>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClaimFunds<'info> {
    #[account(
        mut,
        seeds = [b"CAMPAIGN", user.key().as_ref()],
        bump,
        // constraint = clock.unix_timestamp <= campaign.campaign_last_date @ ErrorCode::CampaignActive
    )]
    pub campaign: Account<'info, CrowdCampaign>,
    #[account(mut, constraint = campaign.campaign_owner == user.key() @ ErrorCode::IllegalOwner)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub clock: Sysvar<'info, Clock>,
}

#[derive(Accounts)]
pub struct CancelCampaign<'info> {
    #[account(
        mut,
        seeds = [b"CAMPAIGN", user.key().as_ref()],
        bump
    )]
    pub campaign: Account<'info, CrowdCampaign>,
    #[account(mut, constraint = campaign.campaign_owner == user.key() @ ErrorCode::IllegalOwner)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub clock: Sysvar<'info, Clock>,
}

#[derive(Accounts)]
pub struct Refunds<'info> {
    #[account(
        mut,
        seeds = [b"CAMPAIGN", user.key().as_ref()],
        bump
    )]
    pub campaign: Account<'info, CrowdCampaign>,
    #[account(mut, constraint = campaign.campaign_owner == user.key() @ ErrorCode::IllegalOwner)]
    pub user: Signer<'info>,
    /// CHECK: Refund destination can be any valid Solana account
    #[account(mut)]
    pub to_account: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub clock: Sysvar<'info, Clock>,
}

#[account]
#[derive(InitSpace)]
pub struct CrowdCampaign {
    pub campaign_owner: Pubkey,
    #[max_len(50)]
    pub campaign_name: String,
    pub campaign_target_amount: u64,
    pub campaign_last_date: i64,
    pub campaign_amount_withdrawn: u64,
    pub campaign_status: u8, // 0 = active, 1 = cancelled, 2 = successful
}

#[error_code]
pub enum ErrorCode {
    #[msg("The campaign is still active.")]
    CampaignActive,
    #[msg("You are not the owner of this campaign.")]
    IllegalOwner,
}
