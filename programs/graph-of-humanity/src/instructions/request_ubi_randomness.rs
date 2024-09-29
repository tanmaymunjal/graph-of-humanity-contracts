use crate::constants::UBI_USERS_PER_ACC;
use crate::error::GraphOfHumanityErrors;
use crate::state::{DistributionEpoch, Treasury, UBIRandomnessAccount};
use anchor_lang::prelude::*;
use orao_solana_vrf::program::OraoVrf;
use orao_solana_vrf::state::NetworkState;
use orao_solana_vrf::CONFIG_ACCOUNT_SEED;
use orao_solana_vrf::RANDOMNESS_ACCOUNT_SEED;

#[derive(Accounts)]
#[instruction(force: [u8; 32])]
pub struct RequestRandomnessUBI<'info> {
    #[account(mut)]
    pub cranker: Signer<'info>,
    #[account(
        seeds = [
            b"treasury"
        ],
        bump=treasury.bump,
        constraint = treasury.distribution_active == true @GraphOfHumanityErrors::NoEpochStarted
    )]
    pub treasury: Account<'info, Treasury>,
    #[account(
        mut,
        seeds = [
            treasury.distributions.to_string().as_bytes(),
            b"di_epoch",
        ],
        bump=epoch.bump
    )]
    pub epoch: Account<'info, DistributionEpoch>,
    #[account(
        init_if_needed,
        payer = cranker,
        space = 8+UBIRandomnessAccount::INIT_SPACE,
        seeds = [
            epoch.key().as_ref(),
            &force,
            b"ubi_randomness_acc"
        ],
        bump,
        constraint = ubi_randomness_acc.accounts.len() <UBI_USERS_PER_ACC as usize @GraphOfHumanityErrors::RandomnessFull
    )]
    pub ubi_randomness_acc: Account<'info, UBIRandomnessAccount>,
    /// CHECK: The account's data is validated within the ORAO VRF program
    #[account(
        mut,
        seeds = [RANDOMNESS_ACCOUNT_SEED, &force],
        bump,
        seeds::program = orao_solana_vrf::ID
    )]
    pub randomness_account: AccountInfo<'info>,
    /// CHECK: This is the ORAO VRF program
    pub vrf_program: Program<'info, OraoVrf>,
    #[account(
        mut,
        seeds = [CONFIG_ACCOUNT_SEED],
        bump,
        seeds::program = orao_solana_vrf::ID
    )]
    pub vrf_config: Account<'info, NetworkState>,
    /// CHECK: This is the treasury account for the ORAO VRF program
    #[account(mut)]
    pub vrf_treasury: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<RequestRandomnessUBI>, force: [u8; 32]) -> Result<()> {
    let randomness_account = &ctx.accounts.randomness_account;
    let epoch = &ctx.accounts.epoch;
    let ubi_randomness_acc = &mut ctx.accounts.ubi_randomness_acc;
    // Zero seed is illegal in VRF
    require!(
        force != [0_u8; 32],
        GraphOfHumanityErrors::NullSeedInvalidForVrf
    );

    // Request randomness from ORAO VRF
    let cpi_program = ctx.accounts.vrf_program.to_account_info();
    let cpi_accounts = orao_solana_vrf::cpi::accounts::RequestV2 {
        payer: ctx.accounts.cranker.to_account_info(),
        network_state: ctx.accounts.vrf_config.to_account_info(),
        treasury: ctx.accounts.vrf_treasury.to_account_info(),
        request: randomness_account.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    orao_solana_vrf::cpi::request_v2(cpi_ctx, force)?;

    ubi_randomness_acc.bump = ctx.bumps.ubi_randomness_acc;
    ubi_randomness_acc.epoch = epoch.key();
    ubi_randomness_acc.randomness_account = randomness_account.key();
    ubi_randomness_acc.accounts = vec![];

    Ok(())
}
