use crate::states::*;
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{ Mint, TokenInterface, TokenAccount};

pub const INCINERATOR: Pubkey = pubkey!("1nc1nerator11111111111111111111111111111111");
pub const WSOL: Pubkey = pubkey!("So11111111111111111111111111111111111111112");

pub const FEE_LAMPORTS: u64 = 4_000_000; // 0.004 sol fee

// allowed admins who can withdraw the collected fee
pub const ADMIN_KEYS: [Pubkey; 3] = [
    pubkey!("CfPH1CUwMNzm9WjVVkn6mDy7FwufSVvvhb8CDVHuz3yu"),
    pubkey!("D62wnUEKxn7ukixVsxscn7fuFWW8ZZ2UgHATcytWWsBd"),
    pubkey!("HxpubkaPUDh8hQWrLrMhuECkscgY4aNpDGZb2gG4MhTo"),
];

#[derive(Accounts)]
pub struct InitializeUserStatsSpl<'info> {
    /// PDA per (user, mint)
    #[account(
        init, 
        payer = user, 
        space = 8 + UserStats::MAX_SIZE,
        seeds=[b"user_stats", user.key().as_ref(), mint.key().as_ref()], 
        bump
    )]
    pub user_stats_spl: Account<'info, UserStats>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeUserStatsSol<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + UserStats::MAX_SIZE,
        seeds = [b"user_stats", user.key().as_ref(), b"sol"],
        bump
    )]
    pub user_stats_sol: Account<'info, UserStats>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeFeeVault<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = 8,                            
        seeds = [b"fee_vault"],
        bump
    )]
    pub fee_vault: Account<'info, FeeVault>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BurnSpl<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,

    /// Per-user+mint stats PDA
    #[account(
        mut, 
        seeds=[b"user_stats", user.key().as_ref(), mint.key().as_ref()],
        bump = user_stats_spl.bump,
        )]
    pub user_stats_spl: Account<'info, UserStats>,

    // fee
    #[account(
        mut,
        seeds = [b"fee_vault"],
        bump
    )]
    pub fee_vault: Account<'info, FeeVault>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BurnSol<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    /// CHECK: 
    #[account(mut, address = INCINERATOR)]
    pub burn_account: UncheckedAccount<'info>,

    /// WSOL
    #[account(mut)]
    pub user_wsol_ata: Option<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [b"user_stats", user.key().as_ref(), b"sol"],
        bump = user_stats_sol.bump
    )]
    pub user_stats_sol: Account<'info, UserStats>,

    // fee
    #[account(
        mut,
        seeds = [b"fee_vault"],
        bump
    )]
    pub fee_vault: Account<'info, FeeVault>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawFee<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [b"fee_vault"],
        bump
    )]
    pub fee_vault: Account<'info, FeeVault>,

    /// CHECK:
    #[account(mut)]
    pub to: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}