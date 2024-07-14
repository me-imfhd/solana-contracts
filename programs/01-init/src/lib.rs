use anchor_lang::prelude::*;

declare_id!("4FvUxKS5t2NaEGkuSnqrf4rZhtaet1S4obj7negZQvV6");

#[program]
pub mod init {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
