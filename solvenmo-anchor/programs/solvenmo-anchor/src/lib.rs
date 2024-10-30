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

    pub fn deposit_funds(ctx: Context<DepositFunds>, amount: u64) -> Result<()> {
        require!(amount > 0, ErrorCode::InvalidAmount);

        let ix = system_instruction::transfer(ctx.accounts.signer.key(), ctx.accounts.cash_account.to_account_info().key, amount);

        invoke(
            &ix,
            &[ctx.accounts.signer.clone(), ctx.accounts.cash_account.to_account_info()],
        )?;

        Ok(())
    }

    pub fn withdraw_funds(ctx: Context<WithdrawFunds>, amount: u64) -> Result<()> {
        require!(amount > 0, ErrorCode::InvalidAmount);

        let cash_account = &mut ctx.accounts.cash_account;
        let wallet = &mut ctx.accounts.signer.to_account_info();

        require!(*cash_account.owner == ctx.accounts.signer.key(), ErrorCode::InvalidSigner);

        **cash_account.try_borrow_mut_lamports()? -= amount;
        **wallet.try_borrow_mut_lamports()? += amount;

        Ok(())
    }

    pub fn transfer_funds(
        ctx: Context<TransferFunds>,
        _recipient: Pubkey,
        amount: u64,
    ) -> Result<()> {
        require!(amount > 0, ErrorCode::InvalidAmount);

        let from_cash_account = &mut ctx.accounts.from_cash_account.to_account_info();
        let to_cash_account = &mut ctx.accounts.to_cash_account.to_account_info();

        require!(*cash_account.owner == ctx.accounts.signer.key(), ErrorCode::InvalidSigner);

        **from_cash_account.try_borrow_mut_lamports()? -= amount;
        **to_cash_account.try_borrow_mut_lamports()? += amount;

        Ok(())
    }
    pub fn add_friend(ctx: Context<AddFriend>, pubkey: Pubkey) -> Result<()> {
        let cash_account = &mut ctx.accounts.cash_account;
        cash_account.friends.push(pubkey);
        Ok(())
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

#[derive(Accounts)]
pub struct DepositFunds<'info> {
    #[account(
        mut,
        seeds = [b"cash-account". signer.key().as_ref()],
        bump,
    )]
    pub cash_account: Account<'info, CashAccount>,
    #[account(mut)]
    pub signer: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawFunds<'info> {
    #[account(
        mut,
        seeds = [b"cash-account", signer.key().as_ref()],
        bump,
    )]
    pub cash_account: Account<'info, CashAccount>,
    #[account(mut)]
    pub signer: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(recipient: Pubkey)]
pub struct TransferFunds<'info> {
    #[account(
        mut,
        seeds = [b"cash-account", signer.key().as_ref()],
        bump,
    )]
    pub cash_account: Account<'info, CashAccount>,
    #[account(
        mut,
        seeds = [b"cash-account", recipient.as_ref()],
        bump
    )]
    pub to_cash_account: Account<'info, CashAccount>,
    pub system_program: Program<'info, System>,
    pub signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct AddFriend<'info> {
    #[account(
        mut,
        seeds = [b"cash-account", signer.key().as_ref()],
        bump,
    )]
    pub cash_account: Account<'info, CashAccount>,
    #[account(mut)]
    pub signer: AccountInfo<'info>,
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
    #[msg("Signer does not have access to call this instruction.")]
    InvalidSigner,
}
