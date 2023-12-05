use anchor_lang::prelude::*;

declare_id!("5cv55P8Ndpsr9DomaUQhp8EzttgP5PUXgWQheKCgUew3");

#[program]
pub mod mycalculatorapp {
    use super::*;
    pub fn create(ctx: Context<Create>, init_message: String) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.greeting = init_message;
        Ok(())
    }

    pub fn add(ctx: Context<Addition>, number1: i64 , number2: i64) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = number1 + number2;
        Ok(())
    }

    pub fn sub(ctx: Context<Sub>, num1: i64 , num2: i64) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 - num2;
        Ok(())
    }

    pub fn div(ctx: Context<Div>, num1: i64 , num2: i64) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 / num2;
        calculator.remainder = num1 % num2;
        Ok(())
    }

    pub fn mul(ctx: Context<Mul>, num1: i64 , num2: i64) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 * num2;
        Ok(())
    }
}



#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer=user, space=264)]
    pub calculator: Account<'info, Calculator>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Addition<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>
}

#[derive(Accounts)]
pub struct Sub<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>
}

#[derive(Accounts)]
pub struct Div<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>
}

#[derive(Accounts)]
pub struct Mul<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>
}

#[account]
pub struct Calculator {
    pub greeting: String,
    pub result: i64,
    pub remainder: i64
}