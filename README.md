# Burn Program (burn-contract)

Anchor-based Solana program that lets users provably burn either SPL tokens or native SOL while logging immutable metadata and charging a configurable protocol fee. Every burn updates a per-user statistics PDA so client apps can render lifetime totals without off-chain aggregation. Fees accumulate in a dedicated vault that a multisig-like admin set can withdraw from.

## Features at a Glance
- SPL & SOL burns: burn SPL tokens via burn_checked CPI or native SOL by sending to the canonical incinerator (1nc1nerator...).
- Per-user stats: PDA seeds ["user_stats", user, mint] (SPL) or ["user_stats", user, "sol"] (SOL) capture counters and total burned amounts.
- On-chain logging: each burn emits BurnEvent { user, mint, amount, message, timestamp } for indexers/analytics.
- Protocol fee: flat FEE_LAMPORTS = 4_000_000 (0.004 SOL) on every burn routed to the fee_vault PDA.
- Guarded withdrawals: only admins in ADMIN_KEYS can drain the vault via withdraw_fee.

Program ID: burnKNMLEUJ7ENqn3ASwSAKaxdwz7bXq9cDVhM72iDa

## Instruction Reference

### initialize_user_stats_spl
- Seeds: ["user_stats", user, mint]
- Accounts: user (signer, payer), SPL mint, user token account (interface), token program, system program.
- Purpose: lazily create the PDA the first time a user burns a particular mint.

### initialize_user_stats_sol
- Seeds: ["user_stats", user, "sol"]
- Accounts: user signer, system program.
- Purpose: create SOL stats PDA before calling burn_sol.

### initialize_fee_vault
- Seeds: ["fee_vault"]
- Accounts: payer signer, system program.
- Purpose: create the zero-data FeeVault PDA.

### burn_spl(amount, message)
- Accounts: user signer, token account, mint, user_stats_spl, fee_vault, token program, system program.
- Flow: validates mint immutability + ownership, collects the fee, burns via burn_checked, updates stats, and emits BurnEvent.
- message must be ≤ 256 bytes and will appear in logs (UTF-8 recommended).

### burn_sol(lamports, message)
- Accounts: user signer, optional WSOL ATA (closed if provided), incinerator account (1nc1nerator...), user_stats_sol, fee_vault, token program, system program.
- Flow: optionally unwraps WSOL, enforces lamports > FEE_LAMPORTS, transfers fee to vault, transfers the rest to the incinerator, updates stats, emits event.

### withdraw_fee(lamports)
- Accounts: admin signer, fee_vault, recipient system account, system program, rent sysvar.
- Only admins in ADMIN_KEYS can call; ensures vault stays rent-exempt.

## Interacting Off-Chain
- Use the provided IDL (idl/burnsolana.json or .ts) with Anchor clients or Metaplex Umi, Shank, etc.
- For SPL burns supply the correct token program (SPL Token or Token-2022) that matches the mint.
- Always create the stats PDA(s) and fee_vault before the first burn on a cluster to avoid AccountNotInitialized errors.

## Security Notes
- All arithmetic uses checked math and Anchor constraints, but you should still run audits before mainnet deployment.
- withdraw_fee moves lamports via raw lamport manipulations; keep the admins’ keypairs offline and consider gating the instruction behind a multisig.
- Client UIs should surface the flat fee and ensure messages are user-generated to avoid spoofed logs.
