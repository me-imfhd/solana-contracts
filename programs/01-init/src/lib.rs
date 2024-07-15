use anchor_lang::prelude::*;

declare_id!("ARTXSwB4wPMK5eCojBuM4UsXALiUzW7BP6Xf6Fz9Rb6w");

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
