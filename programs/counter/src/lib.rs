use anchor_lang::prelude::*;

declare_id!("Eo9q6BCYbEmg6r96YGT8jZvtJv7MYDGPacA12LVju2Vt");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
