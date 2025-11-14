pub mod instructions;
pub mod errors;
pub mod states;
pub mod context;
use crate::context::*;

use anchor_lang::prelude::*;

declare_id!("burnKNMLEUJ7ENqn3ASwSAKaxdwz7bXq9cDVhM72iDa");

#[cfg(not(feature = "no-entrypoint"))]
use { default_env::default_env, solana_security_txt::security_txt };

#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    name: "Burnsolana",
    project_url: "https://burnsolana.com",
    contacts: "email:burnsolana@gmail.com,twitter:https://x.com/burnanalos,discord:https://discord.gg/zDgT74N2Y",
    policy: "https://github.com/RicRaimon/burnsolana/blob/main/SECURITY.md",

    source_code: "https://github.com/RicRaimon/burnsolana",
    source_revision: default_env!("GITHUB_SHA", ""),
    source_release: default_env!("GITHUB_REF_NAME", ""),
    auditors: "None",                  
    acknowledgements: "None"
}

/// 4 functions in the program:
/// 1 — PDA initialization:
///   1.1 — initialization for SPL
///   1.2 — initialization for SOL
///   1.3 — initialization of the fee vault
/// 2 — burn an SPL token
/// 3 — burn SOL
/// 4 — withdraw the collected fees

#[program]
pub mod burnsolana {

    use super::*;
    /// 1.1 PDA (SPL) — Initialize UserStats on the first burn of any token.
    /// For each user, there can be exactly as many PDAs
    /// as the number of different tokens they have burned. On subsequent burns
    /// of the same token, we simply update the existing account.
    pub fn initialize_user_stats_spl(ctx: Context<InitializeUserStatsSpl>) -> Result<()> {
        instructions::initialize_user_stats_spl(ctx)
    }
    /// 1.2 PDA SOL
    pub fn initialize_user_stats_sol(ctx: Context<InitializeUserStatsSol>) -> Result<()> {
        instructions::initialize_user_stats_sol(ctx)
    }
    /// 1.3 PDA feeVault
    pub fn initialize_fee_vault(_ctx: Context<InitializeFeeVault>) -> Result<()> {
        Ok(())
    }

    /// 2 - burn an SPL token and update UserStats
    pub fn burn_spl(
        ctx: Context<BurnSpl>,
        amount: u64,    
        message: String,
    ) -> Result<()> {
       instructions::burn_spl(ctx, amount, message)
    }

    /// 3 - burn SOL by transferring to the system incinerator address:
    ///     INCINERATOR = 1nc1nerator11111111111111111111111111111111
    pub fn burn_sol(ctx: Context<BurnSol>, lamports: u64, message: String) -> Result<()> {
        instructions::burn_sol(ctx, lamports, message)
    }

    /// 4 - withdraw fees
    pub fn withdraw_fee(ctx: Context<WithdrawFee>, lamports: u64) -> Result<()> {
        instructions::withdraw_fee(ctx, lamports)
    }
}

