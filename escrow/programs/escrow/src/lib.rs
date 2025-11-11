use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Token, TokenAccount, Mint, transfer, Transfer as TokenTransfer},
    associated_token::{AssociatedToken}
};

declare_id!("4ZJhp4PKHnQYcnbigqvYo6TW8mkMinDgYKVzZWuJWnbC");

#[program]
pub mod escrow {
    use super::*;

    pub fn Initialize_escrow(ctx:Content<InitializeEscrow>, amount: u64, reciever: Pubkey) -> Result<()> {
        let escrow = &mut ctx:amount.escrow;
        escrow.Initializer = ctx.accounts.Initializer.key();
        escrow.reciever = reciever;
        escrow.mint = ctx.accounts.mint.key();
        escrow.amount = amount;
        escrow.bump = ctx.bump.vault_authority;

        let cpi_ctx = cpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TokenTransfer {
                from: ctx.accounts.initializer_token_account.to_account_info(),
                to: ctx.accounts.vault.to_account_info(),
                authority: ctx.accounts.initializer.to_account_info()
            }
        );

        let_ = transfer(cpi_ctx, amount)
        
        Ok(())

       }
    }

#[account]
pub struct Escrow {
    pub initializer: pubKey,
    pub mint: pubKey,
    pub reciever: pubKey,
    pub amount: u64,
    pub bump: u8,
}

#[derive(Accounts)]
pub struct InitializeEscrow {}
