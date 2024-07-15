use anchor_lang::prelude::*;

declare_id!("CdYEPjxYcgi5n4hpVsNJJh1zhh9F7E8GBiH1JN92e4PU");

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
