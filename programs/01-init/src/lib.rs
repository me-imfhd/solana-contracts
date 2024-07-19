use anchor_lang::prelude::*;

declare_id!("EDZqxLDVBhzu5V5zBw6YcfJ91tuX5ruCwSaLp6TwJ1zb");

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
