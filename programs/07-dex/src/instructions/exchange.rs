use std::ops::{ Add, Div, Mul };

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::Token2022,
    token_interface::{ Mint, TokenAccount, transfer_checked, TransferChecked },
};

use crate::{ ExchangeError, LiquidityPool };
/// Base means the asset im buying
/// Quote means the asset im paying with
/// receiving_base_amount = (total_base_quantity * buy_amount) / (total_quote_quantity + buy_amount)
/// Derieved from `Pool = a * b * c * d * Quote * Base`
pub fn exchange(ctx: Context<Exchange>, buy_amount: u64) -> Result<()> {
    ctx.accounts.check_assets_exist()?;
    ctx.accounts.exchange_assets(buy_amount)?;
    Ok(())
}
#[derive(Accounts)]
pub struct Exchange<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    mint_base: InterfaceAccount<'info, Mint>,
    mint_quote: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint_base,
        associated_token::authority = pool,
        associated_token::token_program = token_program
    )]
    pool_token_account_base: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_quote,
        associated_token::authority = pool,
        associated_token::token_program = token_program,
    )]
    pool_token_account_quote: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint_base,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    buyer_token_account_base: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut, 
        associated_token::mint = mint_quote,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    buyer_token_account_quote: InterfaceAccount<'info, TokenAccount>,
    #[account(mut, seeds = [LiquidityPool::SEED_PREFIX.as_bytes()], bump)]
    pool: Account<'info, LiquidityPool>,
    token_program: Program<'info, Token2022>,
    system_program: Program<'info, System>,
    associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Exchange<'info> {
    fn check_assets_exist(&self) -> Result<()> {
        require!(self.pool.assets.contains(&self.mint_base.key()), ExchangeError::InvalidAssetKey);
        require!(self.pool.assets.contains(&self.mint_quote.key()), ExchangeError::InvalidAssetKey);
        Ok(())
    }
    fn calculate_receiving_base_amount(&self, buy_amount: u64) -> Result<u64> {
        let total_base_quantity = convert_to_float(
            self.pool_token_account_base.amount,
            self.mint_base.decimals
        );
        msg!("Initial Base Quantity In Pool: {}", total_base_quantity);
        let total_quote_quantity = convert_to_float(
            self.pool_token_account_quote.amount,
            self.mint_quote.decimals
        );
        msg!("Initial Quote Quantity In Pool: {}", total_quote_quantity);
        let buy_amount = convert_to_float(buy_amount, self.mint_quote.decimals);
        msg!("Buying at quote: {}", buy_amount);

        let receiving_base_amount = total_base_quantity
            .mul(buy_amount)
            .div(total_quote_quantity.add(buy_amount));
        require!(
            receiving_base_amount < total_base_quantity,
            ExchangeError::InvalidSwapNotEnoughLiquidity
        );
        Ok(convert_from_float(receiving_base_amount, self.mint_base.decimals))
    }
    fn exchange_assets(&mut self, buy_amount: u64) -> Result<()> {
        let receiving_base_amount = self.calculate_receiving_base_amount(buy_amount)?;
        msg!(
            "\nInitial User Quote Balance: {}",
            convert_to_float(self.buyer_token_account_quote.amount, self.mint_quote.decimals)
        );
        msg!(
            "Initial User Base Balance: {}",
            convert_to_float(self.buyer_token_account_base.amount, self.mint_base.decimals)
        );
        msg!(
            "Exchanging {} {} for {} {}", // e.g, Exchanging 10 BONK for 1 USDC
            receiving_base_amount,
            self.mint_base.key(),
            buy_amount,
            self.mint_quote.key()
        );
        // Buyer Pays Quote
        transfer_checked(
            CpiContext::new(self.token_program.to_account_info(), TransferChecked {
                authority: self.signer.to_account_info(),
                from: self.buyer_token_account_quote.to_account_info(),
                mint: self.mint_quote.to_account_info(),
                to: self.pool_token_account_quote.to_account_info(),
            }),
            buy_amount,
            self.mint_quote.decimals
        )?;
        let signer_seeds: &[&[&[u8]]] = &[
            &[LiquidityPool::SEED_PREFIX.as_bytes(), &[self.pool.bump]],
        ];
        // Buyer Receives Base
        transfer_checked(
            CpiContext::new(self.token_program.to_account_info(), TransferChecked {
                authority: self.pool.to_account_info(),
                from: self.pool_token_account_base.to_account_info(),
                mint: self.mint_base.to_account_info(),
                to: self.buyer_token_account_base.to_account_info(),
            }).with_signer(signer_seeds),
            receiving_base_amount,
            self.mint_base.decimals
        )?;
        let total_base_quantity = convert_to_float(
            self.pool_token_account_base.amount,
            self.mint_base.decimals
        );
        msg!(
            "Final User Quote Balance: {}",
            convert_to_float(self.buyer_token_account_quote.amount, self.mint_quote.decimals)
        );
        msg!(
            "Final User Base Balance: {}\n",
            convert_to_float(self.buyer_token_account_base.amount, self.mint_base.decimals)
        );
        msg!("Final Base Quantity In Pool: {}", total_base_quantity);
        let total_quote_quantity = convert_to_float(
            self.pool_token_account_quote.amount,
            self.mint_quote.decimals
        );
        msg!("Final Quote Quantity In Pool : {}", total_quote_quantity);
        Ok(())
    }
}

fn convert_to_float(value: u64, decimals: u8) -> f32 {
    (value as f32).div(f32::powf(10.0, decimals as f32))
}

fn convert_from_float(value: f32, decimals: u8) -> u64 {
    value.mul(f32::powf(10.0, decimals as f32)) as u64
}
