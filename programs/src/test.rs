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