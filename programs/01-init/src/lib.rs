use anchor_lang::prelude::*;

declare_id!("8gMSznWbWzyv1gNNZaDvBNNsYidHuL5dBJ3GxcwJwnaM");

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
