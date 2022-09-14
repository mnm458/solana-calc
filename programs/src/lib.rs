use anchor_lang::prelude::*;
// use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("HTsFVYZ81meXsJAgeqPZKsXLeCcC8A8GkRLGTAso9m4g");

#[program]
pub mod calculatordapp {
    use super::*;
    pub fn create(ctx: Context<Create>, init_message: String) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.message = init_message;
        Ok(())
    }

    pub fn addition(ctx: Context<Addition>, num1: u32, num2: u32) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 as i64 + num2 as i64;
        Ok(())
    }

    pub fn subtraction(ctx: Context<Subtraction>, num1: u32, num2: u32) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 as i64 - num2 as i64;
        Ok(())
    }

    pub fn multiplication(ctx: Context<Multiplication>, num1: u32, num2: u32) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result= num1 as i64 * num2 as i64;
        Ok(())
    }

    pub fn division(ctx: Context<Division>, num1: u32, num2: u32) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 as i64 / num2 as i64;
        calculator.remander = num1 as i64 % num2 as i64;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = user, space = 256)]
    pub calculator: Account<'info, Calculator>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Addition<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

#[derive(Accounts)]
pub struct Subtraction<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

#[derive(Accounts)]
pub struct Multiplication<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

#[derive(Accounts)]
pub struct Division<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}