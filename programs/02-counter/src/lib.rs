use anchor_lang::prelude::*;

declare_id!("G7awV9Rp6ryWn1fj8wWAP1jjhGF6zqVh1Bs7bW8QNVTQ");

#[program]
pub mod counter {
    use super::*;

    pub fn create_counter(ctx: Context<CreateCounter>) -> Result<()> {
        msg!("Creating an counter!");
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        counter.user = ctx.accounts.user.key(); // setting signer as user for counter, (when updating we can check if signer and this are same)
        msg!("Current count is {}", counter.count);
        msg!("The Admin PubKey is: {} ", counter.user);
        Ok(())
    }
    pub fn increment_counter(ctx: Context<UpdateCounter>) -> Result<()> {
        msg!("Incrementing the counter!");
        let counter = &mut ctx.accounts.counter;
        counter.count += 1;
        msg!("Current count is {}", counter.count);
        Ok(())
    }
    pub fn decrement_counter(ctx: Context<UpdateCounter>) -> Result<()> {
        msg!("Decrementing the counter!");
        let counter = &mut ctx.accounts.counter;
        counter.count -= 1;
        msg!("Current count is {}", counter.count);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateCounter<'info> {
    #[account(mut)]
    user: Signer<'info>,
    #[account(init, payer = user, space = 8 + 32 + 1)]
    counter: Account<'info, Counter>,
    system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct UpdateCounter<'info> {
    user: Signer<'info>,
    #[account(mut, has_one = user)] // this validates that if the user was set properly or not when creating counter
    counter: Account<'info, Counter>,
}

#[account]
pub struct Counter {
    user: Pubkey,
    count: u8,
}
