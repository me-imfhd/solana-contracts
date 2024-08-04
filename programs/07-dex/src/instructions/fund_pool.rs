use anchor_lang::{ prelude::*, system_program };
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::Token2022,
    token_interface::{ Mint, TokenAccount, TransferChecked, transfer_checked },
};
use crate::LiquidityPool;

pub fn fund_pool(ctx: Context<FundPool>, amount: u64) -> Result<()> {
    ctx.accounts.add_asset()?;
    ctx.accounts.fund_pool(amount)?;
    Ok(())
}
#[derive(Accounts)]
pub struct FundPool<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    mint: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = pool,
        associated_token::token_program = token_program
    )]
    pool_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut, 
        constraint = mint.key() == funder_token_account.mint.key() &&
        signer.key() == funder_token_account.owner.key()
    )]
    funder_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut, seeds = [LiquidityPool::SEED_PREFIX.as_bytes()], bump)]
    pool: Account<'info, LiquidityPool>,
    system_program: Program<'info, System>,
    token_program: Program<'info, Token2022>,
    associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> FundPool<'info> {
    fn add_asset(&mut self) -> Result<()> {
        if let false = self.pool.assets.contains(&self.mint.key()) {
            self.realloc(32)?;
            self.pool.assets.push(self.mint.key());
        }
        Ok(())
    }
    fn realloc(&mut self, space_to_add: usize) -> Result<()> {
        let account_info = self.pool.to_account_info();
        let new_account_size = account_info.data_len() + space_to_add;
        let lamports_required = Rent::get()?.minimum_balance(new_account_size);
        let additional_rent_to_fund = lamports_required - account_info.lamports();
        system_program::transfer(
            CpiContext::new(self.system_program.to_account_info(), system_program::Transfer {
                from: self.signer.to_account_info(),
                to: account_info.clone(),
            }),
            additional_rent_to_fund
        )?;
        account_info.realloc(new_account_size, false)?;
        Ok(())
    }
    fn fund_pool(&mut self, amount: u64) -> Result<()> {
        transfer_checked(
            CpiContext::new(self.token_program.to_account_info(), TransferChecked {
                authority: self.signer.to_account_info(),
                from: self.funder_token_account.to_account_info(),
                mint: self.mint.to_account_info(),
                to: self.pool_token_account.to_account_info(),
            }),
            amount,
            self.mint.decimals
        )?;
        Ok(())
    }
}
