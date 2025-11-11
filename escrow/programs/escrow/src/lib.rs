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
        
    }


        Ok(())
    }

#[derive(Accounts)]
pub struct Initialize {}
