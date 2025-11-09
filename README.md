# Burn Program (burn-contract)

Anchor-based Solana program that lets users provably burn either SPL tokens or native SOL while logging immutable metadata and charging a protocol fee. Every burn updates a per-user statistics PDA so client apps can render lifetime totals without off-chain aggregation.

## Features at a Glance
- SPL & SOL burns: burn SPL tokens via burn_checked CPI or native SOL by sending to the canonical incinerator (1nc1nerator...).
- Per-user stats: PDA seeds ["user_stats", user, mint] (SPL) or ["user_stats", user, "sol"] (SOL) capture counters and total burned amounts.
- On-chain logging: each burn emits BurnEvent { user, mint, amount, message, timestamp } for indexers/analytics.
- Protocol fee: flat FEE_LAMPORTS = 4_000_000 (0.004 SOL) on every burn routed to the fee_vault PDA.

Program ID: burnKNMLEUJ7ENqn3ASwSAKaxdwz7bXq9cDVhM72iDa

## Instruction Reference

### initialize_user_stats_spl
- Purpose: lazily create the PDA the first time a user burns a particular mint.

### initialize_user_stats_sol
- Purpose: create SOL stats PDA before calling burn_sol.

### initialize_fee_vault
- Purpose: create the zero-data FeeVault PDA.

### burn_spl(amount, message)
- Flow: validates mint immutability + ownership, burns via burn_checked, updates stats, and emits BurnEvent.
- message must be ≤ 256 bytes and will appear in logs (UTF-8 recommended).

### burn_sol(lamports, message)
- Flow: optionally unwraps WSOL, enforces lamports > FEE_LAMPORTS, transfers the rest to the incinerator, updates stats, emits event.

### withdraw_fee(lamports).
- Only admins in ADMIN_KEYS can call;

## Interacting
- Use the provided IDL (idl/burnsolana.json or .ts) with Anchor clients or Metaplex Umi, Shank, etc.
- For SPL burns supply the correct token program (SPL Token or Token-2022) that matches the mint.
- Always create the stats PDA(s) and fee_vault before the first burn on a cluster to avoid AccountNotInitialized errors.
