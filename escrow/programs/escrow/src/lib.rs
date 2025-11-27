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

    pub fn claim_escrow(ctx:Context<ClaimEscrow>) -> Result<()> {
        let escrow_key = ctx.accounts.escrow.key();
        let bump = ctx.accounts.escrow.bump;
        let seeds = &[b"vault", escrow_key.as_ref(), &[bump]];
        let signer = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TokenTransfer {
                from: ctx.accounts.vault.to_account_info(),
                to: ctx.accounts.reciever.to_account_info(),
                authority: ctx.accounts.vault_authority.to_account_info()
            },
            signer
        )

            let _ = transfer(cpi_ctx, ctx.accounts.escrow.amount);

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
pub struct InitializeEscrow<'info> {
    #[account(
        init, 
        payer = initializer,
        space = 8 + 32 + 32 + 32 + 8 +1 
    )]
    pub escrow: Account<'info, escrow>,

    #[account(mut)]
    pub initializer: Signer<'info>,

    #[account(mut)]
    pub initializer_token_account: Account<'info, TokenAccount>

    #[account(
        seeds = [b"vault", escrow_key().as_ref()],
        bump
    )]

    pub vault_authority:  UncheckedAccounts<'info>,

    #[acconts(mut
        init,
        payer: initializer,
        associated_token:mint = mint,
        associated_token:authority = vault_authority,
    )]

    pub vault: Accounts<'info, TokenAccount>

    pub mint: Accounts<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>

}

#[derive(Accounts)]
pub struct ClaimEscrow<'info> {
    #[accounts(mut, has_one = reciever)]

    pub escrow: Accounts<'info>, Escrow>,

    #[accounts(
        seeds = [b"vault", escrow.key().as_ref()],
        bump = escrow.bump
    )]

    pub vault_authority: UncheckedAccounts<'info>,

    #[accounts(mut)]
    pub vault: Accounts<'info, TokenAccount>

    #[accounts(mut)]
    pub reciever: Signer<'info>

    #[accounts(mut)]
    pub reciever_token_account: Accounts<'info, TokenAccount>

    #[accounts(mut)]
    pub token_program: Program<'info, Token> 
}

#{error_code}
pub enum EscrowError {
    #[msg("Escrow already initialized")]
    EscrowAlreadyInitialized,

    #[msg("Escrow not initialized")]
    EscrowNotInitialized,

    #[msg("Missing vault bump")]
    MissingVaultBump
}
