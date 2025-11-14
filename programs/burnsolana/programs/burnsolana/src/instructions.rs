use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer, transfer};
use anchor_spl::token::{self, CloseAccount};
use anchor_spl::token_2022::spl_token_2022::state::AccountState;
use anchor_spl::token_interface::BurnChecked;
use anchor_lang::context::CpiContext;

use crate::context::*;
use crate::states::*;
use crate::errors::ErrorCode;

/// 1.1 PDA SPL STATS
pub fn initialize_user_stats_spl(ctx: Context<InitializeUserStatsSpl>) -> Result<()> {
    let stats = &mut ctx.accounts.user_stats_spl;
    stats.bump = ctx.bumps.user_stats_spl;
    stats.user = ctx.accounts.user.key();
    stats.mint = ctx.accounts.mint.key();
    stats.total_amount = 0;
    stats.burn_count = 0;
    Ok(())
}
/// 1.2 PDA SOL STATS
pub fn initialize_user_stats_sol(ctx: Context<InitializeUserStatsSol>) -> Result<()> {
    let stats = &mut ctx.accounts.user_stats_sol;
    stats.bump = ctx.bumps.user_stats_sol;
    stats.user = ctx.accounts.user.key();
    stats.mint = System::id(); 
    stats.total_amount = 0;
    stats.burn_count = 0;
    Ok(())
}
///---------------------------------------------------------------------------
/// 2 - burn an SPL token on Solana and update UserStats
pub fn burn_spl(
    ctx: Context<BurnSpl>,
    amount: u64,
    message: String,
) -> Result<()> {
    let user = &ctx.accounts.user;
    let token_account = &ctx.accounts.token_account;
    let mint = &ctx.accounts.mint;
    let user_stats = &mut ctx.accounts.user_stats_spl;

    // CHECKS
    require!(mint.is_initialized, ErrorCode::MintNotInitialezed); 
    require!(mint.mint_authority.is_none(), ErrorCode::MintAuthorityPresent); 
    require!(token_account.state != AccountState::Frozen, ErrorCode::TokenAccountFrozen); 

    require!(message.as_bytes().len() <= 256, ErrorCode::MessageTooLong);

    require_keys_eq!(token_account.mint, mint.key(), ErrorCode::MintMismatch);
    require_keys_eq!(token_account.owner, user.key(), ErrorCode::NotAccountOwner);

    // Balance checks
    require!(amount > 0, ErrorCode::AmountMustBePositive);
    require!(token_account.amount >= amount, ErrorCode::InsufficientFunds);

    // verify that both mint and token_account are owned by the token program that was passed in the accounts. 
    let passed_program = ctx.accounts.token_program.key();
    require_keys_eq!(*mint.to_account_info().owner, passed_program, ErrorCode::WrongTokenProgram);
    require_keys_eq!(*token_account.to_account_info().owner, passed_program, ErrorCode::WrongTokenProgram);
    
    //fee
    let lamports = anchor_lang::Lamports::get_lamports(&user.to_account_info());
    require!(&lamports > &FEE_LAMPORTS, ErrorCode::InsufficientFunds);
   
    let cpi_ctx_fee = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        Transfer {
            from: ctx.accounts.user.to_account_info(),
            to: ctx.accounts.fee_vault.to_account_info(),
        }
    );
    transfer(cpi_ctx_fee, FEE_LAMPORTS)?;

    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(), 
        BurnChecked {
            mint: mint.to_account_info(),
            from: token_account.to_account_info(),
            authority: user.to_account_info(),
        }
    );

    let decimals = mint.decimals;
    
    anchor_spl::token_interface::burn_checked(cpi_ctx, amount, decimals)?;

    // Update user_stats
    user_stats.total_amount = user_stats.total_amount.checked_add(amount).ok_or(ErrorCode::Overflow)?;
    user_stats.burn_count = user_stats.burn_count.checked_add(1).ok_or(ErrorCode::Overflow)?;

    msg!(&message);

    // Emit logs on-chain for history and analytics
    emit!(BurnEvent {
        user: user.key(),
        mint: mint.key(),
        amount,
        message: message.clone(),
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}

/// 3 - burn SOL by transferring to the system account
pub fn burn_sol(ctx: Context<BurnSol>, lamports: u64, message: String) -> Result<()> {
    require!(message.as_bytes().len() <= 256, ErrorCode::MessageTooLong);
    // Balance check
    require!(&lamports > &FEE_LAMPORTS, ErrorCode::InsufficientFunds);
    let user = &ctx.accounts.user;

    // 1) If the user’s WSOL ATA is provided — close the WSOL account (CPI к SPL Token).
    if let Some(wsol_ata) = ctx.accounts.user_wsol_ata.as_ref() {
        // Safety checks
        require_keys_eq!(wsol_ata.mint, WSOL, ErrorCode::MintMismatch);
        require_keys_eq!(wsol_ata.owner, ctx.accounts.user.key(), ErrorCode::NotAccountOwner);

        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            CloseAccount {
                account: wsol_ata.to_account_info(),
                destination: user.to_account_info(),
                authority: user.to_account_info(),
            });
        token::close_account(cpi_ctx)?; // unwrap WSOL -> SOL 
    };

    // 2) After optional unwrap, the user must have enough lamports.
    let user_lamports_after = **ctx.accounts.user.to_account_info().lamports.borrow();
    require!(user_lamports_after >= lamports, ErrorCode::InsufficientFunds);

    let burn_amount = lamports.checked_sub(FEE_LAMPORTS).ok_or(ErrorCode::Overflow);

    let cpi_ctx_fee = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        Transfer {
            from: ctx.accounts.user.to_account_info(),
            to: ctx.accounts.fee_vault.to_account_info(),
        }
    );

    transfer(cpi_ctx_fee, FEE_LAMPORTS)?;

    let cpi_ctx_burn = CpiContext::new(
        ctx.accounts.system_program.to_account_info(), 
        Transfer {
            from: ctx.accounts.user.to_account_info(),
            to: ctx.accounts.burn_account.to_account_info(), 
        },
    );

    transfer(cpi_ctx_burn, burn_amount.unwrap())?;

    // update UserStats
    let stats = &mut ctx.accounts.user_stats_sol;
    stats.burn_count = stats.burn_count.checked_add(1).ok_or(ErrorCode::Overflow)?;
    stats.total_amount = stats.total_amount.checked_add(lamports).ok_or(ErrorCode::Overflow)?;

    msg!(&message);

    emit!(BurnEvent {
        user: stats.user,
        mint: stats.mint,
        amount: lamports,
        message: message.clone(),
        timestamp: Clock::get()?.unix_timestamp,
    });
    Ok(())
}

// 4) Withdraw fee
pub fn withdraw_fee(ctx: Context<WithdrawFee>, lamports: u64) -> Result<()> {
    // admin authorization
    require!(ADMIN_KEYS.contains(&ctx.accounts.admin.key()), ErrorCode::Unauthorized);

    let rent_min = ctx.accounts.rent.minimum_balance(8);
    let current_balance = ctx.accounts.fee_vault.get_lamports();
    require!(lamports < current_balance.saturating_sub(rent_min), ErrorCode::InsufficientFunds);

    let from_ai = ctx.accounts.fee_vault.to_account_info();
    let to_ai = ctx.accounts.to.to_account_info();

    **from_ai.try_borrow_mut_lamports()? -= lamports;
    **to_ai.try_borrow_mut_lamports()? += lamports;

    Ok(())
}
