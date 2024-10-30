use anchor_lang::prelude::*;

declare_id!("5aGwP2KPT9embJfJgc2zSgxYc836HBWbVceZbykPnGPg");

#[program]
pub mod solvenmo_anchor {
    use super::*;

    pub fn initialize_account(ctx: Context<Initialize>) -> Result<()> {
        let cash_account = &mut ctx.accounts.cash_account;
        cash_account.owner = ctx.accounts.signer.key;
        cash_account.friends = Vec::new();
    }
}

#[derive(Accounts)]
pub struct InitializeAccount<'info> {
    #[account(
        init,
        seeds = [b"cash_account", signer.key().as_ref()],
        bump,
        payer = signer,
        space = CashAccount::INIT_SPACE,
    )]
    pub cash_account: Account<'info, CashAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct CashAccount {
    pub owner: Pubkey,
    #[max_len(100)]
    pub friends: Vec<Pubkey>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("The provided amount must be greater than 0")]
    InvalidAmount,

}
