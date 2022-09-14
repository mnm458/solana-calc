use anchor_lang::prelude::*;
// use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod program {
    use super::*;
    pub fn create(ctx: Context<Create>) -> Result<()> {
        let campaign_details = &mut ctx.accounts.campaign_details;
        campaign_details.amount_donated = 0;
        Ok(())
    }

    pub fn create_campaign(ctx: Context<Addition>) -> Result<()> {
        let campaign_details = &mut ctx.accounts.campaign_details;
        campaign_details.amount_donated = 0;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Addition>) -> Result<()> {
        Ok(())
    }

    pub fn donate(ctx: Context<Addition>) -> Result<()> {
        Ok(())
    }
   
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = user, space = 256)]
    pub campaign_details: Account<'info, CampaignDetails>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
