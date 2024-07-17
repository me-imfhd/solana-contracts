use anchor_lang::prelude::*;

declare_id!("ALwDjEwuMGbh3hRN9SG9GjxvxKvFcTLaE49YmPDgLGS4");

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
